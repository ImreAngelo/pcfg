use hashbrown::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::{format, println};

use logos::Logos;

use crate::grammar::{Classification, Groups, Occurrences, Token, Variable, into_groups};
use crate::trainer::TrainingError::IOError;

type Terminals = HashMap<Variable, Occurrences<String>>;
// type BaseStructure = Vec<Variable>;

// CLASSIFY

/// Returns an iterator of tokens
fn classify(line: &str) -> impl Iterator<Item = Token<'_>> {
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

// TRAIN
struct TrainingData(Terminals, Occurrences<Vec<Variable>>);

/// Count occurrences of variables and base structures
fn process_file(path: &str) -> Result<TrainingData, io::Error> {
    let file = File::open(path)?;
    
    let mut reader = BufReader::new(file);
    
    // Count variables
    let mut terminals = Terminals::new();
    let mut buffer = String::new();
    
    // Count base structures
    let mut base_structures = Occurrences::new();
    let mut base_buffer = Vec::new();

    while reader.read_line(&mut buffer)? > 0 {
        for token in classify(buffer.trim_end()) {
            // Using entry_ref only allocates a new String if hashmap does not already contain it
            *terminals
                .entry(token.variable)
                .or_default()
                .entry_ref(token.slice)
                .or_insert(0) += 1;

            // Count base structures
            base_buffer.push(token.variable);
        }

        // Again only allocate once per unique base structure
        *base_structures.entry_ref(base_buffer.as_slice()).or_insert(0) += 1;

        base_buffer.clear();
        buffer.clear();
    }

    Ok(TrainingData(terminals, base_structures))
}

/// Create groups from training data (Vec<Groups<String>>, Vec<Groups<Vec<Variable>>>)
fn group_data(data: TrainingData) -> () {
    let x: Vec<(Variable, Groups<String>)> = data.0.into_iter().map(|(var, map)| {
        (var, into_groups(map))
    }).collect();

    let mut sum = 0;

    for g in &x {
        for (l, strings) in g.1.iter() {
            let mut strings = strings.clone();
            strings.truncate(4);

            let label = match g.0 {
                Variable(Classification::Alpha, l) => format!("A{l}"),
                Variable(Classification::Years, l) => format!("Y{l}"),
                Variable(Classification::Digits, l) => format!("D{l}"),
                Variable(_, l) => format!("S{l}"),
            };

            println!("{} with cardinality {}: {:?}", label, l, strings);
            // println!("Group #{}", l);

            sum += 1;
        }
    }

    // Base structures
    let y = into_groups(data.1);

    println!("\nBase structures:");
    for g in &y {
        let display: Vec<String> = g.1.iter().map(|vars| {
            vars.iter().map(|var| match var {
                Variable(Classification::Alpha, l) => format!("A{l}"),
                Variable(Classification::Years, l) => format!("Y{l}"),
                Variable(Classification::Digits, l) => format!("D{l}"),
                Variable(_, l) => format!("S{l}"),
            }).collect::<Vec<String>>().join(" ")
        }).take(4).collect();

        println!("Base {}: {:?}", g.0, display);
    }

    println!("");
    println!("Found {} groups of variables.", sum);
    println!("Found {} groups of base structures", y.iter().count());

    // (x, y)
}

fn group_terminals(terminals: Terminals) {
    // let group = terminals.into_iter().map(|x| into_groups(x.1))...
    // terminals
    //     .into_iter()
    //     .map(|(var, occ)| (var, into_groups(occ)))
    for (_, occ) in terminals.into_iter() {
        let x = into_groups(occ);
        println!("{}", x.iter().count());
    }
}

#[derive(Debug)]
pub enum TrainingError {
    IOError(io::Error),
}

pub fn train(path: &str) -> Result<(), TrainingError> {
    let data = process_file(path).map_err(IOError)?;
    let groups = group_data(data);
    Ok(())
}
