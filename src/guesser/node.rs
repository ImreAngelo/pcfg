/// State information used by iterator
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct Node {
    cost: u64,
    base: usize,
    groups: Vec<usize>,
}

impl Node {
    pub(crate) fn new(cost: u64, base: usize, groups: Vec<usize>) -> Self {
        Node { cost, base, groups }
    }
}
