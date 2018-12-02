use structopt::StructOpt;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

static AOC_PROBLEM: &'static str = "2.2";

#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn differ_by_single_char(a: &String, b: &String) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.chars().zip(b.chars()).fold(
        0,
        |acc, chars| {
            if chars.0 != chars.1 {
                acc + 1
            } else {
                acc
            }
        },
    ) == 1
}

fn solve(reader: BufReader<File>) -> Option<String> {
    // It feels like we could do something interesting here with a sort of modified
    // trie-building algorithm to check as we go, but let's brute force for now ;)
    let words: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    for i in 0..words.len() {
        for j in i..words.len() {
            if differ_by_single_char(&words[i], &words[j]) {
                let common: String = words[i]
                    .chars()
                    .zip(words[j].chars())
                    .filter(|item| item.0 == item.1)
                    .map(|item| item.0)
                    .collect();

                return Some(common);
            }
        }
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
