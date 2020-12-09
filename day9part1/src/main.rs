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
    let mut lines_iter = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let mut nums = Vec::new();
    for _ in 0..25 {
        match lines_iter.next() {
            Some(l) => nums.push(l.parse::<u64>().unwrap()),
            None => return Err(InputError::InvalidPreamble),
        }
    }
    loop {
        match lines_iter.next() {
            Some(l) => {
                let num = l.parse::<u64>().unwrap();
                if !has_values_summing_to(&nums[(nums.len() - 25)..], num) {
                    println!("{}", num);
                    return Ok(());
                }
                nums.push(num);
            }
            None => break,
        }
    }
    Ok(())
}

fn has_values_summing_to(nums: &[u64], sum: u64) -> bool {
    let mut copy = vec![0; nums.len()];
    copy.copy_from_slice(nums);
    copy.sort();
    for i in 0..copy.len() {
        let value = copy[i];
        if sum < value {
            continue;
        }
        if let Ok(_) = copy.binary_search(&(sum - value)) {
            return true;
        }
    }
    return false;
}

#[derive(Debug)]
enum InputError {
    InvalidArguments,
    InvalidPreamble,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InputError::InvalidArguments => write!(f, "usage: day9part1 <path to input text file>"),
            InputError::InvalidPreamble => write!(f, "invalid preamble"),
        }
    }
}

impl std::error::Error for InputError {}
