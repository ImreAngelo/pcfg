use std::{io, path::Path};

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

    // /// Consume the model, handing back its owned pieces. Lets a generator
    // /// take ownership of the dictionary instead of cloning or borrowing it.
    pub fn into_parts(self) -> (Groups<BaseStructure>, HashMap<Variable, Groups<String>>) {
        (self.bases, self.terminals)
    }
    
    // TODO: Serialize
    pub fn load(path: &Path) -> io::Result<Model> { todo!() }
    pub fn save(&self, path: &Path) -> io::Result<()> { todo!() }
}