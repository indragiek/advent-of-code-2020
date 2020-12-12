use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), InputError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(InputError::InvalidArguments);
    }
    let filename = &args[1];
    let file = File::open(filename).expect("failed to open file");
    let instrs: Vec<Instruction> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line).unwrap())
        .collect();
    let mut pos = ShipPosition {
        direction: Direction::East,
        east: 0,
        north: 0,
    };
    for instr in instrs {
        pos = evaluate_instruction(pos, instr);
    }
    println!("{}", pos.east.abs() + pos.north.abs());
    Ok(())
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Clone, Copy)]
struct Instruction {
    action: Action,
    distance: i32,
}

#[derive(Clone, Copy)]
struct ShipPosition {
    direction: Direction,
    east: i32,
    north: i32,
}

fn evaluate_instruction(pos: ShipPosition, instr: Instruction) -> ShipPosition {
    return ShipPosition {
        direction: match instr.action {
            Action::Left => turn(pos.direction, true, instr.distance / 90),
            Action::Right => turn(pos.direction, false, instr.distance / 90),
            _ => pos.direction,
        },
        east: match (pos.direction, instr.action) {
            (Direction::East, Action::Forward) => pos.east + instr.distance,
            (Direction::West, Action::Forward) => pos.east - instr.distance,
            (_, Action::East) => pos.east + instr.distance,
            (_, Action::West) => pos.east - instr.distance,
            _ => pos.east,
        },
        north: match (pos.direction, instr.action) {
            (Direction::North, Action::Forward) => pos.north + instr.distance,
            (Direction::South, Action::Forward) => pos.north - instr.distance,
            (_, Action::North) => pos.north + instr.distance,
            (_, Action::South) => pos.north - instr.distance,
            _ => pos.north,
        },
    };
}

fn turn(direction: Direction, left: bool, times: i32) -> Direction {
    let mut new_direction = direction;
    for _ in 0..times {
        new_direction = match (new_direction, left) {
            (Direction::North, true) => Direction::West,
            (Direction::South, true) => Direction::East,
            (Direction::West, true) => Direction::South,
            (Direction::East, true) => Direction::North,
            (Direction::North, false) => Direction::East,
            (Direction::South, false) => Direction::West,
            (Direction::West, false) => Direction::North,
            (Direction::East, false) => Direction::South,
        }
    }
    return new_direction;
}

fn parse_line(line: String) -> Result<Instruction, InputError> {
    if line.is_empty() {
        return Err(InputError::InvalidLine(line));
    }
    Ok(Instruction {
        action: match line.chars().nth(0).unwrap() {
            'N' => Action::North,
            'S' => Action::South,
            'E' => Action::East,
            'W' => Action::West,
            'L' => Action::Left,
            'R' => Action::Right,
            'F' => Action::Forward,
            c => return Err(InputError::InvalidChar(c)),
        },
        distance: line[1..].parse::<i32>().unwrap(),
    })
}

#[derive(Debug)]
enum InputError {
    InvalidArguments,
    InvalidChar(char),
    InvalidLine(String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InputError::InvalidArguments => {
                write!(f, "usage: day12part1 <path to input text file>")
            }
            InputError::InvalidChar(c) => write!(f, "invalid char: {}", c),
            InputError::InvalidLine(s) => write!(f, "invalid line: {}", s),
        }
    }
}

impl std::error::Error for InputError {}
