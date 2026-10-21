use std::io::{self, BufRead};

use hashbrown::HashMap;

use crate::{
    grammar::{BaseStructure, Variable, model::Model}, 
    trainer::{classify::classify, occurrences::Occurrences}
};

pub mod occurrences;
mod classify;

pub(crate) struct TrainingData { 
    terminals: HashMap<Variable, Occurrences<String>>, 
    occurrences: Occurrences<BaseStructure>
}

/// 
pub fn process_line(line: &str, data: &mut TrainingData, buffer: &mut Vec<Variable>) {
    for token in classify(&line) {
        *data.terminals
            .entry(token.variable).or_default()
            .entry_ref(token.slice)
            .or_insert(0) += 1;

        buffer.push(token.variable);
    }
    
    // Only allocate once per unique base structure with entry_ref
    *data.occurrences
        .entry_ref(buffer.as_slice())
        .or_insert(0) += 1;

    buffer.clear();
}

/// 
pub fn train(mut reader: impl BufRead) -> io::Result<Model> {
    let mut data = TrainingData {
        terminals: HashMap::new(), 
        occurrences: HashMap::new()
    };

    let mut base_buffer = Vec::new();
    let mut str_buffer = String::new();

    while reader.read_line(&mut str_buffer)? > 0 {
        process_line(str_buffer.trim_end(), &mut data, &mut base_buffer);
        str_buffer.clear();
    }
    
    Ok(Model::from(data))
}

/// Returns a Model containing ...
#[deprecated = "Use train instead"]
pub fn train_alloc(lines: impl Iterator<Item = String>) -> Model {
    let mut data = TrainingData {
        terminals: HashMap::new(), 
        occurrences: HashMap::new()
    };

    let mut base_buffer = Vec::new();

    for line in lines {
        process_line(&line, &mut data, &mut base_buffer);
    }

    Model::from(data)
}

impl Model {
    pub fn from(data: TrainingData) -> Self {
        let bases = occurrences::into_groups(data.occurrences);
        let terminals = data.terminals
            .into_iter()
            .map(|(var, occ)| (var, occurrences::into_groups(occ)))
            .collect();

        Model::new(terminals, bases)
    }
}