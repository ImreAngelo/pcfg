use std::{io::{self, BufWriter, ErrorKind, Write}, path::Path};

use crate::guesser::Guesser;

mod grammar;
mod guesser;
mod trainer;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match (args.get(1).map(String::as_str), args.get(2)) {
        (Some("train"), Some(data)) => {
            let file = io::BufReader::new(std::fs::File::open(data)?);
            let model = trainer::train(file)?;
            
            println!("Processed {} variable groups and {} base structure groups", model.terminals().iter().count(), model.base_structures().iter().count());

            // for t in model.base_structures().iter().take(10) {
            //     println!(" ----- Group {:?}: ", t.0);
            //     for vars in t.1.iter() {
            //         println!("{:?}", vars.iter().take(10).collect::<Vec<_>>());
            //     }
            //     println!("");
            // }

            // for t in model.terminals().iter().take(2) {
            //     println!(" ----- {:?}: ", t.0);
            //     for (count, terminals) in t.1.iter() {
            //         println!("{count}: {:?} ({})", terminals.iter().take(10).collect::<Vec<_>>(), terminals.len());
            //     }
            //     println!("");
            // }

            model.save(Path::new("model"))
        }
        (Some("run"), Some(path)) => {
            let model = grammar::model::Model::load(Path::new(path))?;
            let guesser = Guesser::from(model);
            match print_all(guesser) {
                Err(e) if e.kind() == ErrorKind::BrokenPipe => Ok(()),
                other => other,
            }
        }
        _ => {
            eprintln!("usage: pcfg train <data> | run <model>");
            std::process::exit(2);
        }
    }
}

fn print_all(guesses: impl Iterator<Item = String>) -> io::Result<()> {
    let mut out = BufWriter::new(io::stdout().lock());
    for g in guesses {
        writeln!(out, "{g}")?;
    }
    out.flush()
}