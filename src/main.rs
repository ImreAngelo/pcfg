use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use pcfg::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match (args.get(1).map(String::as_str), args.get(2)) {
        (Some("train"), Some(data)) => {
            let file = io::BufReader::new(File::open(data)?);
            let grammar = train(file)?;
            grammar.save(Path::new("model"))
        }
        (Some("run"), Some(path)) => {
            let path = Path::new(path);
            let grammar = Grammar::load(path)?;

            // TODO: grammar.into_guesses().for_each_guess(|s| writeln!(out, "{g}"))?;

            match print_all(grammar.into_guesses()) {
                Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
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
    let mut out = io::BufWriter::new(io::stdout().lock());
    for g in guesses {
        writeln!(out, "{g}")?;
        // out.flush()?; // For debugging
    }
    out.flush()
}
