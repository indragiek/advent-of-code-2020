use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let timestamp = lines[0].parse::<u32>().unwrap();
    let result = lines[1]
        .split(",")
        .filter(|&id| id != "x")
        .map(|id| id.parse::<u32>().unwrap())
        .map(|id| (id - (timestamp % id), id))
        .min()
        .unwrap();
    println!("{}", result.0 * result.1);
}
