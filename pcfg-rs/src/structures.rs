use std::eprintln;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Structure {
    #[regex(r"\p{L}+")]        // runs of letters (Unicode-aware)
    Letters,
    #[regex(r"[0-9]+")]        // runs of digits
    Digits,
    #[regex(r"[^\p{L}0-9]+")]  // anything else = symbols
    Symbols,
}

/// 
pub fn classify(line: &str) -> Vec<Structure> {
    let mut lex = Structure::lexer(line);
    let mut structures = Vec::new();

    while let Some(token) = lex.next() {
        match token {
            Ok(structure) => { structures.push(structure); },
            Err(()) => eprintln!("Error: Unrecognized token at position {}", lex.span().start),
        }
    }

    structures
}