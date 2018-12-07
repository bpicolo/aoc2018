use aoc::ProblemHarness;

use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "1.1";

fn solve(reader: &mut BufReader<File>) -> Result<i32, String> {
    Ok(reader
        .lines()
        .filter_map(|x| x.unwrap().parse::<i32>().ok())
        .sum())
}

fn main() {
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
