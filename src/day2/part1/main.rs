use structopt::StructOpt;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

static AOC_PROBLEM: &'static str = "2.1";

#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn contains_n_of_any_letter(word: String, n: i32) -> bool {
    word.chars()
        .fold(HashMap::new(), |mut acc, chr| {
            let entry = acc.entry(chr).or_insert(0);
            *entry += 1;
            acc
        })
        .values()
        .any(|f| *f == n)
}

fn solve(reader: BufReader<File>) -> Option<i32> {
    let mut exactly_two_count = 0;
    let mut exactly_three_count = 0;

    let contains_2 = |x| contains_n_of_any_letter(x, 2);
    let contains_3 = |x| contains_n_of_any_letter(x, 3);

    for line in reader.lines() {
        let word = line.unwrap();

        // How do I avoid this clone?
        if contains_2(word.clone()) {
            exactly_two_count += 1;
        }
        if contains_3(word) {
            exactly_three_count += 1;
        }
    }

    Some(exactly_three_count * exactly_two_count)
}

fn main() {
    let args = Cli::from_args();
    let reader = BufReader::new(File::open(&args.input).expect("input file not found"));

    println!("-----Part: {}------", AOC_PROBLEM);
    match solve(reader) {
        Some(solution) => println!("Solution: {}", solution),
        None => println!("Failed to find a solution. 🤨"),
    }
}
