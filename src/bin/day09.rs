extern crate env_logger;
#[macro_use]
extern crate log;

use env_logger::Env;

use advent_of_code::intcode;

fn run_program(input: i64) -> Vec<i64> {
    let memory = intcode::read_program("inputs/day09".to_string());
    info!("run the program");
    intcode::run(memory.clone(), vec![input])
}

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let out1 = run_program(1);
    info!("day 9 part 1: {:?}", out1);

    let out2 = run_program(2);
    info!("day 9 part 2: {:?}", out2);

    info!("finished")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn log_test() {
        init(); // have to setup logging first

        let inputs = vec![vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99], ];
        assert_eq!(intcode::run(inputs[0].clone(), vec![]), inputs[0]);
        info!("test done")
    }

    #[test]
    fn can_handle_big_numbers() {
        let inputs = vec![vec![104, 1125899906842624, 99], vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], ];
        for i in inputs {
            intcode::run(i, vec![]);
        }
    }

    #[test]
    fn reuse_testing_from_input() {
        // the challenge tested a bunch of opcodes with different parameter modes
        // if it outputs any number before the expected value, it is a faulty opcode
        assert_eq!(run_program(1), vec![2682107844]);
    }
}
