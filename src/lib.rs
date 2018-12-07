use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

pub mod geometry;
pub mod polymers;

use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

pub struct ProblemHarness<'a, T: Debug> {
    problem: &'static str,
    solution: &'a Fn(&mut BufReader<File>) -> Result<T, String>,
}

impl<'a, T: Debug> ProblemHarness<'a, T> {
    pub fn new(
        problem: &'static str,
        solution: &'a Fn(&mut BufReader<File>) -> Result<T, String>,
    ) -> ProblemHarness<'a, T> {
        ProblemHarness {
            problem: problem,
            solution: solution,
        }
    }

    pub fn solve(&self) {
        let args = Cli::from_args();
        let mut reader = BufReader::new(File::open(&args.input).expect("input file not found"));

        println!("-----Part: {}------", self.problem);
        match (self.solution)(&mut reader) {
            Ok(out) => println!("Solution: {:?}", out),
            Err(e) => println!("Failed to find a solution: {}", e),
        }
    }
}
