use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::structures::{Structure, classify};

pub type Terminals = HashMap<(Structure, usize), HashMap<String, u32>>;

/// Trains the model using the data from the specified file
pub fn train(path: &str) -> io::Result<()> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {}. Error: {}", path, e),
            ));
        }
    };

    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    
    let mut terminals: Terminals = HashMap::new();

    while reader.read_line(&mut buf)? > 0 {
        for seg in classify(buf.trim_end()) {
            *terminals
                .entry((seg.structure, seg.length))
                .or_default()
                .entry(seg.slice)
                .or_insert(0) += 1;
        }
        buf.clear();
    }

    dbg!(terminals);

    Ok(())
}