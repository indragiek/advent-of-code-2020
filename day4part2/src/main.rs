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
        println!("usage: day4part2 <path to input text file>");
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
        return validate_birth_year(self.byr.clone())
            && validate_issue_year(self.iyr.clone())
            && validate_expiration_year(self.eyr.clone())
            && validate_height(self.hgt.clone())
            && validate_hair_color(self.hcl.clone())
            && validate_eye_color(self.ecl.clone())
            && validate_passport_id(self.pid.clone());
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

fn validate_birth_year(byr: Option<String>) -> bool {
    return byr
        .map(|x| x.parse::<u32>().unwrap())
        .filter(|&x| x >= 1920 && x <= 2002)
        .is_some();
}

fn validate_issue_year(byr: Option<String>) -> bool {
    return byr
        .map(|x| x.parse::<u32>().unwrap())
        .filter(|&x| x >= 2010 && x <= 2020)
        .is_some();
}

fn validate_expiration_year(byr: Option<String>) -> bool {
    return byr
        .map(|x| x.parse::<u32>().unwrap())
        .filter(|&x| x >= 2020 && x <= 2030)
        .is_some();
}

fn validate_height(byr: Option<String>) -> bool {
    match byr {
        None => return false,
        Some(mut byr) => {
            byr.pop();
            match byr.pop() {
                None => return false,
                Some('c') => {
                    let cm = byr.parse::<u32>().unwrap();
                    return cm >= 150 && cm <= 193;
                }
                Some('i') => {
                    let inch = byr.parse::<u32>().unwrap();
                    return inch >= 59 && inch <= 76;
                }
                _ => return false,
            }
        }
    }
}

fn validate_hair_color(hcl: Option<String>) -> bool {
    match hcl {
        None => false,
        Some(hcl) => {
            hcl.len() == 7
                && hcl.starts_with("#")
                && hcl[1..].chars().all(|x| char::is_ascii_hexdigit(&x))
        }
    }
}

fn validate_eye_color(ecl: Option<String>) -> bool {
    if ecl.is_none() {
        return false;
    }

    match ecl.unwrap().as_str() {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn validate_passport_id(pid: Option<String>) -> bool {
    match pid {
        None => false,
        Some(pid) => pid.len() == 9 && pid.chars().all(|x| char::is_numeric(x)),
    }
}
