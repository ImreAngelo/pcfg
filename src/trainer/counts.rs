use std::collections::BTreeMap;

use hashbrown::HashMap;

use crate::grammar::Groups;

/// V2 - Group by occurrence, not by variable
pub type Counts<K> = HashMap<K, u32>;

/// Invert a hashmap with (key, count) to a group with (count, vector of keys)
/// NOTE: BTree was faster than sorting for rockyou.txt
// pub fn into_groups<K>(counts: Counts<K>) -> Groups<K> {
//     let mut pairs: Vec<(K, u32)> = counts.into_iter().collect();
//     pairs.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count)); // descending

//     let mut groups: Groups<K> = Vec::new();
//     for (key, count) in pairs {
//         match groups.last_mut() {
//             Some((c, keys)) if *c == count => keys.push(key),
//             _ => groups.push((count, vec![key])),
//         }
//     }
//     groups
// }

pub fn into_groups<K>(occ: Counts<K>) -> Groups<K> {
    // TODO: vec / hashmap and sort instead gives slower benchmarks for rockyou.txt
    let mut btree: BTreeMap<u32, Vec<K>> = BTreeMap::new();

    for (key, count) in occ.into_iter() {
        btree.entry(count).or_default().push(key);
    }

    btree.into_iter().rev().collect()
}