use std::collections::HashSet;
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
    let num_fields = your_ticket.len();
    let valid_tickets: Vec<Vec<u32>> = nearby_tickets
        .into_iter()
        .filter(|ticket| is_valid_ticket(ticket, &rules))
        .collect();
    let mut valid_rules: Vec<(usize, Vec<String>)> = (0..num_fields)
        .map(|i| -> (usize, Vec<u32>) {
            (i, valid_tickets.iter().map(|ticket| ticket[i]).collect())
        })
        .map(|(i, values)| -> (usize, Vec<String>) {
            (
                i,
                rules
                    .iter()
                    .filter(|rule| {
                        values
                            .iter()
                            .map(|&value| !rule.validate(value))
                            .filter(|&x| x)
                            .count()
                            == 0
                    })
                    .map(|rule| rule.field.clone())
                    .collect(),
            )
        })
        .collect();
    valid_rules.sort_by(|a, b| a.1.len().partial_cmp(&b.1.len()).unwrap());

    let mut claimed_fields = HashSet::new();
    let field_assignments: Vec<(usize, String)> = valid_rules
        .iter()
        .map(|(i, fields)| -> (usize, String) {
            let available_fields: Vec<String> = fields
                .clone()
                .into_iter()
                .filter(|field| !claimed_fields.contains(field))
                .collect();
            (
                i.clone(),
                match available_fields.len() {
                    1 => {
                        let field = available_fields[0].clone();
                        claimed_fields.insert(field.clone());
                        field
                    }
                    _ => "<undefined>".to_string(),
                },
            )
        })
        .collect();

    let product: u64 = field_assignments
        .iter()
        .filter(|(_, field)| field.starts_with("departure"))
        .map(|&(i, _)| your_ticket[i] as u64)
        .product();
    println!("{}", product);
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

fn is_valid_ticket(ticket: &Vec<u32>, rules: &Vec<Rule>) -> bool {
    return ticket
        .iter()
        .map(|&field| {
            rules
                .iter()
                .map(|rule| rule.validate(field))
                .filter(|&x| x)
                .count()
        })
        .find(|&count| count == 0)
        .is_none();
}
