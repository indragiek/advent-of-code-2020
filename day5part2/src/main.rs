use std::collections::HashMap;
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
        println!("usage: day5part2 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let mut existing_seats = HashMap::new();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line).unwrap())
        .for_each(|pass| {
            let _ = existing_seats.insert(pass.id, true);
        });
    for row in 1..127 {
        for col in 0..8 {
            let id = seat_id(row, col);
            if existing_seats.contains_key(&id) {
                continue;
            }
            if existing_seats.contains_key(&(id - 1)) && existing_seats.contains_key(&(id + 1)) {
                println!("{}", id);
                process::exit(0);
            }
        }
    }
    println!("could not determine seat id");
    process::exit(1);
}

struct BoardingPass {
    row: u32,
    column: u32,
    id: u32,
}

fn seat_id(row: u32, col: u32) -> u32 {
    return (row * 8) + col;
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
        id: seat_id(row_start, col_start),
    })
}
