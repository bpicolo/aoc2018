use aoc::ProblemHarness;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "3.1";

#[derive(Hash, PartialEq, Eq, Debug)]
struct IntersectionPoint {
    x: i32,
    y: i32,
}

struct FabricClaim {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

impl FabricClaim {
    pub fn from_serialized(s: String) -> FabricClaim {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)"
            )
            .unwrap();
        }
        let caps = RE.captures(s.as_str()).unwrap();

        FabricClaim {
            id: caps["id"].parse().unwrap(),
            top: caps["top"].parse().unwrap(),
            left: caps["left"].parse().unwrap(),
            width: caps["width"].parse().unwrap(),
            height: caps["height"].parse().unwrap(),
        }
    }

    #[inline]
    pub fn right(&self) -> i32 {
        return self.left + self.width;
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        return self.top + self.height;
    }

    pub fn intersects(&self, other: &FabricClaim) -> bool {
        self.left < other.right()
            && self.right() > other.left
            && self.top < other.bottom()
            && self.bottom() > other.top
    }

    pub fn intersecting_points(&self, other: &FabricClaim) -> Option<Vec<IntersectionPoint>> {
        if !self.intersects(other) {
            None
        } else {
            let left = cmp::max(self.left, other.left);
            let top = cmp::max(self.top, other.top);
            let right = cmp::min(self.right(), other.right());
            let bottom = cmp::min(self.bottom(), other.bottom());

            let mut out = Vec::new();
            for i in left..right {
                for j in top..bottom {
                    out.push(IntersectionPoint { x: i, y: j })
                }
            }

            Some(out)
        }
    }
}

fn solve(reader: BufReader<File>) -> Option<usize> {
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

    Some(intersecting.len())
}

fn main() {
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
