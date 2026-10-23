use std::{fs, io, path::Path};

use hashbrown::HashMap;
use logos::Logos;
use serde::{Deserialize, Serialize};

// ----------------------------------------------------------------------------------------------------------
// Used by Variable and trainer::classify

#[derive(Serialize, Deserialize, Logos, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Classification {
    #[regex(r"\p{L}+")] // Unicode-aware
    Alpha,
    #[regex(r"(19|20)[0-9][0-9]")]
    Years,
    #[regex(r"[0-9]+")]
    Digits,
    #[regex(r"[^\p{L}0-9]+")]
    Special,
}

// ----------------------------------------------------------------------------------------------------------
// Used by trainer and guesser

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Variable(pub Classification, pub usize);

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cls = match self.0 {
            Classification::Alpha => "A",
            Classification::Years => "Y",
            Classification::Digits => "D",
            Classification::Special => "S",
        };
        
        f.write_fmt(format_args!("{}{}", cls, self.1))
    }
}

// ----------------------------------------------------------------------------------------------------------
// Used by guesser and conversion from Grammar<Count> -> Grammar<Cost>

/// TODO: Load as CLI flag with default value
const COST_SCALE: f64 = 1_000_000.0; 

/// Lower cost is more probable.
/// TODO: Handle f64::INFINITY
pub fn cost(count: Count, total: Cost) -> Cost {
    (-(count as f64 / total as f64).ln() * COST_SCALE) as Cost
}

/// Convert a grouping by counts to a grouping by costs
fn to_costs<K>(groups: Groups<K, Count>) -> Groups<K, Cost> {
    let total = groups.iter().map(|(c, ks)| (*c as u64) * (ks.len() as u64)).sum();
    groups.into_iter().map(|(c, ks)| (cost(c, total), ks)).collect()
}

// ----------------------------------------------------------------------------------------------------------
// Generic Grammar<T>, and the conversion Grammar<Count> <-> Grammar<Cost>

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

    pub fn rules(&self) -> &Rules<W> { &self.rules }
    pub fn bases(&self) -> &Groups<BaseStructure, W> { &self.bases }

    pub fn into_parts(self) -> (Rules<W>, Groups<BaseStructure, W>) {
        (self.rules, self.bases)
    }
}

impl From<Grammar<Count>> for Grammar<Cost> {
    fn from(g: Grammar<Count>) -> Self {
        Grammar {
            rules: g.rules.into_iter().map(|(v, gs)| (v, to_costs(gs))).collect(),
            bases: to_costs(g.bases),
        }
    }
}

// ----------------------------------------------------------------------------------------------------------
// Serialization; only used by Grammar<Count>

/// Only de/serialize model in the more space-efficient mode
impl Grammar<Count> {
    /// Load model from file at path
    pub fn load(path: &Path) -> io::Result<Grammar<Count>> {
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