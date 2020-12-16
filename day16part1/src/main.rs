use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

enum ParseStage {
    Rules,
    YourTicket,
    NearbyTickets,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let mut stage = ParseStage::Rules;
    let mut rules = Vec::new();
    let mut your_ticket = Vec::new();
    let mut nearby_tickets = Vec::new();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| match line.as_str() {
            "" => {}
            "your ticket:" => stage = ParseStage::YourTicket,
            "nearby tickets:" => stage = ParseStage::NearbyTickets,
            l => match &stage {
                ParseStage::Rules => rules.push(parse_rule(&l).unwrap()),
                ParseStage::YourTicket => your_ticket = parse_ticket(l),
                ParseStage::NearbyTickets => nearby_tickets.push(parse_ticket(l)),
            },
        });
    let rate: u32 = nearby_tickets
        .iter()
        .map(|ticket| ticket_scanning_error_rate(ticket, &rules))
        .sum();
    println!("{}", rate);
}

struct Range {
    start: u32,
    end: u32,
}

struct Rule {
    field: String,
    ranges: Vec<Range>,
}

impl Rule {
    fn validate(&self, value: u32) -> bool {
        for r in &self.ranges {
            if value >= r.start && value <= r.end {
                return true;
            }
        }
        return false;
    }
}

fn parse_ticket(line: &str) -> Vec<u32> {
    return line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
}

fn parse_rule(line: &str) -> Option<Rule> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([\w\s]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    }
    let captures = RE.captures(line)?;
    return Some(Rule {
        field: captures.get(1)?.as_str().to_string(),
        ranges: vec![
            Range {
                start: captures.get(2)?.as_str().parse::<u32>().ok()?,
                end: captures.get(3)?.as_str().parse::<u32>().ok()?,
            },
            Range {
                start: captures.get(4)?.as_str().parse::<u32>().ok()?,
                end: captures.get(5)?.as_str().parse::<u32>().ok()?,
            },
        ],
    });
}

fn ticket_scanning_error_rate(ticket: &Vec<u32>, rules: &Vec<Rule>) -> u32 {
    return ticket
        .iter()
        .map(|&field| {
            if rules
                .iter()
                .map(|rule| rule.validate(field))
                .filter(|&x| x)
                .count()
                == 0
            {
                return field;
            } else {
                return 0;
            }
        })
        .sum();
}
