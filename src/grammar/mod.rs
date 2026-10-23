//! # Grammar
//! The trained model is a Probabilitstic Context-Free *Grammar*

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

mod display;
mod io;

// ----------------------------------------------------------------------------------------------------------
// Used by Variable and trainer::classify

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Class {
    Alpha,
    Years,
    Digits,
    Special,
}

// ----------------------------------------------------------------------------------------------------------
// Used by trainer, guesser and grammar

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Variable(pub Class, pub usize);

/// Each element is grouped by occurrences, so K = Variable or BaseStructure
pub type Groups<K, W> = Vec<(W, Vec<K>)>;
pub type Rules<W> = HashMap<Variable, Groups<String, W>>; // TODO: Productions?
pub type BaseStructure = Vec<Variable>;

pub type Count = u32;
pub type Cost = u64;

/// Contains the data for the model, grouped by count or by cost
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Grammar<W> {
    rules: HashMap<Variable, Groups<String, W>>, // Rules<W>,
    bases: Groups<BaseStructure, W>,
}

impl<W> Grammar<W> {
    pub fn new(rules: Rules<W>, bases: Groups<BaseStructure, W>) -> Self {
        Grammar { rules, bases }
    }

    pub fn rules(&self) -> &Rules<W> {
        &self.rules
    }
    pub fn bases(&self) -> &Groups<BaseStructure, W> {
        &self.bases
    }

    pub fn into_parts(self) -> (Rules<W>, Groups<BaseStructure, W>) {
        (self.rules, self.bases)
    }
}
