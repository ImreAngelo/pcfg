use hashbrown::HashMap;

use crate::grammar::{Groups, Variable, into_groups};

mod classify;
mod process;

pub fn train(path: &str) -> Result<(HashMap<Variable, Groups<String>>, Groups<Vec<Variable>>), std::io::Error> {
    let data = process::process_file(path)?; 

    // Terminals
    let ts = data.0.into_iter()
        .map(|(var, occ)| (var, into_groups(occ)))
        .collect();

    // Base structures
    let bs = into_groups(data.1);

    Ok((ts, bs))
}


// DEBUG: Create groups from training data
/*
fn group_data(data: TrainingData) {
    let x: Vec<(Variable, Groups<String>)> = data.0.into_iter().map(|(var, map)| {
        (var, into_groups(map))
    }).collect();

    let mut sum = 0;

    for g in &x {
        for (l, strings) in g.1.iter() {
            let mut strings = strings.clone();
            strings.truncate(4);

            let label = match g.0 {
                Variable(Classification::Alpha, l) => format!("A{l}"),
                Variable(Classification::Years, l) => format!("Y{l}"),
                Variable(Classification::Digits, l) => format!("D{l}"),
                Variable(_, l) => format!("S{l}"),
            };

            println!("{} with cardinality {}: {:?}", label, l, strings);
            // println!("Group #{}", l);

            sum += 1;
        }
    }

    // Base structures
    let y = into_groups(data.1);

    println!("\nBase structures:");
    for g in &y {
        let display: Vec<String> = g.1.iter().map(|vars| {
            vars.iter().map(|var| match var {
                Variable(Classification::Alpha, l) => format!("A{l}"),
                Variable(Classification::Years, l) => format!("Y{l}"),
                Variable(Classification::Digits, l) => format!("D{l}"),
                Variable(_, l) => format!("S{l}"),
            }).collect::<Vec<String>>().join(" ")
        }).take(4).collect();

        println!("Base {}: {:?}", g.0, display);
    }

    println!("");
    println!("Found {} groups of variables.", sum);
    println!("Found {} groups of base structures", y.iter().count());
}
*/