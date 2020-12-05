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
        println!("usage: day3part2 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let map: Vec<Vec<bool>> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line).unwrap())
        .collect();

    let slopes = [
        (1usize, 1usize),
        (3usize, 1usize),
        (5usize, 1usize),
        (7usize, 1usize),
        (1usize, 2usize),
    ];
    let mut values = Vec::new();
    for slope in slopes.iter() {
        let mut pos_x: usize = 0;
        let mut pos_y: usize = 0;
        let mut hit_trees: usize = 0;
        loop {
            pos_y += slope.1;
            if pos_y >= map.len() {
                break;
            }
            pos_x += slope.0;
            let row = &map[pos_y];
            if row[pos_x % row.len()] {
                hit_trees += 1
            }
        }
        values.push(hit_trees);
    }

    println!("{}", values.into_iter().fold(1, |a, b| a * b));
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
