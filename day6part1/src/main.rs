use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day6part1 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let mut sums = Vec::new();
    let mut unique_questions = HashSet::new();
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                sums.push(unique_questions.len());
                unique_questions.clear();
                continue;
            }
            for c in line.chars() {
                unique_questions.insert(c);
            }
        }
    }
    sums.push(unique_questions.len());
    println!("{}", sums.into_iter().fold(0, |a, b| a + b));
}
