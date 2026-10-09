use std::fs::File;
use std::io::{BufRead, BufReader};

use hashbrown::HashMap;

use crate::grammar::{BaseStructure, model::Model, Variable};
use crate::trainer::classify::classify;
use crate::trainer::occurrences::Occurrences;

pub use std::io::Error as TrainingError;

type Terminals = HashMap<Variable, Occurrences<String>>;

pub struct TrainingData(pub Terminals, pub Occurrences<BaseStructure>);

/// Count occurrences of variables and base structures
pub fn process_file(path: &str) -> Result<TrainingData, TrainingError> {
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
            // Using entry_ref only allocates a new String
            // if hashmap does not already contain it
            *terminals
                .entry(token.variable)
                .or_default()
                .entry_ref(token.slice)
                .or_insert(0) += 1;

            // Count base structures
            base_buffer.push(token.variable);
        }

        // Again only allocate once per unique base structure
        *base_structures
            .entry_ref(base_buffer.as_slice())
            .or_insert(0) += 1;

        base_buffer.clear();
        buffer.clear();
    }

    Ok(TrainingData(terminals, base_structures))
}

impl Model {
    pub fn from(data: TrainingData) -> Self {
        let bases = super::occurrences::into_groups(data.1);
        let terminals = data
            .0
            .into_iter()
            .map(|(var, occ)| (var, super::occurrences::into_groups(occ)))
            .collect();

        Model::new(terminals, bases)
    }
}
