use std::{fs, io, path::Path};

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::grammar::*;

/// Contains the training data compacted
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Model {
    terminals: HashMap<Variable, Groups<String>>,
    bases: Groups<BaseStructure>,
}

impl Model {
    pub fn new(terminals: HashMap<Variable, Groups<String>>, bases: Groups<BaseStructure>) -> Self {
        Model { terminals, bases }
    }

    /// Consume the model, handing back its owned pieces. Lets a generator
    /// take ownership of the dictionary instead of cloning or borrowing it.
    pub fn into_parts(self) -> (Groups<BaseStructure>, HashMap<Variable, Groups<String>>) {
        (self.bases, self.terminals)
    }
    
    /// Load model from file at path
    pub fn load(path: &Path) -> io::Result<Model> {
        postcard::from_bytes(&fs::read(path)?).map_err(io::Error::other)
    }

    /// Save model to file at path
    pub fn save(&self, path: &Path) -> io::Result<()> {
        let inner = fs::File::create(path)?;
        let writer = io::BufWriter::new(inner);
        postcard::to_io(self, writer).map_err(io::Error::other)?;
        Ok(())
    }
}