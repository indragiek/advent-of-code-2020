use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let sum: i64 = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            eval(&chars)
        })
        .sum();
    println!("{}", sum);
}

fn parse_value_from_start(expr: &[char]) -> (i64, usize) {
    let mut end = 1usize;
    let mut is_subexpr = false;
    match expr[0] {
        '0'..='9' => {
            while end < expr.len() {
                match expr[end] {
                    '0'..='9' => end += 1,
                    _ => break,
                }
            }
        }
        '(' => {
            is_subexpr = true;
            let mut bracket_count = 1usize;
            while bracket_count > 0 {
                match expr[end] {
                    '(' => bracket_count += 1,
                    ')' => bracket_count -= 1,
                    _ => {}
                }
                end += 1;
            }
        }
        c => panic!(
            "unrecognized lhs character: '{}' in '{}'",
            c,
            chars_to_str(expr)
        ),
    }
    let lhs_val = match is_subexpr {
        true => eval(&expr[1..(end - 1)]),
        false => {
            let lhs: String = expr[0..end].iter().collect();
            lhs.parse::<i64>().unwrap()
        }
    };
    return (lhs_val, end);
}

fn chars_to_str(expr: &[char]) -> String {
    expr.iter().collect()
}

fn eval(expr: &[char]) -> i64 {
    if expr.is_empty() {
        return 0;
    }
    let mut acc = 0i64;
    let mut i = 0usize;
    while i < expr.len() {
        match expr[i] {
            ' ' => i += 1,
            '+' => {
                i += 2;
                let (val, end) = parse_value_from_start(&expr[i..]);
                acc += val;
                i += end;
            }
            '*' => {
                i += 2;
                let (val, end) = parse_value_from_start(&expr[i..]);
                acc *= val;
                i += end;
            }
            '0'..='9' | '(' => {
                let (val, end) = parse_value_from_start(&expr[i..]);
                acc = val;
                i += end;
            }
            c => panic!(
                "unrecognized lhs character: '{}' in '{}'",
                c,
                chars_to_str(expr)
            ),
        }
    }
    return acc;
}
