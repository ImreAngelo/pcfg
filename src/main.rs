use std::env;
use std::io::{BufWriter, Write, stdout};

mod grammar;
mod guesser;
mod trainer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Assume command is "train" for now
    let command = &args[1];
    
    match command.as_str() {
        "train" => {
            let path = &args[2];

            let _name = if args.len() > 3 {
                &args[3]
            } else {
                path.split("/").last().unwrap_or("default_ruleset")
            };

            let model = trainer::train(path).expect("Training failed");
            println!("Processed {} variable groups and {} base structure groups", model.terminals().iter().count(), model.base_structures().iter().count());

            // TODO: Serialize, in the meantime:
            let mut out = BufWriter::new(stdout().lock());
            for guess in guesser::generate(model) {
                writeln!(out, "{guess}")?; // TODO: Gracefully exit on Ctrl + C exit
            }
        },
        "guess" => {
            println!("TODO: Deserialize ruleset.");
        },
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }

    Ok(())
}
