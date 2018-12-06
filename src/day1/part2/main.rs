use aoc::ProblemHarness;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "1.2";

fn solve(reader: &mut BufReader<File>) -> Option<i32> {
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
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
