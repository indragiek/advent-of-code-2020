use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

type Coordinate = (isize, isize, isize);
type ConwayCubeGrid = HashSet<Coordinate>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let mut grid: ConwayCubeGrid = HashSet::new();

    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .for_each(|(x, _)| {
                    grid.insert((x as isize, y as isize, 0));
                });
        });

    for _ in 0..6 {
        let mut new_grid = grid.clone();
        for x in (grid.iter().map(|(x, _, _)| x).min().unwrap() - 1)
            ..=(grid.iter().map(|(x, _, _)| x).max().unwrap() + 1)
        {
            for y in (grid.iter().map(|(_, y, _)| y).min().unwrap() - 1)
                ..=(grid.iter().map(|(_, y, _)| y).max().unwrap() + 1)
            {
                for z in (grid.iter().map(|(_, _, z)| z).min().unwrap() - 1)
                    ..=(grid.iter().map(|(_, _, z)| z).max().unwrap() + 1)
                {
                    let coord = (x, y, z);
                    match (grid.contains(&coord), num_active_neighbors(&grid, coord)) {
                        (true, 3) => {}
                        (true, 4) => {}
                        (true, _) => {
                            let _ = new_grid.remove(&coord);
                        }
                        (false, 3) => {
                            let _ = new_grid.insert(coord);
                        }
                        (false, _) => {}
                    }
                }
            }
        }
        grid = new_grid;
    }

    println!("{}", grid.len());
}

fn num_active_neighbors(grid: &ConwayCubeGrid, coord: Coordinate) -> usize {
    let mut count = 0usize;
    for &x in &[coord.0 - 1, coord.0, coord.0 + 1] {
        for &y in &[coord.1 - 1, coord.1, coord.1 + 1] {
            for &z in &[coord.2 - 1, coord.2, coord.2 + 1] {
                if grid.contains(&(x, y, z)) {
                    count += 1;
                }
            }
        }
    }
    return count;
}
