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
            let file = open_input(data)?;
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
        (Some("-v"), _) => {
            println!("Version 0.1.5");
            std::process::exit(0);
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

fn open_input(path: &str) -> io::Result<Box<dyn io::BufRead>> {
    let path = Path::new(&path);
    let file = File::open(path)?;

    #[cfg(feature = "progress")]
    let raw: Box<dyn io::Read> = {
        let len = file.metadata()?.len();
        let pb = indicatif::ProgressBar::new(len);
        pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
            )
            .unwrap(),
        );
        Box::new(pb.wrap_read(file))
    };

    #[cfg(not(feature = "progress"))]
    let raw: Box<dyn Read> = Box::new(file); // TODO: Custom impl Read that does not use vtable indirection

    #[cfg(feature = "gzip")]
    let raw: Box<dyn io::Read> = if path.extension().is_some_and(|e| e == "gz") {
        Box::new(flate2::read::GzDecoder::new(raw))
    } else {
        raw
    };

    Ok(Box::new(io::BufReader::new(raw)))
}
