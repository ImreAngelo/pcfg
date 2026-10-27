use std::cmp::Reverse;

use crate::{
    grammar::Cost,
    guesser::{Guesses, node::Node},
};

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

// TODO: Optimize everything
impl Guesses {
    // TODO: Optimizations
    pub fn for_each_guess(&self) -> ! {
        // print directly to buffer
        todo!()
    }

    // TODO: Abstract common functions into shared functions
    // fn chosen_group(&self, base: usize, groups: &[usize]) -> impl Iterator<Item = &(u64, Vec<String>)> {
    //     &self.bases[base].1.iter().zip(groups)
    //         .map(|(v, &i)| &self.rules[v][i])
    //         // .map(|x| x)
    // }

    /// Calculate what the cost of a (new) Node should be
    fn node_cost(&self, base: usize, groups: &[usize]) -> Cost {
        let (cost, vars) = &self.bases[base];
        let c: Cost = vars
            .iter()
            .zip(groups)
            .map(|(v, &i)| self.rules[v][i].0)
            .sum();
        cost + c
    }

    /// Calculate what the Guesses.size of the current Node should be
    fn node_size(&self, node: &Node) -> usize {
        let (_, base) = &self.bases[node.base];
        base.iter()
            .zip(&node.groups)
            .map(|(v, &i)| self.rules[v][i].1.len())
            .fold(1, |v, len| v.saturating_mul(len))
    }

    /// Return the next element in the cartesian product of current group (build single output)
    fn next_cartesian(&self) -> String {
        // TODO: String builder: only update the pre-terminal(s) that changes
        let node = self.current.as_ref().unwrap();

        let chosen = self.bases[node.base]
            .1
            .iter()
            .zip(&node.groups)
            .map(|(v, &i)| &self.rules[v][i].1)
            .collect::<Vec<_>>();

        // Convert offset to slot positions
        let mut k = self.offset;
        let mut slots = Vec::with_capacity(node.groups.len());

        for group in chosen.iter().rev() {
            let len = group.len();
            slots.push(k % len);
            k /= len;
        }

        // Build string
        chosen
            .into_iter()
            .zip(slots.into_iter().rev())
            .map(|(t, i)| t[i].as_str())
            .collect::<String>()
    }

    /// For each child: compute all of its parents (you are one of them). If the largest one is you,
    /// push the child. Otherwise do nothing, because a later parent will take care of it.
    fn is_adopter(&self, popped: &Node, child: &Node) -> bool {
        (0..child.groups.len())
            .filter(|&j| child.groups[j] > 0)
            .map(|j| {
                let mut groups = child.groups.clone();
                groups[j] -= 1;
                Node::new(self.node_cost(child.base, &groups), child.base, groups)
            })
            .all(|parent| parent <= *popped)
    }

    /// Get all children for a parent node
    fn get_children(&self, parent: &Node) -> Vec<Node> {
        let base = parent.base;
        let mut out = Vec::new();

        for (i, v) in self.bases[base].1.iter().enumerate() {
            let rule = &self.rules[v];
            let gi = parent.groups[i];

            // check groups[i] + 1 is a valid group index
            if rule.len() <= gi + 1 {
                continue;
            }

            let mut groups = parent.groups.clone();
            groups[i] += 1;

            let cost = self.node_cost(base, &groups);

            out.push(Node::new(cost, base, groups));
        }

        out
    }

    /// Pop a new node, add its adopted children to the queue
    fn advance_node(&mut self) -> Option<()> {
        let popped = self.queue.pop().map(|r| r.0)?;

        for child in self.get_children(&popped) {
            if self.is_adopter(&popped, &child) {
                self.queue.push(Reverse(child));
            }
        }

        self.size = self.node_size(&popped);
        self.current = Some(popped);
        self.offset = 0;

        Some(())
    }
}
