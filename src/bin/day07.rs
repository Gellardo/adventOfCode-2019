extern crate advent_of_code;

use std::fs;
use std::ops::RangeInclusive;

use advent_of_code::intcode;

fn run_amplifier(mem: Vec<i32>, phase_setting: i32, mut input: Vec<i32>) -> Vec<i32> {
    input.insert(0, phase_setting);
    let output = intcode::run(mem, input);
    return output;
}

fn run_amplifiers(mem: Vec<i32>, phase_settings: Vec<i32>, looping: bool) -> i32 {
    if !looping {
        let mut input = vec![0];
        for phase_setting in phase_settings {
            input = run_amplifier(mem.clone(), phase_setting, input);
        }
        return input[0];
    }
    0 // not implemented
}

fn max_thrust(mem: Vec<i32>) -> (Vec<i32>, i32) {
    max_thrust_range(mem, 0..=4, false)
}

fn max_thrust_range(mem: Vec<i32>, range: RangeInclusive<i32>, looping: bool) -> (Vec<i32>, i32) {
    let mut max = 0;
    let mut input = vec![0, 0, 0, 0, 0];
    let range = 0..=4;
    for i in range.clone() {
        let mut phase_settings = vec![i];
        for j in range.clone() {
            if phase_settings.contains(&j) { continue; };
            phase_settings.push(j);
            for k in range.clone() {
                if phase_settings.contains(&k) { continue; };
                phase_settings.push(k);
                for l in range.clone() {
                    if phase_settings.contains(&l) { continue; };
                    phase_settings.push(l);
                    for m in range.clone() {
                        if phase_settings.contains(&m) { continue; };
                        phase_settings.push(m);
                        let output = run_amplifiers(mem.clone(), phase_settings.clone(), looping);
                        if output > max {
                            max = output;
                            input = phase_settings.clone();
                        }
                        phase_settings.pop();
                    }
                    phase_settings.pop();
                }
                phase_settings.pop();
            }
            phase_settings.pop();
        }
        phase_settings.pop();
    }
    return (input, max);
}

fn main() {
    let memory = intcode::read_program("inputs/day07".to_string());

    println!("{:?}", max_thrust(memory));
    println!("day 7 part 1: done");

    println!("day 7 part 2: done");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let mem = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
        assert_eq!(run_amplifiers(mem.clone(), vec![4, 3, 2, 1, 0], false), 43210);

        assert_eq!(max_thrust(mem).1, 43210);
    }

//    #[test]
//    fn run_feedback_loop() {
//        let mem = vec![3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27,
//                       4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5];
//
//        let mem = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
//        assert_eq!(run_amplifiers(mem.clone(), vec![9, 8, 7, 6, 5], true), 139629729);
//    }
}
