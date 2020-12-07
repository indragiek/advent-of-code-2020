use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

struct Rule {
    parent_bag_color: String,
    child_bags: Vec<(u32, String)>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day7part2 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let bags: HashMap<_, _> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line).expect("failed to parse line"))
        .map(|rule| (rule.parent_bag_color, rule.child_bags))
        .into_iter()
        .collect();
    println!("{}", count_bags(&bags, "shiny gold"));
}

fn count_bags(bags: &HashMap<String, Vec<(u32, String)>>, check_color: &str) -> u32 {
    return bags
        .get(check_color)
        .unwrap_or(&Vec::new())
        .into_iter()
        .map(|bag| bag.0 + (bag.0 * count_bags(bags, bag.1.as_str())))
        .sum();
}

fn parse_line(line: String) -> Result<Rule, InputError> {
    let root_components: Vec<&str> = line.split(" bags contain ").collect();
    if root_components.len() != 2 {
        return Err(InputError::InvalidLine(line));
    }
    Ok(Rule {
        parent_bag_color: root_components[0].to_string(),
        child_bags: root_components[1]
            .split(", ")
            .filter(|component| !component.starts_with("no other bags"))
            .map(|component| {
                let count_and_color: Vec<&str> = component
                    .split(" bag")
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .collect();
                return (
                    count_and_color[0].parse::<u32>().unwrap(),
                    count_and_color[1..].join(" "),
                );
            })
            .collect(),
    })
}

#[derive(Debug)]
enum InputError {
    InvalidLine(String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InputError::InvalidLine(s) => write!(f, "invalid line: {}", s),
        }
    }
}

impl std::error::Error for InputError {}
