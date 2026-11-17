use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use argh::FromArgs;

use pcfg::*;

/// Fast PCFG password cracker.
#[derive(FromArgs)]
struct Cli {
    /// print version and exit
    #[argh(switch, short = 'v')]
    version: bool,

    /// subcommand: train or run
    #[argh(subcommand)]
    command: Option<Command>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Command {
    Train(TrainArgs),
    Run(RunArgs),
}

/// Train a model on a (gzipped) text file.
#[derive(FromArgs)]
#[argh(subcommand, name = "train")]
struct TrainArgs {
    /// path to (gzipped) text file
    #[argh(positional)]
    path: String,

    /// name the model
    #[argh(option, short = 'n', default = "String::from(\"model\")")]
    name: String,
}

/// Output password guesses from a trained model.
#[derive(FromArgs)]
#[argh(subcommand, name = "run")]
struct RunArgs {
    /// path to pcfg model file
    #[argh(positional)]
    path: String,
}

// TODO: Do not return io::Result
fn main() -> io::Result<()> {
    let cli: Cli = argh::from_env();

    if cli.version {
        println!(
            "{} version {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        return Ok(());
    }

    match cli.command {
        Some(Command::Train(args)) => {
            let file = open_input(&args.path)?;
            let grammar = train(file)?;
            grammar.save(Path::new(&args.name))
        }
        Some(Command::Run(args)) => {
            let path = Path::new(&args.path);
            let grammar = Grammar::load(path)?;

            // TODO: grammar.into_guesses().for_each_guess(|s| writeln!(out, "{g}"))?;

            match print_all(grammar.into_guesses()) {
                Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
                other => other,
            }
        }
        None => {
            eprintln!("No command given. Try --help.");
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
