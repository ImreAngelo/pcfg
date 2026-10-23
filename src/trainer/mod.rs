use std::io::{self, BufRead};

use crate::grammar::*;

mod classify;
mod count;

/// Trains a PCFG on the input file
pub fn train(mut reader: impl BufRead) -> io::Result<Grammar<Count>> {
    let mut data = count::TrainingData::default();
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        data.add_line(line.trim_end());
        line.clear();
    }

    Ok(data.into())
}
