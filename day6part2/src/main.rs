use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::once;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day6part2 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let mut sums = Vec::new();
    let mut unique_questions = HashMap::new();
    let mut group_count = 0;
    for line in io::BufReader::new(file)
        .lines()
        .chain(once(Ok("".to_string())))
    {
        if let Ok(line) = line {
            if line.is_empty() {
                sums.push(
                    unique_questions
                        .iter()
                        .filter(|&(_, v)| v == &group_count)
                        .count(),
                );
                unique_questions.clear();
                group_count = 0;
                continue;
            }
            for c in line.chars() {
                unique_questions.insert(c, unique_questions.get(&c).unwrap_or(&0) + 1);
            }
            group_count += 1;
        }
    }
    println!("{}", sums.into_iter().fold(0, |a, b| a + b));
}
