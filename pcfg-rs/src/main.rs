use std::env;
use std::path::Path;

mod storage;
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

            let name = if args.len() > 3 {
                &args[3]
            } else {
                path.split("/").last().unwrap_or("default_ruleset")
            };

            let terminals = train(path).expect("Training failed");

            let dir = Path::new("rulesets").join(name);
            storage::save(&terminals, &dir).expect("Saving failed");
            println!(
                "Saved {} buckets to {}",
                terminals.len(),
                dir.display()
            );
        },
        "run" => {
            println!("Run command is not yet implemented.");
        },
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
