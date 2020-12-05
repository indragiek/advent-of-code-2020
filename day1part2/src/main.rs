use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day1part2 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let mut values: Vec<u32> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    values.sort();

    let target_sum: u32 = 2020;
    for i in 0..values.len() {
        let i_value = values[i];
        if i_value > target_sum {
            continue;
        }
        if let Ok((j, k)) = indices_summing_to(&values, target_sum - i_value) {
            println!("{}", i_value * values[j] * values[k]);
            process::exit(0);
        }
    }
    println!("did not find a trio of numbers that sum to {}", target_sum);
    process::exit(1);
}

fn indices_summing_to(values: &[u32], sum: u32) -> Result<(usize, usize), Box<dyn Error>> {
    for i in 0..values.len() {
        let i_value = values[i];
        if i_value > sum {
            continue;
        }
        if let Ok(j) = values.binary_search(&(sum - i_value)) {
            return Ok((i, j));
        }
    }
    return Err("no pairs that produce the sum".into());
}
