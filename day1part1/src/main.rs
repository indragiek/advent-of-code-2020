use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day1 <path to input text file>");
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

    for i in 0..values.len() {
        let value = values[i];
        if let Ok(index) = values.binary_search(&(2020 - value)) {
            println!("{}", value * values[index]);
            process::exit(0);
        }
    }
    println!("did not find a pair of numbers that sum to 2020");
    process::exit(1);
}
