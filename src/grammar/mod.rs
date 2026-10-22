use logos::Logos;
use serde::{Deserialize, Serialize};

pub mod model;

#[derive(Serialize, Deserialize, Logos, PartialEq, Eq, Hash, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Variable(pub Classification, pub usize);

/// Each element is grouped by occurrences, so K = Variable or BaseStructure
pub type Groups<K> = Vec<(u32, Vec<K>)>;

pub type BaseStructure = Vec<Variable>;


impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cls = match self.0 {
            Classification::Alpha => "A",
            Classification::Years => "Y",
            Classification::Digits => "D",
            Classification::Special => "S",
        };
        
        f.write_fmt(format_args!("{}{}", cls, self.1))
    }
}