use std::io::{self, BufRead, BufReader};
use std::fs::File;
use hashbrown::HashMap;

use logos::Logos;

use crate::grammar::{Classification, Groups, Occurrences, Token, Variable, into_groups};
use crate::trainer::TrainingError::IOError;

type Terminals = HashMap<Variable, Occurrences<String>>;
// type BaseStructure = Vec<Variable>;

// CLASSIFY

/// Returns an iterator of tokens
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

// TRAIN

/// Count occurrences of variables and base structures
fn process_file(path: &str) -> Result<Terminals, io::Error> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut buffer = String::new(); 

    // 
    let mut terminals = Terminals::new();
    // let mut base_structures: Occurrences<BaseStructure> = HashMap::new();

    while reader.read_line(&mut buffer)? > 0 {
        // TODO: Count base structures 

        for token in classify(buffer.trim_end()) {
            // Using entry_ref only allocates a new String if hashmap does not already contain it
            *terminals
                .entry(token.variable).or_default()
                .entry_ref(token.slice).or_insert(0) += 1;
        }

        buffer.clear();
    }

    Ok(terminals)
}

/// Group 
fn group_terminals(terminals: Terminals) -> Result<(), TrainingError> {
    let group = terminals.into_iter().map(|x| (x.1 as Occurrences<String>).into_groups());

    Ok(())
}

enum TrainingError {
    IOError(io::Error),
}

pub fn train(path: &str) -> Result<(), TrainingError> {
    let terminals = process_file(path).map_err(IOError);

    Ok(())
}