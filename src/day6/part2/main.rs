use aoc::geometry::{BoundingBox, Point};
use aoc::ProblemHarness;

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
    let maximum_total_distance = 10_000;

    points.sort_by_key(|p| p.x);
    let bounding_box = find_bounding_box(&points);

    let mut area = 0;
    for x in (bounding_box.top_left.x)..=(bounding_box.bottom_right.x) {
        for y in (bounding_box.top_left.y)..=(bounding_box.bottom_right.y) {
            let check_point = Point { x: x, y: y };

            let mut total_distance = 0;
            for point in &points {
                total_distance += point.manhattan_distance_to(&check_point);
            }

            if total_distance < maximum_total_distance {
                area += 1;
            }
        }
    }

    Ok(area)
}

fn main() {
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
