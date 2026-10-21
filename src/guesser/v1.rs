//! v1 generator: the pivot ("deadbeat dad") algorithm from Weir et al.,
//! *Password Cracking Using Probabilistic Context-Free Grammars* (IEEE S&P 2009).
//!
//! Every guess has probability `P(base) * Π P(terminal_i)`. Enumerating all
//! combinations and sorting is infeasible, so the paper walks the space with a
//! max-priority queue so guesses come out in non-increasing probability order.
//!
//! Each variable's terminals are kept sorted by descending probability. A
//! *pre-terminal* picks one terminal index per slot of a base structure; the
//! root picks index 0 everywhere (the most probable choice). Popping a node
//! emits its guess and pushes children that advance a single slot to its next,
//! less-probable terminal — so a child's probability is always <= its parent's.
//!
//! The `pivot` field is the dedup trick: from a node with pivot `p`, children
//! only advance slots `i >= p`, and the child adopts pivot `i`. That gives every
//! reachable pre-terminal exactly one parent, so nothing is enqueued twice.
//!
//! Unlike the 2009 paper, terminals here are learned for *every* classification
//! (including alpha), so a pre-terminal already fixes a concrete string — there
//! is no separate dictionary-expansion step and one pre-terminal is one guess.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use hashbrown::HashMap;

use crate::grammar::{BaseStructure, Variable, model::Model};

/// One variable's terminals, flattened into a single list ordered by descending
/// probability. `strings[i]` is the i-th most likely terminal, `probs[i]` its
/// probability.
struct Terminals {
    strings: Vec<String>,
    probs: Vec<f64>,
}

/// A candidate guess: a base structure with one chosen terminal per slot.
struct PreTerminal {
    prob: f64,
    /// Index into `Generator::bases`.
    base: usize,
    /// Chosen terminal index for each slot of the base structure.
    indices: Vec<usize>,
    /// Lowest slot a child is allowed to advance (dedup, see module docs).
    pivot: usize,
}

// Order purely by probability so the `BinaryHeap` behaves as a max-heap on it.
// (NaN can't arise from count/total ratios, so treating it as Equal is safe.)
impl PartialEq for PreTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.prob == other.prob
    }
}
impl Eq for PreTerminal {}
impl PartialOrd for PreTerminal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PreTerminal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.prob.partial_cmp(&other.prob).unwrap_or(Ordering::Equal)
    }
}

pub struct Generator {
    /// Base structures with their probabilities (order is irrelevant; the heap
    /// sorts). Indexed by `PreTerminal::base`.
    bases: Vec<(f64, BaseStructure)>,
    /// Flattened, probability-sorted terminals per variable.
    terminals: HashMap<Variable, Terminals>,
    /// Frontier of candidates, most probable first.
    queue: BinaryHeap<PreTerminal>,
}

impl Generator {
    pub fn new(model: Model) -> Self {
        let (terminal_groups, base_groups) = model.into_parts();

        // P(terminal) = count / (total occurrences for that variable).
        let terminals = terminal_groups
            .into_iter()
            .map(|(var, groups)| {
                let total: f64 = groups
                    .iter()
                    .map(|(count, items)| *count as f64 * items.len() as f64)
                    .sum();

                let mut strings = Vec::new();
                let mut probs = Vec::new();
                for (count, items) in groups {
                    let p = count as f64 / total;
                    for s in items {
                        strings.push(s);
                        probs.push(p);
                    }
                }

                (var, Terminals { strings, probs })
            })
            .collect::<HashMap<_, _>>();

        // P(base) = count / (total base-structure occurrences).
        let base_total: f64 = base_groups
            .iter()
            .map(|(count, items)| *count as f64 * items.len() as f64)
            .sum();

        let mut bases = Vec::new();
        for (count, items) in base_groups {
            let p = count as f64 / base_total;
            for structure in items {
                bases.push((p, structure));
            }
        }

        // Seed the frontier with the most probable pre-terminal (index 0 in
        // every slot) of each base structure.
        let mut queue = BinaryHeap::new();
        for (base, (base_prob, structure)) in bases.iter().enumerate() {
            let mut prob = *base_prob;
            let mut usable = true;
            for var in structure {
                match terminals.get(var) {
                    Some(t) if !t.probs.is_empty() => prob *= t.probs[0],
                    // A variable with no learned terminals can't be filled.
                    _ => {
                        usable = false;
                        break;
                    }
                }
            }
            if usable {
                queue.push(PreTerminal {
                    prob,
                    base,
                    indices: vec![0; structure.len()],
                    pivot: 0,
                });
            }
        }

        Generator { bases, terminals, queue }
    }
}

impl Iterator for Generator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop()?;
        let structure = &self.bases[node.base].1;

        // Expand the chosen terminals into the actual guess string.
        let mut guess = String::new();
        for (slot, var) in structure.iter().enumerate() {
            let t = &self.terminals[var];
            guess.push_str(&t.strings[node.indices[slot]]);
        }

        // Push children: advance one slot (at or after the pivot) to its next,
        // less probable terminal. The child inherits that slot as its pivot.
        for i in node.pivot..structure.len() {
            let t = &self.terminals[&structure[i]];
            let cur = node.indices[i];
            if cur + 1 < t.strings.len() {
                let mut indices = node.indices.clone();
                indices[i] = cur + 1;
                let prob = node.prob / t.probs[cur] * t.probs[cur + 1];
                self.queue.push(PreTerminal { prob, base: node.base, indices, pivot: i });
            }
        }

        Some(guess)
    }
}
