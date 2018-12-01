use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part2(input_file: &str) {
    let f = File::open(&input_file).expect(format!("file not found: {}", input_file).as_str());

    let mut seen = HashSet::new();
    let lines: Vec<_> = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut current_freq: i32 = 0;
    seen.insert(0);

    for line in lines.iter().cycle() {
        let val = line.parse::<i32>().unwrap();
        current_freq += val;

        if seen.contains(&current_freq) {
            println!("Part 1.2 result is: {}", current_freq);
            return;
        }

        seen.insert(current_freq);
    }

    assert!(true, "No result found for part 2");
}

fn part1(input_file: &str) {
    let f = File::open(&input_file).expect(format!("file not found: {}", input_file).as_str());

    let sum: i32 = BufReader::new(f)
        .lines()
        .filter_map(|x| x.unwrap().parse::<i32>().ok())
        .sum();

    println!("Part 1.1 result is: {}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    part1(input_file);
    part2(input_file);
}
