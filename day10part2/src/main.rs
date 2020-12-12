use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::once;

fn main() -> Result<(), InputError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(InputError::InvalidArguments);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let mut adapters: Vec<u64> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
    adapters.sort();
    let device_joltage = adapters.iter().max().unwrap();
    println!("{}", count_combinations(&adapters, *device_joltage));
    Ok(())
}

fn count_combinations(adapters: &Vec<u64>, device_joltage: u64) -> u64 {
    let mut counts = HashMap::new();
    counts.insert(0u64, 1u64);

    for joltage in once(&0u64)
        .chain(adapters.clone().iter())
        .chain(once(&device_joltage))
    {
        let self_count = counts.get(&joltage).unwrap_or(&0).clone();
        for diff in 1..=3 {
            let new_joltage = joltage + diff;
            let new_count = counts.get(&new_joltage).unwrap_or(&0) + self_count;
            counts.insert(new_joltage, new_count);
        }
    }
    return counts.get(&device_joltage).unwrap_or(&0).clone();
}

#[derive(Debug)]
enum InputError {
    InvalidArguments,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InputError::InvalidArguments => {
                write!(f, "usage: day10part2 <path to input text file>")
            }
        }
    }
}

impl std::error::Error for InputError {}
