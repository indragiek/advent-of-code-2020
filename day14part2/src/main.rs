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
    let mut current_bitmask = String::new();

    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| match parse_bitmask_str(&line) {
            Some(bitmask) => current_bitmask = bitmask,
            None => match parse_memstore_str(&line) {
                Some(memstore) => bitmasked_addresses(memstore.address, &current_bitmask)
                    .iter()
                    .for_each(|&address| match memstore.value {
                        0 => {
                            let _ = memory.remove(&address);
                        }
                        value => {
                            let _ = memory.insert(address, value);
                        }
                    }),
                None => {}
            },
        });

    let sum: u64 = memory.values().sum();
    println!("{}", sum);
}

fn bitmasked_addresses(address: u64, bitmask: &str) -> Vec<u64> {
    let masked_addr: String = format!("{:036b}", address)
        .chars()
        .enumerate()
        .map(|(i, c)| match bitmask.chars().nth(i).unwrap() {
            'X' => 'X',
            '0' => c,
            '1' => '1',
            v => v,
        })
        .collect();
    let floating_count = masked_addr.chars().filter(|&c| c == 'X').count();
    (0..2u32.pow(floating_count as u32))
        .map(|combo| {
            let mut x_index = 0usize;
            let combo_str = format!("{:0width$b}", combo, width = floating_count);
            let new_addr: String = masked_addr
                .chars()
                .map(|c| match c {
                    'X' => {
                        let new_c = combo_str.chars().nth(x_index).unwrap();
                        x_index += 1;
                        return new_c;
                    }
                    ch => ch,
                })
                .collect();
            return u64::from_str_radix(&new_addr, 2).unwrap();
        })
        .collect()
}

fn parse_bitmask_str(line: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mask = ([01X]{36})").unwrap();
    }
    return RE
        .captures(line)?
        .get(1)
        .map(|cap| cap.as_str().to_string());
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
