use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

#[derive(Debug)]
enum InputError {
    InvalidCharacter(char),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InputError::InvalidCharacter(c) => write!(f, "invalid character: {}", c),
        }
    }
}

impl std::error::Error for InputError {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day3part1 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let map: Vec<Vec<bool>> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line).unwrap())
        .collect();

    let mut pos_x: usize = 0;
    let mut pos_y: usize = 0;
    let mut hit_trees: usize = 0;

    loop {
        pos_y += 1;
        if pos_y == map.len() {
            break;
        }
        pos_x += 3;
        let row = &map[pos_y];
        if row[pos_x % row.len()] {
            hit_trees += 1
        }
    }
    println!("{}", hit_trees);
}

fn parse_line(line: String) -> Result<Vec<bool>, InputError> {
    let mut trees = Vec::new();
    for c in line.chars() {
        match c {
            '.' => trees.push(false),
            '#' => trees.push(true),
            _ => return Err(InputError::InvalidCharacter(c)),
        }
    }
    Ok(trees)
}
