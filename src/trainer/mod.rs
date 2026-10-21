use std::io::{self, BufRead};

use hashbrown::HashMap;

use crate::{
    grammar::{BaseStructure, Variable, model::Model}, 
    trainer::{classify::classify, counts::Counts}
};

mod counts;
mod classify;

pub(crate) struct TrainingData { 
    terminals: HashMap<Variable, Counts<String>>, 
    counts: Counts<BaseStructure>
}

impl From<TrainingData> for Model {
    fn from(data: TrainingData) -> Model {
        let bases = counts::into_groups(data.counts);
        let terminals = data.terminals
            .into_iter()
            .map(|(var, occ)| (var, counts::into_groups(occ)))
            .collect();

        Model::new(terminals, bases)
    }
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
    *data.counts
        .entry_ref(buffer.as_slice())
        .or_insert(0) += 1;

    buffer.clear();
}

/// 
pub fn train(mut reader: impl BufRead) -> io::Result<Model> {
    let mut data = TrainingData {
        terminals: HashMap::new(), 
        counts: HashMap::new()
    };

    let mut base_buffer = Vec::new();
    let mut str_buffer = String::new();

    while reader.read_line(&mut str_buffer)? > 0 {
        process_line(str_buffer.trim_end(), &mut data, &mut base_buffer);
        str_buffer.clear();
    }
    
    Ok(data.into())
}
