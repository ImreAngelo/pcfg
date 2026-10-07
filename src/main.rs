use std::env;

mod grammar;
// mod guesser;
mod trainer;

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

            trainer::train(path).expect("Training failed");
            // println!("Processed {} unique variables", terminals.iter().count());
        },
        "guess" => {
            println!("Run command is not yet implemented.");
        },
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
