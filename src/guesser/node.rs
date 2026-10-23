/// 
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct Node {
    cost: u64,
    base: usize,
    groups: Vec<usize>, // the subscripts: group index per slot
}


impl Node {
    pub(crate) fn new(cost: u64, base: usize, groups: Vec<usize>) -> Self {
        Node { cost, base, groups }
    }
}