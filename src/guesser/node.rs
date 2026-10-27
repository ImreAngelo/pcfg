use crate::grammar::Cost;

/// State information used by iterator
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(super) struct Node {
    pub(super) cost: Cost,
    pub(super) base: usize,
    pub(super) groups: Vec<usize>,
}

impl Node {
    pub(super) fn new(cost: u64, base: usize, groups: Vec<usize>) -> Self {
        Node { cost, base, groups }
    }
}
