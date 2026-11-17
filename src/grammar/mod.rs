//! # Grammar
//! The trained model is a Probabilitstic Context-Free *Grammar*

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

mod display;
mod io;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Class {
    Alpha,
    Years,
    Digits,
    Special,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Variable(pub Class, pub usize);

/// Each element is grouped by occurrences, so K = Variable or BaseStructure
pub type Groups<K, W> = Vec<(W, Vec<K>)>;
pub type Rules<W> = HashMap<Variable, Groups<String, W>>; // TODO: Productions?
pub type BaseStructure = Vec<Variable>;

pub type Count = u32;
pub type Cost = u64;

/// Contains the data for the model, grouped by count or by cost
/// NOTE: Clone is only derived for test/bench builds (feature = "testing")
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "testing", derive(Clone))]
pub struct Grammar<W> {
    rules: Rules<W>, // HashMap<Variable, Groups<String, W>>
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
