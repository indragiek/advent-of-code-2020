use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::option::Option;
use std::process;

#[derive(Debug)]
enum InputError {
    InvalidKeyValuePair(String),
    UnsupportedKey(String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InputError::InvalidKeyValuePair(s) => write!(f, "invalid key value pair: {}", s),
            InputError::UnsupportedKey(s) => write!(f, "unsupported key: {}", s),
        }
    }
}

impl std::error::Error for InputError {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day4part1 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let mut passport = Passport::new();
    let mut valid_passports = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                if passport.is_valid() {
                    valid_passports += 1
                }
                passport.clear();
                continue;
            }
            passport.merge_line(line).expect("failed to parse line");
        }
    }
    println!("{}", valid_passports);
}

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        return Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
    }

    fn merge_line(&mut self, line: String) -> Result<(), InputError> {
        let pairs: Result<Vec<_>, _> = line
            .split_whitespace()
            .map(|raw_pair| parse_key_value(raw_pair))
            .collect();
        for (key, value) in pairs? {
            match key {
                "byr" => self.byr = Some(value.to_string()),
                "iyr" => self.iyr = Some(value.to_string()),
                "eyr" => self.eyr = Some(value.to_string()),
                "hgt" => self.hgt = Some(value.to_string()),
                "hcl" => self.hcl = Some(value.to_string()),
                "ecl" => self.ecl = Some(value.to_string()),
                "pid" => self.pid = Some(value.to_string()),
                "cid" => self.cid = Some(value.to_string()),
                _ => return Err(InputError::UnsupportedKey(key.to_string())),
            }
        }
        Ok(())
    }

    fn is_valid(&self) -> bool {
        return self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();
    }

    fn clear(&mut self) {
        self.byr = None;
        self.iyr = None;
        self.eyr = None;
        self.hgt = None;
        self.hcl = None;
        self.ecl = None;
        self.pid = None;
        self.cid = None;
    }
}

fn parse_key_value(raw_pair: &str) -> Result<(&str, &str), InputError> {
    let components: Vec<&str> = raw_pair.split(":").collect();
    if components.len() < 2 {
        return Err(InputError::InvalidKeyValuePair(raw_pair.to_string()));
    }
    Ok((components[0], components[1]))
}
