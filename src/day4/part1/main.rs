use aoc::ProblemHarness;

#[macro_use]
extern crate lazy_static;

use chrono::{NaiveDate, NaiveDateTime, Timelike};
use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

static AOC_PROBLEM: &'static str = "4.1";

#[derive(Debug)]
enum LogRecordType {
    StartedShift(i32),
    FellAsleep,
    WokeUp,
}

#[derive(Debug)]
struct LogRecord {
    time: NaiveDateTime,
    record_type: LogRecordType,
}

struct SleepSpan {
    start: NaiveDateTime,
    end: Option<NaiveDateTime>,
}

impl LogRecord {
    fn deserialize(s: &str) -> LogRecord {
        lazy_static! {
            static ref DATETIME_RE: Regex = Regex::new(
                r"\[(?P<year>\d+)\-(?P<month>\d+)\-(?P<day>\d+) (?P<hour>\d+):(?P<minute>\d+)\]"
            )
            .unwrap();
            static ref GUARD_ID_RE: Regex = Regex::new(r" #(?P<id>\d+) ").unwrap();
        }

        let caps = DATETIME_RE.captures(s).unwrap();
        let time = NaiveDate::from_ymd(
            caps["year"].parse().unwrap(),
            caps["month"].parse().unwrap(),
            caps["day"].parse().unwrap(),
        )
        .and_hms(
            caps["hour"].parse().unwrap(),
            caps["minute"].parse().unwrap(),
            0,
        );

        LogRecord {
            time: time,
            record_type: if s.contains("falls asleep") {
                LogRecordType::FellAsleep
            } else if s.contains("wakes up") {
                LogRecordType::WokeUp
            } else {
                let caps = GUARD_ID_RE.captures(s).unwrap();
                LogRecordType::StartedShift(caps["id"].parse().unwrap())
            },
        }
    }
}

fn find_sleepiest_guard(spans_by_guard: &HashMap<i32, Vec<SleepSpan>>) -> (i32, i64) {
    let sleepiest = spans_by_guard
        .iter()
        .map(|(k, v)| {
            (
                k,
                v.iter().fold(0, |sum, span| {
                    sum + (span.end.unwrap() - span.start).num_minutes()
                }),
            )
        })
        .max_by_key(|k| k.1)
        .expect("Expected at least one sleepy guard.");

    (*sleepiest.0, sleepiest.1)
}

fn find_most_slept_minute(spans: &Vec<SleepSpan>) -> i32 {
    // These are guaranteed to be in the midnight hour 00:00 -> 00:59
    let mut histogram = HashMap::new();
    for span in spans {
        for i in span.start.minute()..span.end.unwrap().minute() {
            let minute = histogram.entry(i).or_insert(0);
            *minute += 1;
        }
    }

    let most_sleepy_minute = histogram.iter().max_by_key(|k| k.1).unwrap().0;

    *most_sleepy_minute as i32
}

fn solve(reader: BufReader<File>) -> Option<i32> {
    let mut records: Vec<_> = reader
        .lines()
        .map(|c| LogRecord::deserialize(c.unwrap().as_str()))
        .collect();

    records.sort_by_key(|r| r.time);

    let mut sleeping_spans_by_guard: HashMap<i32, Vec<SleepSpan>> = HashMap::new();
    let mut current_guard_id: Option<i32> = None;
    let mut current_span: Option<SleepSpan> = None;

    for record in records {
        match record.record_type {
            LogRecordType::StartedShift(guard_id) => {
                current_guard_id = Some(guard_id);
            }
            LogRecordType::FellAsleep => {
                current_span = Some(SleepSpan {
                    start: record.time,
                    end: None,
                });
            }
            LogRecordType::WokeUp => {
                if !sleeping_spans_by_guard.contains_key(&current_guard_id.unwrap()) {
                    sleeping_spans_by_guard.insert(current_guard_id.unwrap(), vec![]);
                }
                let mut span = current_span.unwrap();
                span.end = Some(record.time);

                sleeping_spans_by_guard
                    .get_mut(&current_guard_id.expect("No active guard woke up?"))
                    .unwrap()
                    .push(span);
                current_span = None;
            }
        }
    }

    let (sleepiest_guard_id, sleepytime) = find_sleepiest_guard(&sleeping_spans_by_guard);
    println!(
        "Sleepiest guard is {} having slept for {} minutes",
        sleepiest_guard_id, sleepytime
    );
    let most_slept =
        find_most_slept_minute(sleeping_spans_by_guard.get(&sleepiest_guard_id).unwrap());

    Some(most_slept * sleepiest_guard_id)
}

fn main() {
    ProblemHarness::new(AOC_PROBLEM, &solve).solve()
}
