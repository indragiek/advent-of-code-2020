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
    let mut waypoint_pos = Position {
        east: 10.0,
        north: 1.0,
    };
    let mut ship_pos = Position {
        east: 0.0,
        north: 0.0,
    };
    for instr in instrs {
        let result = evaluate_instruction(waypoint_pos, ship_pos, instr);
        waypoint_pos = result.0;
        ship_pos = result.1;
    }
    println!("{}", (ship_pos.east.abs() + ship_pos.north.abs()).round());
    Ok(())
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
    value: f32,
}

#[derive(Clone, Copy)]
struct Position {
    east: f32,
    north: f32,
}

fn evaluate_instruction(
    waypoint_pos: Position,
    ship_pos: Position,
    instr: Instruction,
) -> (Position, Position) {
    let new_waypoint_pos = Position {
        east: match instr.action {
            Action::East => waypoint_pos.east + instr.value,
            Action::West => waypoint_pos.east - instr.value,
            Action::Left => {
                waypoint_pos.north * (-instr.value.to_radians()).sin()
                    + waypoint_pos.east * (-instr.value.to_radians()).cos()
            }
            Action::Right => {
                waypoint_pos.north * instr.value.to_radians().sin()
                    + waypoint_pos.east * instr.value.to_radians().cos()
            }
            _ => waypoint_pos.east,
        },
        north: match instr.action {
            Action::North => waypoint_pos.north + instr.value,
            Action::South => waypoint_pos.north - instr.value,
            Action::Left => {
                waypoint_pos.north * (-instr.value.to_radians()).cos()
                    - waypoint_pos.east * (-instr.value.to_radians()).sin()
            }
            Action::Right => {
                waypoint_pos.north * instr.value.to_radians().cos()
                    - waypoint_pos.east * instr.value.to_radians().sin()
            }
            _ => waypoint_pos.north,
        },
    };
    let new_ship_pos = Position {
        east: match instr.action {
            Action::Forward => ship_pos.east + (waypoint_pos.east * instr.value),
            _ => ship_pos.east,
        },
        north: match instr.action {
            Action::Forward => ship_pos.north + (waypoint_pos.north * instr.value),
            _ => ship_pos.north,
        },
    };
    return (new_waypoint_pos, new_ship_pos);
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
        value: line[1..].parse::<f32>().unwrap(),
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
                write!(f, "usage: day12part2 <path to input text file>")
            }
            InputError::InvalidChar(c) => write!(f, "invalid char: {}", c),
            InputError::InvalidLine(s) => write!(f, "invalid line: {}", s),
        }
    }
}

impl std::error::Error for InputError {}
