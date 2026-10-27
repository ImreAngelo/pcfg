use std::{cmp::Reverse, collections::BinaryHeap};

mod adoption;
mod cost;
mod node;

use crate::{grammar::*, guesser::node::*};

/// Iterator for making guesses
pub struct Guesses {
    bases: Vec<(Cost, BaseStructure)>, // Groups<Variable, Cost>
    rules: Rules<Cost>,
    queue: BinaryHeap<Reverse<Node>>,
    current: Option<Node>,
    size: usize,
    offset: usize,
}

impl Guesses {
    fn new(grammar: Grammar<Cost>) -> Self {
        let (rules, bases) = grammar.into_parts();
        let bases = bases
            .into_iter()
            .flat_map(|(c, bs)| bs.into_iter().map(move |b| (c, b)))
            .collect::<Vec<_>>();

        let queue = populate_queue(&bases, &rules);

        Guesses {
            bases,
            rules,
            queue,
            current: None,
            size: 0,
            offset: 0,
        }
    }
}

impl Grammar<Count> {
    pub fn into_guesses(self) -> Guesses {
        Guesses::new(self.into())
    }
}

///--------------
///   HELPERS
///--------------

/// Build initial priority queue
fn populate_queue(
    bases: &[(Cost, BaseStructure)],
    rules: &Rules<Cost>,
) -> BinaryHeap<Reverse<Node>> {
    let mut q = BinaryHeap::new();
    for (i, (c, structure)) in bases.iter().enumerate() {
        let slots = structure.len();
        let cost = c + structure.iter().fold(0, |s, k| s + rules[k][0].0);

        let node = Node::new(cost, i, vec![0; slots]);

        q.push(Reverse(node));
    }
    q
}

// TODO: Relax the strict probability decrease constraint; see https://github.com/lakiw/pcfg_cracker/blob/master/docs/source/pcfg_guesser.rst
//       and https://github.com/lakiw/pcfg_cracker/blob/master/docs/build/latex/pcfgdevelopersguide.pdf (chapter 8)
// TODO: Switch between versions with CLI flag
// mod v2; for the adoption algorithm, v1 for pivot
