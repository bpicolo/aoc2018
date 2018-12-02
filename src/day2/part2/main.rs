use aoc::ProblemHarness;

use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "2.2";

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

    for i in 0..words.len() - 1 {
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
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
