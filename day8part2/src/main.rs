use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

enum OpCode {
    Acc,
    Jmp,
    Nop,
}

impl OpCode {
    fn is_acc(&self) -> bool {
        match *self {
            OpCode::Acc => true,
            _ => false,
        }
    }
}

impl Copy for OpCode {}

impl Clone for OpCode {
    fn clone(&self) -> Self {
        *self
    }
}

struct Instruction {
    op_code: OpCode,
    offset: i32,
}

impl Copy for Instruction {}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        *self
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: day8part2 <path to input text file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let instructions: Vec<Instruction> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line).expect("failed to parse line"))
        .collect();
    println!("{}", eval_repair(instructions));
}

fn eval_repair(instructions: Vec<Instruction>) -> i32 {
    for (idx, instr) in instructions.iter().enumerate() {
        if instr.op_code.is_acc() {
            continue;
        }
        let mut new_instructions = instructions.clone();
        match instr.op_code {
            OpCode::Nop => new_instructions[idx].op_code = OpCode::Jmp,
            OpCode::Jmp => new_instructions[idx].op_code = OpCode::Nop,
            _ => {}
        }
        let (acc, completed) = eval_until_infinite_loop(new_instructions);
        if completed {
            return acc;
        }
    }
    return 0;
}

fn eval_until_infinite_loop(instructions: Vec<Instruction>) -> (i32, bool) {
    if instructions.is_empty() {
        return (0, true);
    }
    let mut instr_index = 0i32;
    let mut seen_indices = HashSet::new();
    let mut acc = 0i32;
    while !seen_indices.contains(&instr_index) {
        seen_indices.insert(instr_index);
        let instr = &instructions[instr_index as usize];
        match instr.op_code {
            OpCode::Nop => instr_index += 1,
            OpCode::Acc => {
                acc += instr.offset;
                instr_index += 1;
            }
            OpCode::Jmp => instr_index += instr.offset,
        }
        if (instr_index as usize) == instructions.len() {
            return (acc, true);
        }
    }
    return (acc, false);
}

fn parse_line(line: String) -> Result<Instruction, InputError> {
    let root_components: Vec<&str> = line.split_whitespace().collect();
    if root_components.len() != 2 {
        return Err(InputError::InvalidLine(line));
    }
    let offset_str = root_components[1];
    Ok(Instruction {
        op_code: match root_components[0] {
            "nop" => OpCode::Nop,
            "jmp" => OpCode::Jmp,
            "acc" => OpCode::Acc,
            _ => return Err(InputError::InvalidOpCode(line)),
        },
        offset: if offset_str.starts_with("+") {
            offset_str[1..].parse::<i32>().unwrap()
        } else {
            offset_str.parse::<i32>().unwrap()
        },
    })
}

#[derive(Debug)]
enum InputError {
    InvalidLine(String),
    InvalidOpCode(String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InputError::InvalidLine(s) => write!(f, "invalid line: {}", s),
            InputError::InvalidOpCode(s) => write!(f, "invalid opcode: {}", s),
        }
    }
}

impl std::error::Error for InputError {}
