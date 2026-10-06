use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader}};

use logos::Logos;

use crate::grammar::{Classification, Token, Variable};

pub type Terminals = HashMap<Variable, HashMap<String, u32>>;

/// CLASSIFY
fn classify(line: &str) -> Vec<Token> {
    let mut lex = Classification::lexer(line);
    let mut structures = Vec::new();

    while let Some(token) = lex.next() {
        match token {
            Ok(classification) => {
                let slice = lex.slice().into();
                let length = lex.slice().chars().count();
                let var = Variable(classification, length);
                structures.push(Token (var, slice));
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

    // Final guesses
    let mut terminals = Terminals::new();

    while reader.read_line(&mut buffer)? > 0 {
        classify(buffer.trim_end()).iter().for_each(|token| {
            *terminals
                .entry(token.0).or_default()
                .entry(token.1.clone()).or_insert(0) += 1;
        });

        buffer.clear();
    }

    // TODO: Do something
    dbg!(terminals);

    Ok(())
}