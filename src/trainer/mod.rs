use std::{eprintln, fs::File};
use std::io::{self, BufRead, BufReader};
use hashbrown::HashMap;

use logos::Logos;

use crate::grammar::{Classification, Token, Variable};

type Occurrences<T> = HashMap<T, u32>;
type Terminals = HashMap<Variable, Occurrences<String>>;
// type BaseStructure = Vec<Variable>;

/// CLASSIFY
fn classify(line: &str) -> impl Iterator<Item = Token<'_>> {
    Classification::lexer(line).spanned().filter_map(|(res, range)| match res {
        Ok(t) => Some(Token {
            variable: Variable(t, range.len()),
            slice: &line[range]
        }),
        _ => { // This should never happen
            eprintln!("Unrecognized token at position {}", range.start);
            None
        }
    })
}

/// TRAIN
pub fn train(path: &str) -> Result<Terminals, io::Error> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut buffer = String::new(); 

    // 
    let mut terminals = Terminals::new();
    // let mut base_structures: Occurrences<BaseStructure> = HashMap::new();

    while reader.read_line(&mut buffer)? > 0 {
        // TODO: Increment base in base_structures 

        for token in classify(buffer.trim_end()) {
            // Using entry_ref only allocates a new String if hashmap does not already contain it
            *terminals
                .entry(token.variable).or_default()
                .entry_ref(token.slice).or_insert(0) += 1;
        }

        buffer.clear();
    }

    // TODO: Do something
    // dbg!(terminals);

    Ok(terminals)
}