use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn part2(input_file: &str) {
    // It feels like we could do something interesting here with a sort of modified
    // trie-building algorithm to check as we go, but let's brute force for now ;)
    let f = File::open(&input_file).expect(format!("file not found: {}", input_file).as_str());
    let words: Vec<String> = BufReader::new(f).lines().map(|x| x.unwrap()).collect();

    for i in 0..words.len() {
        for j in i..words.len() {
            if differ_by_single_char(&words[i], &words[j]) {
                let common: String = words[i]
                    .chars()
                    .zip(words[j].chars())
                    .filter(|item| item.0 == item.1)
                    .map(|item| item.0)
                    .collect();

                println!(
                    "Part 2.2 found similar words {} and {}, result is: {}",
                    words[i], words[j], common,
                );
            }
        }
    }
}

fn contains_n_of_any_letter(word: String, n: i32) -> bool {
    word.chars()
        .fold(HashMap::new(), |mut acc, chr| {
            let entry = acc.entry(chr).or_insert(0);
            *entry += 1;
            acc
        })
        .values()
        .any(|f| *f == n)
}

fn part1(input_file: &str) {
    let f = File::open(&input_file).expect(format!("file not found: {}", input_file).as_str());

    let mut exactly_two_count = 0;
    let mut exactly_three_count = 0;

    let contains_2 = |x| contains_n_of_any_letter(x, 2);
    let contains_3 = |x| contains_n_of_any_letter(x, 3);

    for line in BufReader::new(f).lines() {
        let word = line.unwrap();

        // How do I avoid this clone?
        if contains_2(word.clone()) {
            exactly_two_count += 1;
        }
        if contains_3(word) {
            exactly_three_count += 1;
        }
    }

    println!(
        "Part 2.1 result is: {}",
        exactly_three_count * exactly_two_count
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    part1(input_file);
    part2(input_file);
}
