use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

// `BufRead::fill_buf` is only used to sniff the gzip header.
#[cfg(feature = "gzip")]
use std::io::BufRead;

use argh::FromArgs;

use pcfg::*;

/// Fast PCFG password cracker.
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help"))]
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
#[argh(subcommand, name = "train", help_triggers("-h", "--help"))]
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
#[argh(subcommand, name = "run", help_triggers("-h", "--help"))]
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
    let raw: Box<dyn io::Read> = Box::new(file); // TODO: Custom impl Read that does not use vtable indirection

    // Buffer first so the header can be sniffed without consuming it
    #[cfg_attr(not(feature = "gzip"), allow(unused_mut))]
    let mut buffered = io::BufReader::new(raw);

    // Detect gzip by magic bytes (1f 8b) in header
    #[cfg(feature = "gzip")]
    if buffered.fill_buf()?.starts_with(&[0x1f, 0x8b]) {
        let decoder = flate2::read::GzDecoder::new(buffered);
        return Ok(Box::new(io::BufReader::new(decoder)));
    }

    Ok(Box::new(buffered))
}
