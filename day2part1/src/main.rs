use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

#[derive(Debug)]
enum InputError {
    InvalidLineFormat,
    InvalidMinMax,
    InvalidLetter,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InputError::InvalidLineFormat => write!(f, "invalid line format"),
            InputError::InvalidMinMax => write!(f, "invalid min or max"),
            InputError::InvalidLetter => write!(f, "invalid letter"),
        }
    }
}

impl std::error::Error for InputError {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day1part2 <path to input text file>");
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
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordPolicy {
    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();
        return (count >= self.min) && (count <= self.max);
    }
}

fn parse_line(line: String) -> Result<PasswordPolicy, InputError> {
    let components: Vec<&str> = line.split_whitespace().collect();
    if components.len() < 3 {
        return Err(InputError::InvalidLineFormat);
    }
    let min_max_components: Vec<&str> = components[0].split("-").collect();
    if min_max_components.len() < 2 {
        return Err(InputError::InvalidMinMax);
    }

    let min = min_max_components[0]
        .parse::<usize>()
        .map_err(|_| InputError::InvalidMinMax)?;
    let max = min_max_components[1]
        .parse::<usize>()
        .map_err(|_| InputError::InvalidMinMax)?;

    let letter_str = components[1];
    if letter_str.is_empty() {
        return Err(InputError::InvalidLetter);
    }
    let letter = letter_str.chars().nth(0).ok_or(InputError::InvalidLetter)?;

    Ok(PasswordPolicy {
        min: min,
        max: max,
        letter: letter,
        password: String::from(components[2]),
    })
}
