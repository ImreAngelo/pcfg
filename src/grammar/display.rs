/// -- DISPLAY TRAITS
/// Not used in release mode!
/// TODO: Convert to impl fmt::Display for ...
use crate::grammar::*;

#[allow(dead_code)]
pub(crate) fn format_base(base: &BaseStructure) -> String {
    base.iter()
        .map(|&v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[allow(dead_code)]
pub(crate) fn print_bases(bases: &Vec<(Cost, BaseStructure)>) {
    println!();
    print!("bases = ");

    for x in bases.iter().take(10) {
        print!("({}, {}), ", x.0, format_base(&x.1));
    }

    if bases.len() <= 10 {
        return;
    }

    print!("... ");

    for x in bases.iter().rev().take(1) {
        print!("({}, {})", x.0, format_base(&x.1));
    }

    println!();
}

#[allow(dead_code)]
pub(crate) fn print_rules(rules: &HashMap<Variable, Vec<(Cost, Vec<String>)>>) {
    println!();

    for (k, v) in rules.iter().take(8) {
        print!("{}:", k);
        let dots = String::from("...");

        for (cost, strs) in v.iter().take(3) {
            let cutoff = 4;
            let mut display = strs.iter().take(cutoff).collect::<Vec<_>>();
            if strs.len() > cutoff {
                display.push(&dots);
            };

            println!("\t({cost}, {:?})", display);
        }
    }

    println!();
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cls = match self.0 {
            Class::Alpha => "A",
            Class::Years => "Y",
            Class::Digits => "D",
            Class::Special => "S",
        };

        f.write_fmt(format_args!("{}{}", cls, self.1))
    }
}
