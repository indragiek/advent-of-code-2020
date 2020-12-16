use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let start_numbers: Vec<u32> = fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let mut age: HashMap<u32, usize> = start_numbers
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i))
        .collect();
    let mut was_first_time = true;
    let mut previous_num = *start_numbers.last().unwrap();
    for i in start_numbers.len()..2020 {
        let next_num = match was_first_time {
            true => 0,
            false => i - 1 - age.get(&previous_num).unwrap(),
        } as u32;
        age.insert(previous_num, i - 1);
        previous_num = next_num;
        was_first_time = !age.contains_key(&next_num);
    }
    println!("{}", previous_num);
}
