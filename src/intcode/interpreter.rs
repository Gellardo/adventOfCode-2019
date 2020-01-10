use std::collections::HashMap;
use std::convert::TryInto;

use crate::intcode::virtual_memory::VirtMem;

pub struct State {
    mem: VirtMem,
    ip: usize,
    relative_base: usize,
    input: Vec<i32>,
    output: Vec<i32>,
}

fn get_op_map() -> HashMap<i32, Box<dyn Fn(&mut State, i32) -> Option<usize>>> {
    let mut op_map: HashMap<i32, Box<dyn Fn(&mut State, i32) -> Option<usize>>> = HashMap::new();
    op_map.insert(1, Box::new(|state, flags| {
        let target = state.mem[state.ip + 3] as usize;
        state.mem[target] = handle_flags_state(&state, 1, flags) + handle_flags_state(&state, 2, flags);
        return Some(4);
    })); // add
    op_map.insert(2, Box::new(|state, flags| {
        let target = state.mem[state.ip + 3] as usize;
        state.mem[target] = handle_flags_state(&state, 1, flags) * handle_flags_state(&state, 2, flags);
        return Some(4);
    })); // mult
    op_map.insert(3, Box::new(|state, _| {
        let target = state.mem[state.ip + 1] as usize;
        state.mem[target] = input(state);
        Some(2)
    })); //input
    op_map.insert(4, Box::new(|state, flags| {
        output(state, handle_flags_state(&state, 1, flags));
        Some(2)
    })); // output
    op_map.insert(5, Box::new(|state, flags| {
        if handle_flags_state(&state, 1, flags) != 0 {
            state.ip = handle_flags_state(&state, 2, flags).try_into().unwrap();
            Some(0)
        } else { Some(3) }
    })); //jump-true
    op_map.insert(6, Box::new(|state, flags| {
        if handle_flags_state(&state, 1, flags) == 0 {
            state.ip = handle_flags_state(&state, 2, flags).try_into().unwrap();
            Some(0)
        } else { Some(3) }
    })); //jump-false
    op_map.insert(7, Box::new(|state, flags| {
        let target = state.mem[state.ip + 3] as usize;
        state.mem[target] = if handle_flags_state(&state, 1, flags) < handle_flags_state(&state, 2, flags) { 1 } else { 0 };
        Some(4)
    })); // less than
    op_map.insert(8, Box::new(|state, flags| {
        let target = state.mem[state.ip + 3] as usize;
        state.mem[target] = if handle_flags_state(&state, 1, flags) == handle_flags_state(&state, 2, flags) { 1 } else { 0 };
        Some(4)
    })); // eq
    op_map.insert(9, Box::new(|state, flags| {
        state.relative_base = state.relative_base + handle_flags_state(&state, 1, flags) as usize;
        Some(2)
    })); // set base pointer
    op_map.insert(99, Box::new(|_, _| None));
    return op_map;
}

fn handle_flags_state(state: &State, pos: u32, flags: i32) -> i32 {
    return handle_flags(&state.mem, state.ip, state.relative_base, pos, flags);
}

fn handle_flags(memory: &VirtMem, ip: usize, base: usize, pos: u32, flags: i32) -> i32 {
    let positional = flags / 10i32.pow(pos - 1) % 10;
    if positional == 0 {
        return memory[memory[ip + pos as usize] as usize];
    } else if positional == 1 {
        // immediate
        return memory[ip + pos as usize];
    } else if positional == 2 {
        // relative to base
        return memory[base + pos as usize];
    } else {
        panic!("flag not implemented: from memory[{}] with flag {}", ip + pos as usize, positional);
    }
}

fn input(state: &mut State) -> i32 {
    //let mut line = String::new();
    //let stdin = io::stdin();
    //stdin.lock().read_line(&mut line).expect("Could not read line");
    //println!("{}", line);
    return state.input.pop().unwrap();
}

fn output(state: &mut State, value: i32) {
    trace!("output: {}", value);
    state.output.push(value);
}

//trait StepFunction: Fn(&mut Vec<i32>, usize, i32) -> Option<usize>;

fn perform_step(state: &mut State, op_map: &HashMap<i32, Box<dyn Fn(&mut State, i32) -> Option<usize>>>) -> bool {
    let op = state.mem[state.ip] % 100;
    let flags = state.mem[state.ip] / 100;
//    println!("before {},{} {:?}", op, flags, state.mem);
    trace!("perform op @{}: {}", state.ip, op);
    let op = op_map.get(&op).expect(&("Unknown op ".to_owned() + &op.to_string()));
    let delta_ip = op(state, flags);
    if delta_ip.is_none() {
        return false; // break
    }
    trace!("used up {} of [{},{},{},{}]", delta_ip.unwrap(), state.mem[state.ip],
           state.mem[state.ip + 1], state.mem[state.ip + 2], state.mem[state.ip + 3]);
    state.ip += delta_ip.unwrap();
    return true; // continue
}

fn run_until_halt(memory: Vec<i32>, mut input_buffer: Vec<i32>) -> State {
    input_buffer.reverse();
    let mut state = State {
        mem: VirtMem::from(memory),
        ip: 0,
        relative_base: 0,
        input: input_buffer,
        output: Vec::new(),
    };
    let op_map = get_op_map();

    while perform_step(&mut state, &op_map) {}
    return state;
}

pub fn run(memory: Vec<i32>, input_buffer: Vec<i32>) -> Vec<i32> {
    let state = run_until_halt(memory, input_buffer);
    return state.output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_handling() {
        let input = VirtMem::from(vec![-1, 0, 3, 2, 4, 8, 9, 10]);
        assert_eq!(handle_flags(&input, 1, 0, 1, 10), 2);
        assert_eq!(handle_flags(&input, 1, 0, 2, 10), 2);
        assert_eq!(handle_flags(&input, 1, 0, 2, 20), 3);
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
            let state = run_until_halt(inp.clone(), vec![8]);
            assert_eq!(state.output[0], 1);
            let state = run_until_halt(inp, vec![2]);
            assert_eq!(state.output[0], 0);
        }
        for inp in inputs_lt {
            let mut state = run_until_halt(inp.clone(), vec![2]);
            assert_eq!(state.output[0], 1);
            state = run_until_halt(inp, vec![8]);
            assert_eq!(state.output[0], 0);
        }
    }
//    #[test]
//    fn day9_tests() {
//        let inputs = vec![vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99], ];
//        assert_eq!(run_until_halt(inputs[0].clone(), vec![]).output, inputs[0]);
//    }

    #[test]
    fn jumps() {
        let inputs = vec![
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        ];
        for input in inputs {
            let state = run_until_halt(input.clone(), vec![42]);
            assert_eq!(state.output[0], 1);
            let state = run_until_halt(input, vec![0]);
            assert_eq!(state.output[0], 0);
        }
    }
}
