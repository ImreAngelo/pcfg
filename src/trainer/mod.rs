use std::io::{self, BufRead};

use hashbrown::HashMap;

use crate::grammar::*;

mod counts;
mod classify;
mod process;

/// 
pub fn train(mut reader: impl BufRead) -> io::Result<Grammar<Count>> {
    let mut data = process::TrainingData {
        terminals: HashMap::new(), 
        counts: HashMap::new()
    };

    let mut base_buffer = Vec::new();
    let mut str_buffer = String::new();

    while reader.read_line(&mut str_buffer)? > 0 {
        process::line(str_buffer.trim_end(), &mut data, &mut base_buffer);
        str_buffer.clear();
    }
    
    Ok(data.into())
}
