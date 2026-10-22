use hashbrown::HashMap;

use crate::grammar::Variable;


///-------------
///   HELPERS
///-------------

// TODO: abstractions for long composite types
type MyType<N, T> = Vec<(N, Vec<T>)>;

/// Flatten and convert frequency to probability-score/cost
pub(crate) fn convert_bases(base_structures: Vec<(u32, Vec<Vec<Variable>>)>) -> MyType<u64, Variable> {
    let mut bases = Vec::new();
    for (count, b) in base_structures {
        let total = (count as u64) * (b.len() as u64);
        let cost = super::cost(count, total);
        for base in b {
            bases.push((cost, base));
        }
    }
    bases
}

/// Convert frequencies to probability-score/cost
pub(crate) fn convert_terminals(terminals: HashMap<Variable, Vec<(u32, Vec<String>)>>) -> HashMap<Variable, MyType<u64, String>> {
    let mut out = HashMap::<Variable, Vec<(u64, Vec<String>)>>::new();
    for (var, group) in terminals.into_iter() {
        let total = group.iter().fold(0, |s, x| s + x.0 as u64) * (group.len() as u64);
        let mut ts = Vec::<(u64, Vec<String>)>::new();

        // iterate group
        for (count, val) in group.into_iter() {
            let cost = super::cost(count, total);
            ts.push((cost, val));
        }

        out.entry_ref(&var).or_default().extend(ts);
    }

    out
}

/// Pretty print
pub(crate) fn format_base(base: &Vec<Variable>) -> String {
    base.iter().map(|&v| v.to_string())
        .collect::<Vec<_>>().join(" ")
}

pub(crate) fn print_bases(bases: &Vec<(u64, Vec<Variable>)>) {
    println!("");
    print!("bases = ");

    for x in bases.iter().take(10) {
        print!("({}, {}), ", x.0, format_base(&x.1));
    }

    if bases.len() <= 10 { return; }

    print!("... ");
    
    for x in bases.iter().rev().take(2) {
        print!("({}, {}), ", x.0, format_base(&x.1));
    }

    println!("");
}

pub(crate) fn print_terminals(terminals: &HashMap<Variable, Vec<(u64, Vec<String>)>>) {
    println!("");
    // println!("TERMINALS:");
    
    for (k, v) in terminals.iter().take(8) {
        print!("{}:", k);
        let dots = String::from("...");

        for (cost, strs) in v.iter().take(3) {
            let cutoff = 4;
            let mut display = strs.iter().take(cutoff).collect::<Vec<_>>();
            if strs.len() > cutoff { display.push(&dots); };

            println!("\t({cost}, {:?})", display);
        }
    }
    
    println!("");
}
