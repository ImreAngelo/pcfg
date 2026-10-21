use std::io::{self, BufRead, BufWriter, ErrorKind, Write};

mod grammar;
mod guesser;
mod trainer;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match (args.get(1).map(String::as_str), args.get(2)) {
        (Some("train"), Some(data)) => {
            let file = io::BufReader::new(std::fs::File::open(data)?);
            let model = trainer::train(file)?;
            
            // let model = trainer::train(file.lines().map_while(Result::ok));
            println!("Processed {} variable groups and {} base structure groups", model.terminals().iter().count(), model.base_structures().iter().count());

            for t in model.terminals().iter() {
                println!(" ----- {:?}: ", t.0);
                for (count, terminals) in t.1.iter() {
                    println!("{count}: {:?} ({})", terminals.iter().take(10).collect::<Vec<_>>(), terminals.len());
                }
                println!("");
            }

            // match print_all(model.guesses()) {
            //     Err(e) if e.kind() == ErrorKind::BrokenPipe => Ok(()),
            //     other => other,
            // }
            
            // model.save(Path::new("model"))
            Ok(())
        }
        (Some("run"), Some(path)) => {
            todo!()
            // let model = model::Model::load(Path::new(path))?;
            // match print_all(model.guesses()) {
            //     Err(e) if e.kind() == ErrorKind::BrokenPipe => Ok(()),
            //     other => other,
            // }
        }
        _ => {
            eprintln!("usage: myprogram train <data> | run <model>");
            std::process::exit(2);
        }
    }
}

// fn print_all(guesses: impl Iterator<Item = String>) -> io::Result<()> {
//     let mut out = BufWriter::new(io::stdout().lock());
//     for g in guesses {
//         writeln!(out, "{g}")?;
//     }
//     out.flush()
// }