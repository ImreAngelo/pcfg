use logos::Logos;

pub mod model;

#[derive(Logos, PartialEq, Eq, Hash, Debug, Clone, Copy)]
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

/// Each element is grouped by occurrences, so K = Variable or BaseStructure
pub type Groups<K> = Vec<(u32, Vec<K>)>;

pub type BaseStructure = Vec<Variable>;
