use aoc::polymers::solve_inner;
use aoc::ProblemHarness;

use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "5.1";

fn solve(reader: &mut BufReader<File>) -> Option<usize> {
    let mut buf = vec![];
    let _ = reader.read_until(b'\n', &mut buf);
    let slice = buf.as_slice();

    // -2 as read_until includes the final newline byte, which we should ignore
    Some(solve_inner(slice, 0, slice.len() - 2, None).len())
}

fn main() {
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}

#[cfg(test)]
mod tests {
    use super::solve_inner;

    #[test]
    fn it_works() {
        let case = "dabAcCaCBAcCcaDA";
        assert_eq!(
            String::from_utf8(
                solve_inner(case.as_bytes(), 0, case.len() - 1, None)
                    .into_iter()
                    .collect()
            )
            .unwrap(),
            "dabCBAcaDA"
        );
    }
}
