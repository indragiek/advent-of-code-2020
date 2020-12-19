use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let sum: i64 = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| eval(build_ast(&str_to_chars(line))))
        .sum();
    println!("{}", sum);
}

enum AST {
    Value(i64),
    Subexpr(Box<AST>),
    Addition(Box<AST>, Box<AST>),
    Multiplication(Box<AST>, Box<AST>),
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AST::Value(v) => write!(f, "{}", v),
            AST::Subexpr(ast) => write!(f, "({})", ast),
            AST::Addition(lhs, rhs) => write!(f, "{} + {}", lhs, rhs),
            AST::Multiplication(lhs, rhs) => write!(f, "{} * {}", lhs, rhs),
        }
    }
}

fn eval(ast: Box<AST>) -> i64 {
    return match *ast {
        AST::Value(v) => v,
        AST::Subexpr(subexpr_ast) => eval(subexpr_ast),
        AST::Addition(lhs, rhs) => eval(lhs) + eval(rhs),
        AST::Multiplication(lhs, rhs) => eval(lhs) * eval(rhs),
    };
}

fn build_ast(expr: &[char]) -> Box<AST> {
    let (lhs, lhs_end) = parse_operand_from_start(expr);
    return parse_operation(lhs, &expr[lhs_end..]);
}

fn parse_operation(lhs: Box<AST>, remainder: &[char]) -> Box<AST> {
    let op_index = skip_spaces(remainder, 0);
    if op_index.is_none() {
        return lhs;
    }
    let rhs_start = skip_spaces(remainder, op_index.unwrap() + 1);
    if rhs_start.is_none() {
        return lhs;
    }
    match remainder[op_index.unwrap()] {
        '+' => {
            let (rhs, rhs_end) = parse_operand_from_start(&remainder[rhs_start.unwrap()..]);
            let add = Box::new(AST::Addition(lhs, rhs));
            return parse_operation(add, &remainder[(rhs_start.unwrap() + rhs_end)..]);
        }
        '*' => Box::new(AST::Multiplication(
            lhs,
            build_ast(&remainder[rhs_start.unwrap()..]),
        )),
        c => panic!(
            "unrecognized op character: '{}' in '{}'",
            c,
            chars_to_str(remainder)
        ),
    }
}

fn parse_operand_from_start(expr: &[char]) -> (Box<AST>, usize) {
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
    return (
        Box::new(match is_subexpr {
            true => AST::Subexpr(build_ast(&str_to_chars(
                expr[1..(end - 1)].iter().collect(),
            ))),
            false => {
                let s: String = expr[0..end].iter().collect();
                AST::Value(s.parse::<i64>().unwrap())
            }
        }),
        end,
    );
}

fn skip_spaces(expr: &[char], index: usize) -> Option<usize> {
    let mut new_index = index;
    loop {
        if new_index >= expr.len() {
            return None;
        }
        if expr[new_index] != ' ' {
            break;
        }
        new_index += 1;
    }
    return Some(new_index);
}

fn chars_to_str(expr: &[char]) -> String {
    expr.iter().collect()
}

fn str_to_chars(s: String) -> Vec<char> {
    s.chars().collect()
}
