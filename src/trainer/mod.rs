use std::{collections::HashMap, eprintln, fs::File};
use std::io::{self, BufRead, BufReader};

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
        let base = classify(buffer.trim_end());
        
        // TODO: Increment base in base_structures 

        base.for_each(|token| {
            let map = terminals.entry(token.variable).or_default();

            // Only allocate new String if hashmap does not contain one
            if let Some(count) = map.get_mut(token.slice) {
                *count += 1;
            } else {
                map.insert(token.slice.to_owned(), 1);
            }
        });

        buffer.clear();
    }

    // TODO: Do something
    // dbg!(terminals);

    Ok(terminals)
}