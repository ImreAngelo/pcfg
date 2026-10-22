use std::collections::BinaryHeap;

use hashbrown::HashMap;

use crate::grammar::{BaseStructure, Variable, model::Model};

const COST_SCALE: f64 = 1_000_000.0; 

/// Lower cost is more probable
fn cost(count: u32, total: u64) -> u64 {
    (-(count as f64 / total as f64).ln() * COST_SCALE) as u64
}

/// 
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    cost: u64,
    base: usize,
    groups: Vec<usize>, // the subscripts: group index per slot
}

/// 
pub struct Guesser {
    bases: Vec<(u64, BaseStructure)>,
    terminals: HashMap<Variable, Vec<(u64, Vec<String>)>>,
    queue: BinaryHeap<Node>,
    current: Option<Node>,
    size: usize,
    offset: usize,
}




///-------------
///   HELPERS
///-------------

/// Flatten and convert frequency to probability-score
fn convert_bases(base_structures: Vec<(u32, Vec<Vec<Variable>>)>) -> Vec<(u64, Vec<Variable>)> {
    let mut bases = Vec::new();
    for (count, b) in base_structures {
        let total = (count as u64) * (b.len() as u64);
        let cost = cost(count, total);
        for base in b {
            bases.push((cost, base));
        }
    }
    bases
}


///----------------
///   CONVERSION
///----------------

impl From<Model> for Guesser {
    fn from(model: Model) -> Self {
        let (bases, terminals) = model.into_parts();

        let bases = convert_bases(bases);

        // Variables
        let mut terminals = HashMap::new(); // TODO: Keep groupings; have index
        for (var, group) in model.terminals().into_iter() {
            let total = group.iter().fold(0, |s, x| s + x.0 as u64) * (group.len() as u64);

            // iterate group
            for (count, val) in group {
                let cost = cost(*count, total);
                terminals
            }

            terminals.entry_ref(var).or_default()
                .or_insert(0) += ;
        }

        // First node
        let mut queue = BinaryHeap::new();
        queue.push(Node {
            cost: 1000,
            base: 0,
            groups: vec![0; bases[0].1.len()]
        });

        todo!()
        // Guesser { 
        //     bases,
        //     terminals, 
        //     queue,
        //     current: None,
        //     size: 0,
        //     offset: 0
        // }
    }
}




///--------------------
///   IMPLEMENTATION
///--------------------

impl Iterator for Guesser {
    type Item = String;

    /// PCFG next v2 (adoption algorithm)
    fn next(&mut self) -> Option<Self::Item> {
        self.nth(0)
    }

    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        loop {
            let remaining = self.size - self.offset;
            if n < remaining {
                self.offset += n;
                let guess = self.next_cartesian();
                self.offset += 1;
                return Some(guess);
            }
            n -= remaining;                        // skip the rest of this node entirely
            self.advance_node()?;                  // pop next node, push its children, offset = 0, recompute size
        }
    }
}

impl Guesser {
    ///
    fn node_cost(&self, base: usize, groups: &[usize]) -> u64 {
        todo!()
    }

    ///
    fn node_size(&self, node: &Node) -> usize {
        todo!()
    }
    
    /// Return the next element in the cartesian product of current group.
    /// Increments offset.
    fn next_cartesian(&self) -> String {
        todo!() 
    }

    fn advance_node(&mut self) -> Option<Node> {
        todo!()
    }

    fn is_adopter(&self, popped: &Node, child: &Node) -> bool {
        todo!()
    }
}

// TODO: Relax the strict probability decrease constraint; see https://github.com/lakiw/pcfg_cracker/blob/master/docs/source/pcfg_guesser.rst 
//       and https://github.com/lakiw/pcfg_cracker/blob/master/docs/build/latex/pcfgdevelopersguide.pdf (chapter 8)
// TODO: Switch between versions with CLI flag
// mod v2; for the adoption algorithm, v1 for pivot