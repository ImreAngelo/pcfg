use logos::Logos;

use crate::grammar::{Classification, Variable};

/// ...
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
                variable: Variable(t, range.len()),
                slice: &line[range],
            }),
            _ => {
                // This should never happen
                eprintln!("Unrecognized token at position {}", range.start);
                None
            }
        })
}
