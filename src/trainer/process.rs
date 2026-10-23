use hashbrown::HashMap;

use crate::{grammar::*, trainer::{classify::*, counts::*}};


pub(crate) struct TrainingData { 
    pub(crate) terminals: HashMap<Variable, Counts<String>>, 
    pub(crate) counts: Counts<BaseStructure>
}

impl From<TrainingData> for Grammar<Count> {
    fn from(data: TrainingData) -> Grammar<Count> {
        let bases = into_groups(data.counts);
        let terminals = data.terminals
            .into_iter()
            .map(|(var, occ)| (var, into_groups(occ)))
            .collect();

        Grammar::new(terminals, bases)
    }
}

/// 
pub(crate) fn line(line: &str, data: &mut TrainingData, buffer: &mut Vec<Variable>) {
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