use aoc::polymers::solve_inner;
use aoc::ProblemHarness;

use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "5.2";

fn solve(reader: &mut BufReader<File>) -> Result<usize, String> {
    let mut buf = vec![];
    let _ = reader.read_until(b'\n', &mut buf);
    let slice = buf.as_slice();
    // We know these are ascii, so no need to complicate
    let potential_polymers = String::from("abcdefghijklmnopqrstuvwxyz");

    Ok(potential_polymers
        .as_bytes()
        .into_iter()
        .map(|ignore_polymer| solve_inner(slice, 0, slice.len() - 2, Some(*ignore_polymer)).len())
        .min()
        .unwrap())
}

fn main() {
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
