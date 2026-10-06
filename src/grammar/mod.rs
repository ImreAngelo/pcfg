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