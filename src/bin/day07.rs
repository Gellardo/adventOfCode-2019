extern crate advent_of_code;

use std::fs;

use advent_of_code::intcode;

fn run_amplifier(mem: Vec<i32>, phase_setting: i32, input: i32) -> i32 {
    let output = intcode::run(mem, vec![phase_setting, input]);
    println!("output {:?}", output);
    return output[0];
}

fn run_amplifiers(mem: Vec<i32>, phase_settings: Vec<i32>) -> i32 {
    let mut input = 0;
    for phase_setting in phase_settings {
        input = run_amplifier(mem.clone(), phase_setting, input);
        println!("intermediate output {}", input);
    }
    return input;
}

fn max_thrust(mem: Vec<i32>) -> (Vec<i32>, i32) {
    let mut max = 0;
    let mut input = vec![0, 0, 0, 0, 0];
    for i in 0..=4 {
        let mut phase_settings = vec![i];
        for j in 0..=4 {
            if phase_settings.contains(&j) { continue; };
            phase_settings.push(j);
            for k in 0..=4 {
                if phase_settings.contains(&k) { continue; };
                phase_settings.push(k);
                for l in 0..=4 {
                    if phase_settings.contains(&l) { continue; };
                    phase_settings.push(l);
                    for m in 0..=4 {
                        if phase_settings.contains(&m) { continue; };
                        phase_settings.push(m);
                        let output = run_amplifiers(mem.clone(), phase_settings.clone());
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
    let line = fs::read_to_string("inputs/day07").unwrap();
    let memory: Vec<i32> = line.split(",").map(|x| x.trim()).map(|x| x.parse().unwrap()).collect();


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
        assert_eq!(run_amplifiers(mem.clone(), vec![4, 3, 2, 1, 0]), 43210);

        assert_eq!(max_thrust(mem).1, 43210);
    }
}
