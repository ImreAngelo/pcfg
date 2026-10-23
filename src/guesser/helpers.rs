use std::{cmp::Reverse, collections::BinaryHeap};

use hashbrown::HashMap;

use crate::grammar::*;


///-------------
///   HELPERS
///-------------

pub(crate) fn flatten(bases: Vec<(Cost, Vec<BaseStructure>)>) -> Vec<(Cost, BaseStructure)> {
    bases.into_iter()
        .flat_map(|(c, bs)| bs.into_iter().map(move |b| (c, b)))
        .collect()
}

/// Build initial priority queue
pub(crate) fn populate_queue(bases: &Vec<(u64, Vec<Variable>)>, rules: &HashMap<Variable, Vec<(u64, Vec<String>)>>) -> BinaryHeap<Reverse<super::Node>> {
    let mut q = BinaryHeap::new();
    for (c, structure) in bases.iter() {
        let slots = structure.len();
        let cost = c + structure.iter()
            .fold(0, |s, k| s + rules[k][0].0);
        
        q.push(Reverse(super::Node {
            cost,
            base: 0,
            groups: vec![0; slots]
        }));
    }
    q
}

/// Pretty print
pub(crate) fn format_base(base: &Vec<Variable>) -> String {
    base.iter().map(|&v| v.to_string())
        .collect::<Vec<_>>().join(" ")
}

pub(crate) fn print_bases(bases: &Vec<(u64, Vec<Variable>)>) {
    println!();
    print!("bases = ");

    for x in bases.iter().take(10) {
        print!("({}, {}), ", x.0, format_base(&x.1));
    }

    if bases.len() <= 10 { return; }

    print!("... ");
    
    for x in bases.iter().rev().take(1) {
        print!("({}, {})", x.0, format_base(&x.1));
    }

    println!();
}

pub(crate) fn print_terminals(terminals: &HashMap<Variable, Vec<(u64, Vec<String>)>>) {
    println!();
    
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
    
    println!();
}
