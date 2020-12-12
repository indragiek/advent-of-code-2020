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
    let mut map: Vec<Vec<Seat>> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(line).unwrap())
        .collect();
    loop {
        let new_map = simulate(&map);
        if new_map == map {
            break;
        }
        map = new_map;
    }
    println!(
        "{}",
        map.iter()
            .flatten()
            .filter(|&seat| seat == &Seat::OccupiedSeat)
            .count()
    );
    Ok(())
}

#[derive(PartialEq, Clone, Copy)]
enum Seat {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn parse_line(line: String) -> Result<Vec<Seat>, InputError> {
    let mut layout = Vec::new();
    for c in line.chars() {
        match c {
            '.' => layout.push(Seat::Floor),
            'L' => layout.push(Seat::EmptySeat),
            '#' => layout.push(Seat::OccupiedSeat),
            _ => return Err(InputError::InvalidCharacter(c)),
        }
    }
    Ok(layout)
}

fn simulate(map: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    let mut new_map = map.clone();
    for y in 0..map.len() {
        let row = &map[y];
        for x in 0..row.len() {
            match &row[x] {
                Seat::EmptySeat => {
                    if count_all_occupied_adjacent(map, x, y) == 0 {
                        new_map[y][x] = Seat::OccupiedSeat;
                    }
                }
                Seat::OccupiedSeat => {
                    if count_all_occupied_adjacent(map, x, y) >= 5 {
                        new_map[y][x] = Seat::EmptySeat;
                    }
                }
                Seat::Floor => {}
            }
        }
    }
    return new_map;
}

fn seat_position(map: &Vec<Vec<Seat>>, x: i32, y: i32) -> Option<Seat> {
    if x < 0 || y < 0 || y as usize >= map.len() {
        return None;
    }
    let row = &map[y as usize];
    if x as usize >= row.len() {
        return None;
    }
    return Some(row[x as usize]);
}

fn count_occupied_adjacent(
    map: &Vec<Vec<Seat>>,
    x: usize,
    y: usize,
    unit_offset: (i32, i32),
) -> usize {
    let mut multiplier = 1i32;
    let mut count = 0usize;
    loop {
        let mult_offset = (unit_offset.0 * multiplier, unit_offset.1 * multiplier);
        match seat_position(map, x as i32 + mult_offset.0, y as i32 + mult_offset.1) {
            None => break,
            Some(Seat::EmptySeat) => break,
            Some(Seat::OccupiedSeat) => {
                count += 1;
                break;
            }
            Some(Seat::Floor) => {}
        }
        multiplier += 1;
    }
    return count;
}

fn count_all_occupied_adjacent(map: &Vec<Vec<Seat>>, x: usize, y: usize) -> usize {
    return [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .map(|&delta_offset| count_occupied_adjacent(map, x, y, delta_offset))
    .sum();
}

#[derive(Debug)]
enum InputError {
    InvalidArguments,
    InvalidCharacter(char),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InputError::InvalidArguments => {
                write!(f, "usage: day11part2 <path to input text file>")
            }
            InputError::InvalidCharacter(c) => write!(f, "invalid character: {}", c),
        }
    }
}

impl std::error::Error for InputError {}
