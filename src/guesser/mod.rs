use std::{cmp::Reverse, collections::BinaryHeap};

use hashbrown::HashMap;

mod helpers;

use crate::grammar::{BaseStructure, Variable, model::Model};

/// TODO: Load as CLI flag with default value
const COST_SCALE: f64 = 1_000_000.0; 

/// Lower cost is more probable.
/// TODO: Handle f64::INFINITY
fn cost(count: u32, total: u64) -> u64 {
    (-(count as f64 / total as f64).ln() * COST_SCALE) as u64
}

/// 
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Node {
    cost: u64,
    base: usize,
    groups: Vec<usize>, // the subscripts: group index per slot
}

/// 
pub struct Guesser {
    bases: Vec<(u64, BaseStructure)>,
    terminals: HashMap<Variable, Vec<(u64, Vec<String>)>>,
    queue: BinaryHeap<Reverse<Node>>,
    current: Option<Node>,
    size: usize,
    offset: usize,
}

///----------------
///   CONVERSION
///----------------

impl From<Model> for Guesser {
    fn from(model: Model) -> Self {
        let (base_structures, terminals) = model.into_parts();
        
        // TODO: Optimize model data structure if possible
        let bases = helpers::convert_bases(base_structures);
        let terminals = helpers::convert_terminals(terminals);
        
        helpers::print_bases(&bases);
        helpers::print_terminals(&terminals);

        // Process root (S) node -> Add all children (most likely base structures)
        let queue = helpers::populate_queue(&bases, &terminals);

        // TODO: Standardized unit testing
        println!("Queue starts with {:?}", queue.peek().unwrap());
        assert_eq!(queue.len(), bases.len());

        Guesser { 
            bases,
            terminals, 
            queue,
            current: None,
            size: 0,
            offset: 0
        }
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