use std::{io::{self, Write}, path::Path};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match (args.get(1).map(String::as_str), args.get(2)) {
        (Some("train"), Some(data)) => {
            let file = io::BufReader::new(std::fs::File::open(data)?);
            let model = pcfg::trainer::train(file)?;
            model.save(Path::new("model"))
        }
        (Some("run"), Some(path)) => {
            let model = pcfg::grammar::model::Model::load(Path::new(path))?;
            let guesser = pcfg::guesser::Guesser::from(model);
            match print_all(guesser) {
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
    }
    out.flush()
}