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
    let invalid_number = 1038347917u64;
    let nums: Vec<u64> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|n| n <= &invalid_number)
        .collect();
    for window_size in 2..=nums.len() {
        let mut sum: u64 = nums[0..window_size].iter().sum();
        if sum == invalid_number {
            print_min_max_sum(&nums, 0, window_size - 1);
            return Ok(());
        }
        for offset in 1..=(nums.len() - window_size) {
            sum -= nums[offset - 1];
            let new_last = nums[window_size - 1 + offset];
            sum += new_last;
            if sum == invalid_number {
                print_min_max_sum(&nums, offset, window_size - 1 + offset);
                return Ok(());
            }
        }
    }

    Err(InputError::SequenceNotFound)
}

fn print_min_max_sum(nums: &[u64], start_idx: usize, end_idx: usize) {
    let slice = &nums[start_idx..=end_idx];
    println!(
        "{}",
        slice.iter().min().unwrap() + slice.iter().max().unwrap()
    );
}

#[derive(Debug)]
enum InputError {
    InvalidArguments,
    SequenceNotFound,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InputError::InvalidArguments => write!(f, "usage: day9part2 <path to input text file>"),
            InputError::SequenceNotFound => write!(
                f,
                "could not find a numeric sequence that adds up to the invalid number"
            ),
        }
    }
}

impl std::error::Error for InputError {}
