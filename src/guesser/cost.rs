use crate::grammar::*;

/// TODO: Load as CLI flag with default value
const COST_SCALE: f64 = 1_000_000.0;

/// Lower cost is more probable. TODO: Handle f64::INFINITY
pub fn cost(count: Count, total: Cost) -> Cost {
    (-(count as f64 / total as f64).ln() * COST_SCALE) as Cost
}

/// Convert a grouping by counts to a grouping by costs
fn to_costs<K>(groups: Groups<K, Count>) -> Groups<K, Cost> {
    let total = groups
        .iter()
        .map(|(c, ks)| (*c as u64) * (ks.len() as u64))
        .sum();
    groups
        .into_iter()
        .map(|(c, ks)| (cost(c, total), ks))
        .collect()
}

/// Convert from stored model (Grammar<Count>) to live model (Grammar<Cost>)
impl From<Grammar<Count>> for Grammar<Cost> {
    fn from(g: Grammar<Count>) -> Self {
        let (rules, bases) = g.into_parts();
        Grammar::new(
            rules.into_iter().map(|(v, gs)| (v, to_costs(gs))).collect(),
            to_costs(bases),
        )
    }
}
