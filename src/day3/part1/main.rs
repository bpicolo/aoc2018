use aoc::geometry::FabricClaim;
use aoc::ProblemHarness;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "3.1";

fn solve(reader: &mut BufReader<File>) -> Result<usize, String> {
    let mut intersecting = HashSet::new();
    let mut claims: Vec<FabricClaim> = reader
        .lines()
        .map(|c| FabricClaim::from_serialized(c.unwrap()))
        .collect();

    // Simple sweep-and-prune style collision detection. We sort
    // these
    claims.sort_by(|a, b| (a.left).cmp(&b.left));

    for i in 0..claims.len() {
        let claim = &claims[i];

        for j in i + 1..claims.len() {
            let cmp_to_claim = &claims[j];

            // Reached end of candidate collision boxes
            if claim.right() <= cmp_to_claim.left {
                break;
            }

            match claim.intersecting_points(&cmp_to_claim) {
                Some(points) => {
                    for point in points {
                        intersecting.insert(point);
                    }
                }
                None => (),
            }
        }
    }

    Ok(intersecting.len())
}

fn main() {
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
