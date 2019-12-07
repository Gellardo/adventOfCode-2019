extern crate advent_of_code;

use std::fs;

use advent_of_code::intcode;

fn main() {
    let memory = intcode::read_program("inputs/day05".to_string());
    intcode::run(memory.clone(), vec![1]);
    println!("day 5 part 1: done");

    intcode::run(memory, vec![5]);
    println!("day 5 part 2: done");
}
