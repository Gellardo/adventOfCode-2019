extern crate advent_of_code;

use advent_of_code::intcode;
use std::fs;

fn main() {
    let line = fs::read_to_string("day05").unwrap();
    let memory: Vec<i32> = line.split(",").map(|x| x.trim()).map(|x| x.parse().unwrap()).collect();
    intcode::run_until_halt(memory, vec![1]);
}
