use logos::Logos;

use crate::grammar::{Class, Variable};

#[derive(Logos)]
enum Classification {
    #[regex(r"\p{L}+",              |_| Class::Alpha)]
    #[regex(r"(19|20)[0-9][0-9]",   |_| Class::Years)]
    #[regex(r"[0-9]+",              |_| Class::Digits)]
    #[regex(r"[^\p{L}0-9]+",        |_| Class::Special)]
    Class(Class),
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
            Ok(Classification::Class(class)) => {
                let slice = &line[range];
                let variable = Variable(class, slice.chars().count());
                Some(Token { variable, slice })
            }
            _ => {
                // This should never happen
                eprintln!("Unrecognized token at position {}", range.start);
                None
            }
        })
}
