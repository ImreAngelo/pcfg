use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader}};

use logos::Logos;

use crate::grammar::{Classification, Token, Variable};

type Occurrences<T> = HashMap<T, u32>;
type BaseStructure = Vec<Variable>;
type Terminals = HashMap<Variable, HashMap<String, u32>>;

/// CLASSIFY
fn classify(line: &str) -> Vec<Token> {
    let mut lex = Classification::lexer(line);
    let mut structures = Vec::new();

    while let Some(token) = lex.next() {
        match token {
            Ok(classification) => {
                let slice = lex.slice().into();
                let length = lex.slice().chars().count();
                let variable = Variable(classification, length);
                structures.push(Token {
                    variable, 
                    slice,
                });
            }
            Err(()) => eprintln!("Error: Unrecognized token at position {}", lex.span().start),
        }
    }

    structures
}

/// TRAIN
pub fn train(path: &str) -> Result<(), io::Error> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut buffer = String::new(); 

    // 
    let mut terminals = Terminals::new();
    // let mut base_structures: HashMap<BaseStructure, u32> = HashMap::new();

    while reader.read_line(&mut buffer)? > 0 {
        let base = classify(buffer.trim_end());
        
        // TODO: Increment base in base_structures 

        base.iter().for_each(|token| {
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
    dbg!(terminals);

    Ok(())
}