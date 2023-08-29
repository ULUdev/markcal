use clap::Parser;
pub mod args;
pub mod csv;
use csv::*;

fn calculate_mean(elms: &[Mark], subject: &str) -> f32 {
    let mut unique_classes: Vec<String> = elms
        .iter()
        .filter(|x| x.get_subject() == subject)
        .map(|x| x.get_class().to_owned())
        .collect();
    unique_classes.dedup();
    let num_classes_reciprocal: f32 = 1.0 / unique_classes.len() as f32;

    unique_classes
        .into_iter()
        .map(|x| {
            let iter = elms
                .iter()
                .filter(|y| y.get_subject() == subject && y.get_class() == &x);
            iter.clone().map(Mark::get_value).sum::<i32>() as f32 / iter.count() as f32
        })
        .sum::<f32>()
        * num_classes_reciprocal
}

fn produce_report(elms: &[Mark]) -> Vec<String> {
    let mut unique_keys = elms.to_owned();
    unique_keys.dedup_by(|x, y| x.get_subject() == y.get_subject());
    unique_keys
        .into_iter()
        .map(|x| {
            let elms_count = elms
                .iter()
                .filter(|y| y.get_subject() == x.get_subject())
                .count();
            let mean = elms
                .iter()
                .filter(|y| y.get_subject() == x.get_subject())
                .map(|y| y.get_value())
                .sum::<i32>() as f32
                / elms_count as f32;
            format!("{}: {:.2}", x.get_subject(), mean)
        })
        .collect()
}

fn main() {
    let params = args::Params::parse();
    let inf: String = params
        .file
        .unwrap_or("~/.markctl".to_string())
        .replace('~', &std::env::var("HOME").expect("couldn't read $HOME"));
    // file might not exist
    let mut elms = match std::path::Path::new(&inf).exists() {
        true => csv::parse_csv_dat(&inf).expect("failed to parse file"),
        false => Vec::new(),
    };
    match params.command {
        args::Command::Insert {
            sub,
            mark,
            class,
            comment,
        } => {
            elms.push(Mark::new(&sub, mark, &class, comment));
            std::fs::write(inf, csv::to_csv(&elms)).expect("failed to write to file");
        }
        args::Command::List { sort, sub } => {
            let mut out: Vec<Mark> = elms
                .into_iter()
                .filter(|x| x.get_subject() == sub)
                .collect();
            match sort {
                Some(args::Sort::Ascending) => out.sort(),
                Some(args::Sort::Descending) => {
                    out.sort();
                    out.reverse();
                }
                _ => (),
            }
            let out_strings: Vec<String> = out.iter().map(Mark::to_string).collect();
            println!("{}", out_strings.join("\n"));
        }
        args::Command::Avg { sub } => {
            println!("Avg on {}: {:.2}", sub, calculate_mean(&elms, &sub));
        }
        args::Command::Report => {
            println!("{}", produce_report(&elms).join("\n"));
        }
    }
}
