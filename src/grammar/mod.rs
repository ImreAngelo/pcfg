use std::collections::BTreeMap;

use hashbrown::HashMap;
use logos::Logos;

#[derive(Logos, PartialEq, Eq, Hash, Debug, Clone, Copy)] // Clone, Copy
pub enum Classification {
    #[regex(r"\p{L}+")] // Unicode-aware
    Alpha,
    #[regex(r"(19|20)[0-9][0-9]")]
    Years,
    #[regex(r"[0-9]+")]
    Digits,
    #[regex(r"[^\p{L}0-9]+")]
    Special,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Variable(pub Classification, pub usize);

/// V2 - Group by occurrence, not by variable
pub type Occurrences<K> = HashMap<K, u32>;
/// Each element is grouped by occurrences, so K = Variable or BaseStructure
pub type Groups<K> = Vec<(u32, Vec<K>)>;

/// Invert a hashmap with (key, count) to a group with (count, vector of keys)
pub fn into_groups<K>(occ: Occurrences<K>) -> Groups<K> {
    // TODO: Use vec / hashmap and sort instead
    let mut btree: BTreeMap<u32, Vec<K>> = BTreeMap::new();

    for (key, count) in occ.into_iter() {
        btree.entry(count).or_default().push(key);
    }

    btree.into_iter().rev().collect()
}
