use std::io::{self, BufRead};

use crate::grammar::*;

mod classify;
mod count;

/// Trains a PCFG on the input file
pub fn train(mut reader: impl BufRead) -> io::Result<Grammar<Count>> {
    let mut data = count::TrainingData::default();
    let mut buf = Vec::new();

    // Invalid sequences become U+FFFD, which falls into the `Special` class.
    while reader.read_until(b'\n', &mut buf)? > 0 {
        let line = String::from_utf8_lossy(&buf);
        data.add_line(line.trim_end());
        buf.clear();
    }

    Ok(data.into())
}
