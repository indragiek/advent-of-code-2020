use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), InputError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(InputError::InvalidArguments);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let mut adapters: HashSet<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<u32>().unwrap())
        .collect();
    let mut diff_one_count = 0u32;
    let mut diff_three_count = 1u32;
    let mut source_joltage = 0u32;
    while !adapters.is_empty() {
        let mut found_compatible_adapter = false;
        for diff in 1..=3 {
            let total_joltage = source_joltage + diff;
            if adapters.contains(&total_joltage) {
                adapters.remove(&total_joltage);
                match diff {
                    1 => diff_one_count += 1,
                    3 => diff_three_count += 1,
                    _ => {}
                }
                source_joltage = total_joltage;
                found_compatible_adapter = true;
                break;
            }
        }
        if !found_compatible_adapter {
            return Err(InputError::CouldNotFindCompatibleAdapter(source_joltage));
        }
    }
    println!("{}", diff_one_count * diff_three_count);
    Ok(())
}

#[derive(Debug)]
enum InputError {
    InvalidArguments,
    CouldNotFindCompatibleAdapter(u32),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InputError::InvalidArguments => {
                write!(f, "usage: day10part1 <path to input text file>")
            }
            InputError::CouldNotFindCompatibleAdapter(src) => write!(
                f,
                "could not find compatible adapter for source joltage {}",
                src
            ),
        }
    }
}

impl std::error::Error for InputError {}
