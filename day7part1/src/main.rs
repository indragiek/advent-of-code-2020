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
        println!("usage: day6part2 <path to input text file>");
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
    let count = bags
        .keys()
        .map(|k| can_contain_bag(&bags, k.clone(), "shiny gold"))
        .filter(|&x| x)
        .count();
    println!("{}", count);
}

fn can_contain_bag(
    bags: &HashMap<String, Vec<(u32, String)>>,
    check_color: String,
    want_color: &str,
) -> bool {
    return bags
        .get(&check_color)
        .unwrap_or(&Vec::new())
        .into_iter()
        .find(|bag| bag.1 == want_color || can_contain_bag(bags, bag.1.clone(), want_color))
        .is_some();
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
