use std::io::{self, BufRead};

use hashbrown::HashMap;

use crate::grammar::*;

mod count;
mod classify;

/// 
pub fn train(mut reader: impl BufRead) -> io::Result<Grammar<Count>> {
    let mut data = count::TrainingData {
        terminals: HashMap::new(), 
        counts: HashMap::new()
    };

    let mut base_buffer = Vec::new();
    let mut str_buffer = String::new();

    while reader.read_line(&mut str_buffer)? > 0 {
        count::line(str_buffer.trim_end(), &mut data, &mut base_buffer);
        str_buffer.clear();
    }
    
    Ok(data.into())
}
