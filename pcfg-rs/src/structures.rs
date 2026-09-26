use std::eprintln;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Structure {
    #[regex(r"\p{L}+")]        // letters (Unicode-aware)
    Letters,
    #[regex(r"[0-9]+")]        // digits
    Digits,
    #[regex(r"[^\p{L}0-9]+")]  // anything else
    Symbols,
}

pub struct Segment {
    pub structure: Structure,
    pub slice: String,
    pub length: usize,
}

/// 
pub fn classify(line: &str) -> Vec<Segment> {
    let mut lex = Structure::lexer(line);
    let mut structures = Vec::new();

    while let Some(token) = lex.next() {
        match token {
            Ok(structure) => { 
                let slice = lex.slice().into();
                let length = lex.slice().chars().count();
                structures.push(Segment { structure, slice, length }); 
            },
            Err(()) => eprintln!("Error: Unrecognized token at position {}", lex.span().start),
        }
    }

    structures
}