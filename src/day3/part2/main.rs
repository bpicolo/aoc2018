use aoc::geometry::FabricClaim;
use aoc::ProblemHarness;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "3.1";

fn solve(reader: BufReader<File>) -> Option<i32> {
    let mut intersecting = HashSet::new();
    let mut claims: Vec<FabricClaim> = reader
        .lines()
        .map(|c| FabricClaim::from_serialized(c.unwrap()))
        .collect();

    claims.sort_by(|a, b| (a.left).cmp(&b.left));

    for i in 0..claims.len() {
        let claim = &claims[i];

        for j in i + 1..claims.len() {
            let cmp_to_claim = &claims[j];
            if claim.right() <= cmp_to_claim.left {
                break;
            }

            if claim.intersects(&cmp_to_claim) {
                intersecting.insert(claim.id);
                intersecting.insert(cmp_to_claim.id);
            }
        }
    }

    for i in 0..claims.len() {
        if !intersecting.contains(&claims[i].id) {
            return Some(claims[i].id);
        }
    }

    None
}

fn main() {
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
