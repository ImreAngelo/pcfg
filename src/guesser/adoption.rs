use crate::guesser::{Guesses, node::Node};

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

            // skip the rest of this node entirely
            n -= remaining;
            self.advance_node()?; // pop next node, push its children, offset = 0, recompute size
        }
    }
}

impl Guesses {
    // fn chosen_group(&self, base: usize, groups: &[usize]) -> impl Iterator<Item = &(u64, Vec<String>)> {
    // }

    /// ???
    fn node_cost(&self, base: usize, groups: &[usize]) -> u64 {
        if let Some(ref n) = self.current {
            // n.cost, n.base, &n.groups
            // n.cost +
        }

        0
    }

    /// ???
    fn node_size(&self, node: &Node) -> usize {
        todo!()
    }

    /// Return the next element in the cartesian product of current group and increments offset
    fn next_cartesian(&self) -> String {
        todo!()
    }

    /// ???
    fn is_adopter(&self, popped: &Node, child: &Node) -> bool {
        todo!()
    }

    /// ???
    fn advance_node(&mut self) -> Option<Node> {
        todo!()
    }
}
