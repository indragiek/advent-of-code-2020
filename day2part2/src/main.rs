use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

#[derive(Debug)]
enum InputError {
    InvalidLineFormat,
    InvalidPositions,
    InvalidLetter,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InputError::InvalidLineFormat => write!(f, "invalid line format"),
            InputError::InvalidPositions => write!(f, "invalid positions"),
            InputError::InvalidLetter => write!(f, "invalid letter"),
        }
    }
}

impl std::error::Error for InputError {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day2part2 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let count = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line).unwrap())
        .filter(|password| password.is_valid())
        .count();
    println!("{}", count);
}

struct PasswordPolicy {
    pos_1: usize,
    pos_2: usize,
    letter: char,
    password: String,
}

impl PasswordPolicy {
    fn is_valid(&self) -> bool {
        match (
            self.password.chars().nth(self.pos_1 - 1),
            self.password.chars().nth(self.pos_2 - 1),
        ) {
            (Some(c), None) => return c == self.letter,
            (None, Some(c)) => return c == self.letter,
            (Some(c1), Some(c2)) => {
                return (c1 == self.letter && c2 != self.letter)
                    || (c1 != self.letter && c2 == self.letter)
            }
            (None, None) => return false,
        }
    }
}

fn parse_line(line: String) -> Result<PasswordPolicy, InputError> {
    let components: Vec<&str> = line.split_whitespace().collect();
    if components.len() < 3 {
        return Err(InputError::InvalidLineFormat);
    }
    let min_max_components: Vec<&str> = components[0].split("-").collect();
    if min_max_components.len() < 2 {
        return Err(InputError::InvalidPositions);
    }

    let pos_1 = min_max_components[0]
        .parse::<usize>()
        .map_err(|_| InputError::InvalidPositions)?;
    let pos_2 = min_max_components[1]
        .parse::<usize>()
        .map_err(|_| InputError::InvalidPositions)?;

    let letter_str = components[1];
    if letter_str.is_empty() {
        return Err(InputError::InvalidLetter);
    }
    let letter = letter_str.chars().nth(0).ok_or(InputError::InvalidLetter)?;

    Ok(PasswordPolicy {
        pos_1: pos_1,
        pos_2: pos_2,
        letter: letter,
        password: String::from(components[2]),
    })
}
