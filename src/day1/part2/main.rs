use structopt::StructOpt;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

static AOC_PROBLEM: &'static str = "1.2";

#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn solve(reader: BufReader<File>) -> Option<i32> {
    let mut seen = HashSet::new();
    let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    let mut current_freq: i32 = 0;
    seen.insert(0);

    for line in lines.iter().cycle() {
        let val = line.parse::<i32>().unwrap();
        current_freq += val;

        if seen.contains(&current_freq) {
            return Some(current_freq);
        }

        seen.insert(current_freq);
    }

    None
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
