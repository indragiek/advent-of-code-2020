use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut and_mask = 0u64;
    let mut or_mask = 0u64;

    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| match parse_bitmask_str(&line) {
            Some(bitmask) => {
                and_mask = bitmask.and_mask;
                or_mask = bitmask.or_mask;
            }
            None => match parse_memstore_str(&line) {
                Some(memstore) => match (memstore.value & and_mask) | or_mask {
                    0 => {
                        let _ = memory.remove(&memstore.address);
                    }
                    value => {
                        let _ = memory.insert(memstore.address, value);
                    }
                },
                None => {}
            },
        });

    let sum: u64 = memory.values().sum();
    println!("{}", sum);
}

struct ParsedBitmask {
    and_mask: u64,
    or_mask: u64,
}

fn parse_bitmask_str(line: &str) -> Option<ParsedBitmask> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mask = ([01X]{36})").unwrap();
    }
    return RE
        .captures(line)?
        .get(1)
        .map(|cap| cap.as_str())
        .map(|mask| ParsedBitmask {
            and_mask: u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap(),
            or_mask: u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap(),
        });
}

struct ParsedMemoryStore {
    address: u64,
    value: u64,
}

fn parse_memstore_str(line: &str) -> Option<ParsedMemoryStore> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }
    return RE.captures(line).map(|captures| ParsedMemoryStore {
        address: captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
        value: captures.get(2).unwrap().as_str().parse::<u64>().unwrap(),
    });
}
