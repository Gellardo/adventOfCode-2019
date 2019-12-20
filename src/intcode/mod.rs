mod virtual_memory;
mod interpreter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jumps() {
        let inputs = vec![
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        ];
        for input in inputs {
            let output = interpreter::run(input.clone(), vec![42]);
            assert_eq!(output[0], 1);
            let output = interpreter::run(input, vec![0]);
            assert_eq!(output[0], 0);
        }
    }

    #[test]
    fn comparisons() {
        let inputs_eq = vec![
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
        ];
        let inputs_lt = vec![
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
        ];
        for inp in inputs_eq {
            let output = interpreter::run(inp.clone(), vec![8]);
            assert_eq!(output[0], 1);
            let output = interpreter::run(inp, vec![2]);
            assert_eq!(output[0], 0);
        }
        for inp in inputs_lt {
            let mut output = interpreter::run(inp.clone(), vec![2]);
            assert_eq!(output[0], 1);
            output = interpreter::run(inp, vec![8]);
            assert_eq!(output[0], 0);
        }
    }
////    #[test]
////    fn day9_tests() {
////        let inputs = vec![vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99], ];
////        assert_eq!(run_until_halt(inputs[0].clone(), vec![]).output, inputs[0]);
////    }
}
