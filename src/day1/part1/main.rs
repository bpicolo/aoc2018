use structopt::StructOpt;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

static AOC_PROBLEM: &'static str = "1.1";

#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn solve(reader: BufReader<File>) -> Option<i32> {
    Some(
        reader
            .lines()
            .filter_map(|x| x.unwrap().parse::<i32>().ok())
            .sum(),
    )
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
