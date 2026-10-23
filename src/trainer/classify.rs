use logos::Logos;

use crate::grammar::{Class, Variable};

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

/// TODO: Single source of truth for lexer
impl Into<Class> for Classification {
    fn into(self) -> Class {
        match self {
            Classification::Alpha => Class::Alpha,
            Classification::Years => Class::Years,
            Classification::Digits => Class::Digits,
            Classification::Special => Class::Special,
        }
    }
}

/// Not to be confused with Logos::Token, though functionally the same
pub struct Token<'a> {
    pub variable: Variable,
    pub slice: &'a str,
}

/// Returns an iterator of tokens
pub fn classify(line: &str) -> impl Iterator<Item = Token<'_>> {
    Classification::lexer(line)
        .spanned()
        .filter_map(|(res, range)| match res {
            Ok(t) => Some(Token {
                variable: Variable(t.into(), range.len()),
                slice: &line[range],
            }),
            _ => {
                // This should never happen
                eprintln!("Unrecognized token at position {}", range.start);
                None
            }
        })
}
