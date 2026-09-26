use std::env;
// use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::structures::classify;

mod structures;

// Usage: pcfg train <file> <ruleset>
// Later: pcfg run [ruleset]
fn main() {
    let args: Vec<String> = env::args().collect();

    // Assume command is "train" for now
    let command = &args[1];
    let path = &args[2];

    match command.as_str() {
        "train" => {
            train(path).expect("Training failed");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

/// 
fn process_line(line: &str) {
    println!("Processing line: {}", line);
    let structures = classify(&line);
    println!("Classified structures: {:?}", structures);
}

/// Trains the model using the data from the specified file
fn train(path: &str) -> io::Result<()> {
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

    while reader.read_line(&mut buf)? > 0 {
        process_line(&buf.trim_end());
        buf.clear();
    }

    Ok(())
}