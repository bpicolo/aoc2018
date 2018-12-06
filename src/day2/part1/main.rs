use aoc::ProblemHarness;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "2.1";

fn contains_n_of_any_letter(word: &str, n: i32) -> bool {
    word.chars()
        .fold(HashMap::new(), |mut acc, chr| {
            let entry = acc.entry(chr).or_insert(0);
            *entry += 1;
            acc
        })
        .values()
        .any(|f| *f == n)
}

fn solve(reader: &mut BufReader<File>) -> Option<i32> {
    let mut exactly_two_count = 0;
    let mut exactly_three_count = 0;

    let contains_2 = |x: &str| contains_n_of_any_letter(x, 2);
    let contains_3 = |x: &str| contains_n_of_any_letter(x, 3);

    for line in reader.lines() {
        let s = line.unwrap().to_owned();

        if contains_2(&s) {
            exactly_two_count += 1;
        }
        if contains_3(&s) {
            exactly_three_count += 1;
        }
    }

    Some(exactly_three_count * exactly_two_count)
}

fn main() {
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
