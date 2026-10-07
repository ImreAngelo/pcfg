use std::collections::BTreeMap;

use hashbrown::HashMap;
use logos::Logos;

#[derive(Logos, PartialEq, Eq, Hash, Debug, Clone, Copy)] // Clone, Copy
pub enum Classification {
    #[regex(r"\p{L}+")] // Unicode-aware
    Alpha,
    #[regex(r"[1-9][0-9][0-9][0-9]")]
    Years,
    #[regex(r"[0-9]+")]
    Digits,
    #[regex(r"[^\p{L}0-9]+")]
    Special,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Variable(pub Classification, pub usize);

pub struct Token<'a>{
    pub variable: Variable, 
    pub slice: &'a str
}


/// V2 - Group by occurrence, not by variable
pub type Occurrences<K> = HashMap<K, u32>;
/// Each element is grouped by occurrences, so K = Variable or BaseStructure
pub type Groups<K> = Vec<(u32, Vec<K>)>;

pub fn into_groups<K>(occ: Occurrences<K>) -> Groups<K> {
    let closure = |mut btree: BTreeMap<u32, Vec<K>>, (value, count)| {
        btree.entry(count).or_default().push(value);
        btree 
    };

    occ.into_iter().fold(BTreeMap::new(), closure).into_iter().rev().collect()
}

// trait Group<K> {
//     fn into_groups(&self) -> Groups<K>;
// }

// impl<K> Group<K> for Count<K> {
//     fn into_groups(&mut self) -> Groups<K> {
//         self.into_iter().map(|k, count| .entry(count))
//     }
// }