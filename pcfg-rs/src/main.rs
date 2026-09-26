use std::env;

mod structures;
mod training;

use training::train;

// Usage: pcfg train <file> <ruleset>
// Later: pcfg run [ruleset]
fn main() {
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

            train(path).expect("Training failed");
        },
        "run" => {
            println!("Run command is not yet implemented.");
        },
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
