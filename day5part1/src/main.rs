use std::cmp::max;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

#[derive(Debug)]
enum InputError {
    InvalidLine(String),
    InvalidChar(char),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InputError::InvalidLine(s) => write!(f, "invalid line: {}", s),
            InputError::InvalidChar(c) => write!(f, "invalid char: {}", c),
        }
    }
}

impl std::error::Error for InputError {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day5part1 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let mut max_id: u32 = 0;
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line).unwrap())
        .for_each(|passport| max_id = max(max_id, passport.id));

    println!("{}", max_id);
}

struct BoardingPass {
    row: u32,
    column: u32,
    id: u32,
}

fn parse_line(line: String) -> Result<BoardingPass, InputError> {
    if line.len() != 10 {
        return Err(InputError::InvalidLine(line));
    }
    let mut row_start: u32 = 0;
    let mut row_end: u32 = 127;
    for c in line[..7].chars() {
        match c {
            'F' => row_end -= (row_end - row_start + 1) / 2,
            'B' => row_start += (row_end - row_start + 1) / 2,
            _ => return Err(InputError::InvalidChar(c)),
        }
    }

    let mut col_start: u32 = 0;
    let mut col_end: u32 = 7;
    for c in line[7..].chars() {
        match c {
            'R' => col_start += (col_end - col_start + 1) / 2,
            'L' => col_end -= (col_end - col_start + 1) / 2,
            _ => return Err(InputError::InvalidChar(c)),
        }
    }

    Ok(BoardingPass {
        row: row_start,
        column: col_start,
        id: (row_start * 8) + col_start,
    })
}
