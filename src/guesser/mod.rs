use std::collections::BinaryHeap;

use crate::grammar::model::Model;

const FREQUENCY_INTEGER_SCALE: usize = 1_000_000; 

// 
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    cost: u64,          // lower = more probable
    base: usize,
    groups: Vec<usize>, // the subscripts: group index per slot
}

// 
pub struct Guesser {
    model: Model,
    queue: BinaryHeap<Node>,
    current: Option<Node>,
    size: usize,
    offset: usize,
}

impl From<Model> for Guesser {
    fn from(model: Model) -> Self {
        Guesser { 
            model, 
            queue: BinaryHeap::new(), // TODO: add first node to queue
            current: None,
            size: 0,
            offset: 0
        }
    }
}

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
                let guess = self.build_guess();   // decode (current, offset)
                self.offset += 1;
                return Some(guess);
            }
            n -= remaining;                        // skip the rest of this node entirely
            self.advance_node()?;                  // pop next node, push its children, offset = 0, recompute size
        }
    }
}

impl Guesser {
    fn build_guess(&self) -> String {
        todo!() 
    }

    fn advance_node(&self) -> Option<Node> {
        todo!()
    }
}

// TODO: Relax the strict probability decrease constraint; see https://github.com/lakiw/pcfg_cracker/blob/master/docs/source/pcfg_guesser.rst 
//       and https://github.com/lakiw/pcfg_cracker/blob/master/docs/build/latex/pcfgdevelopersguide.pdf (chapter 8)
// TODO: Switch between versions with CLI flag
// mod v2; for the adoption algorithm, v1 for pivot