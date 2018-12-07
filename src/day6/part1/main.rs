use aoc::geometry::{BoundingBox, Point};
use aoc::ProblemHarness;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "6.1";

fn parse_point(s: &String) -> Point {
    let pieces = s.split(", ").collect::<Vec<_>>();

    Point {
        x: pieces[0].parse().unwrap(),
        y: pieces[1].parse().unwrap(),
    }
}

fn find_bounding_box(points: &Vec<Point>) -> BoundingBox {
    BoundingBox {
        top_left: Point {
            x: points.iter().map(|p| p.x).min().unwrap(),
            y: points.iter().map(|p| p.y).min().unwrap(),
        },
        bottom_right: Point {
            x: points.iter().map(|p| p.x).max().unwrap(),
            y: points.iter().map(|p| p.y).max().unwrap(),
        },
    }
}

fn solve(reader: &mut BufReader<File>) -> Result<i32, String> {
    let mut points: Vec<_> = reader.lines().map(|s| parse_point(&s.unwrap())).collect();

    points.sort_by_key(|p| p.x);

    let bounding_box = find_bounding_box(&points);
    let mut have_infinite_bounds = HashSet::new();
    let mut owned_areas = HashMap::new();

    for x in (bounding_box.top_left.x)..=(bounding_box.bottom_right.x) {
        for y in (bounding_box.top_left.y)..=(bounding_box.bottom_right.y) {
            let fill_point = Point { x: x, y: y };

            let mut closest_point = None;
            let mut closest_distance = std::i32::MAX;

            for point in &points {
                let distance = fill_point.manhattan_distance_to(&point);

                if distance == closest_distance {
                    closest_point = None;
                    continue; // this point is not closest to any point
                } else if distance < closest_distance {
                    closest_point = Some(point);
                    closest_distance = distance;
                }
            }

            match closest_point {
                None => continue,
                Some(p) => {
                    if fill_point.is_on_boundary(&bounding_box) {
                        have_infinite_bounds.insert(p);
                    }

                    let area = owned_areas.entry(p).or_insert(0);
                    *area += 1;
                }
            }
        }
    }

    Ok(*owned_areas
        .iter()
        .filter(|&(k, _)| !have_infinite_bounds.contains(k))
        .max_by_key(|kv| kv.1)
        .unwrap()
        .1)
}

fn main() {
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
