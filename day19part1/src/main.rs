use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let mut parse_stage = ParseStage::Rules;
    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| match line.as_str() {
            "" => parse_stage = ParseStage::Messages,
            _ => match parse_stage {
                ParseStage::Rules => {
                    let (rule_number, rule) = parse_rule(line);
                    let _ = rules.insert(rule_number, rule);
                }
                ParseStage::Messages => messages.push(line),
            },
        });

    println!(
        "{}",
        messages
            .iter()
            .map(|msg| {
                let (matches, count) =
                    eval_rule(rules.get(&0).unwrap(), &rules, &str_to_chars(msg.clone()));
                matches && (count == msg.len())
            })
            .filter(|&x| x)
            .count()
    );
}

enum ParseStage {
    Rules,
    Messages,
}

enum Rule {
    Character(char),
    Sequence(Vec<u32>),
    Or(Box<Rule>, Box<Rule>),
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::Character(c) => write!(f, "{}", c),
            Rule::Sequence(seq) => write!(f, "{:?}", seq),
            Rule::Or(r1, r2) => write!(f, "{} | {}", r1, r2),
        }
    }
}

fn parse_rule_numbers(s: &str) -> Vec<u32> {
    s.trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn eval_rule(rule: &Rule, rules: &HashMap<u32, Rule>, msg: &[char]) -> (bool, usize) {
    if msg.is_empty() {
        return (true, 0);
    }
    return match rule {
        Rule::Character(c) => (&msg[0] == c, 1),
        Rule::Sequence(seq) => {
            let mut i = 0usize;
            for rule_number in seq {
                let (matches, count) =
                    eval_rule(rules.get(&rule_number).unwrap(), rules, &msg[i..]);
                if !matches {
                    return (false, 0);
                }
                i += count;
            }
            return (true, i);
        }
        Rule::Or(r1, r2) => {
            let (matches1, count1) = eval_rule(r1, rules, msg);
            if matches1 {
                return (matches1, count1);
            }
            return eval_rule(r2, rules, msg);
        }
    };
}

fn parse_rule(line: String) -> (u32, Rule) {
    let components: Vec<&str> = line.split(":").collect();
    let rule_number = components[0].parse::<u32>().unwrap();
    let or_components: Vec<&str> = components[1].trim().split("|").collect();
    return match or_components.len() {
        1 => {
            let s = or_components[0];
            if s.starts_with("\"") {
                (rule_number, Rule::Character(s.chars().nth(1).unwrap()))
            } else {
                (rule_number, Rule::Sequence(parse_rule_numbers(&s)))
            }
        }
        2 => (
            rule_number,
            Rule::Or(
                Box::new(Rule::Sequence(parse_rule_numbers(or_components[0]))),
                Box::new(Rule::Sequence(parse_rule_numbers(or_components[1]))),
            ),
        ),
        _ => panic!("unexpected rule components: {:?}", or_components),
    };
}

fn str_to_chars(s: String) -> Vec<char> {
    s.chars().collect()
}
