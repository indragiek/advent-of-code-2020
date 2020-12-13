use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let pairs: Vec<(i64, i64)> = lines[1]
        .split(",")
        .enumerate()
        .filter(|&(_, id)| id != "x")
        .map(|(idx, id)| (idx as i64, id.parse::<i64>().unwrap()))
        .collect();
    print!("{}", garners_algorithm(pairs));
}

// m -> number of minutes after
// t -> timestamp
// b -> bus ID
//
// From the part 1 solution we know that the equation for calculating
// the # of minutes after looks like:
//
// m_{i} = b_{i} - (t % b_{i})
//
// By rearranging that, we get the equation:
//
// b_{i} - m_{i} = t % b_{i}
//
// This is in the form a_{i} = a % p_{i} where a_{i} = b_{i} - m{i},
// a = t, and p_{i} = b{i}, which can be solved using the Chinese
// Remainder Theorem and Garner's Algorithm as described here:
// https://cp-algorithms.com/algebra/chinese-remainder-theorem.html
//
// Each pair in the vector passed to this function is an (index, bus ID)
// pair where the index is m_{i} in the above equations, since each bus ID
// in the list is supposed to come 1 minute after the previous bus ID in
// the list.
fn garners_algorithm(pairs: Vec<(i64, i64)>) -> i64 {
    let k = pairs.len();
    let mut x = vec![0; k];
    for i in 0..k {
        x[i] = pairs[i].1 - pairs[i].0;
        for j in 0..i {
            x[i] = mod_inv(pairs[j].1, pairs[i].1) * (x[i] - x[j]);
            x[i] = x[i] % pairs[i].1;
            if x[i] < 0 {
                x[i] += pairs[i].1;
            }
        }
    }

    let mut ts = 0i64;
    for i in 0..k {
        let mut product = x[i];
        for j in 0..i {
            product *= pairs[j].1;
        }
        ts += product;
    }

    return ts;
}

// https://rosettacode.org/wiki/Modular_inverse#Rust
fn mod_inv(a: i64, module: i64) -> i64 {
    let mut mn = (module, a);
    let mut xy = (0, 1);
    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }
    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}
