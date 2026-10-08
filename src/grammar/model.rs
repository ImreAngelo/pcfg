use hashbrown::HashMap;

use crate::grammar::*;

/// Contains the fully parsed and processed model data
pub struct Model {
    terminals: HashMap<Variable, Groups<String>>,
    bases: Groups<BaseStructure>,
}

impl Model {
    pub fn new(terminals: HashMap<Variable, Groups<String>>, bases: Groups<BaseStructure>) -> Self {
        Model { terminals, bases }
    }

    pub fn terminals(&self) -> &HashMap<Variable, Groups<String>> {
        &self.terminals
    }

    pub fn base_structures(&self) -> &Groups<BaseStructure> {
        &self.bases
    }
}
