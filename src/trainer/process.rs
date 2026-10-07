use std::io::{BufRead, BufReader};
use std::fs::File;

use hashbrown::HashMap;

use crate::grammar::{Occurrences, Variable};

pub use std::io::Error as TrainingError;

type Terminals = HashMap<Variable, Occurrences<String>>;
type BaseStructure = Vec<Variable>;

// pub struct TrainingData(pub Terminals, pub Occurrences<BaseStructure>);

/// Count occurrences of variables and base structures
pub fn process_file(path: &str) -> Result<(Terminals, Occurrences<BaseStructure>), TrainingError> {
    let file = File::open(path)?;
    
    let mut reader = BufReader::new(file);
    
    // Count variables
    let mut terminals = Terminals::new();
    let mut buffer = String::new();
    
    // Count base structures
    let mut base_structures = Occurrences::new();
    let mut base_buffer = Vec::new();

    while reader.read_line(&mut buffer)? > 0 {
        for token in super::classify::classify(buffer.trim_end()) {
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

    Ok((terminals, base_structures))
}