use std::collections::BTreeMap;

use hashbrown::HashMap;

use crate::{grammar::*, trainer::classify::*};

pub type Counts<K> = HashMap<K, u32>;

#[derive(Default)]
pub(super) struct TrainingData {
    terminals: HashMap<Variable, Counts<String>>,
    counts: Counts<BaseStructure>,
    buffer: Vec<Variable>,
}

impl From<TrainingData> for Grammar<Count> {
    fn from(data: TrainingData) -> Grammar<Count> {
        let bases = into_groups(data.counts);
        let terminals = data
            .terminals
            .into_iter()
            .map(|(var, occ)| (var, into_groups(occ)))
            .collect();

        Grammar::new(terminals, bases)
    }
}

impl TrainingData {
    /// Processes a single line of text into the model
    pub(super) fn add_line(&mut self, line: &str) {
        for token in classify(&line) {
            *self
                .terminals
                .entry(token.variable)
                .or_default()
                .entry_ref(token.slice)
                .or_insert(0) += 1;

            self.buffer.push(token.variable);
        }

        // Only allocate once per unique base structure with entry_ref
        *self.counts.entry_ref(self.buffer.as_slice()).or_insert(0) += 1;

        self.buffer.clear();
    }
}

/// Inverts a hashmap with (key, count) to a group with (count, vector of keys)
/// NOTE: BTree was faster than unstable_sort'ing a vec for rockyou.txt
fn into_groups<K>(occ: Counts<K>) -> Groups<K, Count> {
    // TODO: vec / hashmap and sort instead gives slower benchmarks for rockyou.txt
    let mut btree: BTreeMap<u32, Vec<K>> = BTreeMap::new();

    for (key, count) in occ.into_iter() {
        btree.entry(count).or_default().push(key);
    }

    btree.into_iter().rev().collect()
}
