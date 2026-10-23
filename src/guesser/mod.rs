use std::{cmp::Reverse, collections::BinaryHeap};

mod helpers;

use crate::{grammar::*, guesser::helpers::*};

/// 
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Node {
    cost: u64,
    base: usize,
    groups: Vec<usize>, // the subscripts: group index per slot
}

/// 
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
        let bases = helpers::flatten(bases);

        let queue = helpers::populate_queue(&bases, &rules);

        Guesses { bases, rules, queue, current: None, size: 0, offset: 0 }
    }

}

impl Grammar<Count> {
    pub fn into_guesses(self) -> Guesses {
        Guesses::new(self.into())
    }
}

///--------------
///   ITERATOR
///--------------

impl Iterator for Guesses {
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

impl Guesses {
    // fn chosen_group(&self, base: usize, groups: &[usize]) -> impl Iterator<Item = &(u64, Vec<String>)> {
    // } 

    /// 
    fn node_cost(&self, base: usize, groups: &[usize]) -> u64 {
        if let Some(ref n) = self.current {
            // n.cost, n.base, &n.groups
            // n.cost + 
        }

        0
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

    fn is_adopter(&self, popped: &Node, child: &Node) -> bool {
        todo!()
    }
    
    fn advance_node(&mut self) -> Option<Node> {
        todo!()
    }
}

// TODO: Relax the strict probability decrease constraint; see https://github.com/lakiw/pcfg_cracker/blob/master/docs/source/pcfg_guesser.rst 
//       and https://github.com/lakiw/pcfg_cracker/blob/master/docs/build/latex/pcfgdevelopersguide.pdf (chapter 8)
// TODO: Switch between versions with CLI flag
// mod v2; for the adoption algorithm, v1 for pivot