use std::collections::HashMap;

pub struct IntCode {
    mem: Vec<i32>,
    ip: usize,
    input: Vec<i32>,
}

fn handle_flags(memory: &Vec<i32>, ip: usize, pos: u32, flags:i32) -> i32 {
    let positional = flags/10i32.pow(pos-1) % 10;
    if positional == 0{
        return memory[memory[ip+pos as usize] as usize];
    } else {
        //immediate
        return memory[ip+pos as usize];
    }
}

fn input(state: &mut IntCode) -> i32 {
    //let mut line = String::new();
    //let stdin = io::stdin();
    //stdin.lock().read_line(&mut line).expect("Could not read line");
    //println!("{}", line);
    return state.input.pop().unwrap()
}
fn output(value: i32) {
    print!("{}", value);
}

//trait StepFunction: Fn(&mut Vec<i32>, usize, i32) -> Option<usize>;

pub fn perform_step(state: &mut IntCode, op_map: &HashMap<i32, Box<dyn Fn(&mut IntCode, i32) -> Option<usize>>>) -> bool {
    let op = state.mem[state.ip] % 100;
    let flags = state.mem[state.ip] / 100;
    //println!("before {} {:?}", op, memory);
    let delta_ip = op_map[&op](state, flags);
    //println!("after {:?}", memory);
    if delta_ip.is_none() {
        return false; // break
    }
    state.ip += delta_ip.unwrap();
    return true; // continue
}

pub fn run_until_halt(memory: Vec<i32>, input_buffer: Vec<i32>){
    let mut state = IntCode{mem: memory, ip: 0, input: input_buffer};
    let mut op_map: HashMap<i32, Box<dyn Fn(&mut IntCode, i32) -> Option<usize>>> = HashMap::new();
    op_map.insert(1, Box::new(|state, flags| {
        let target = state.mem[state.ip+3] as usize;
        state.mem[target] = handle_flags(&state.mem, state.ip, 1, flags) + handle_flags(&state.mem, state.ip, 2, flags); return Some(4);
    }));
    op_map.insert(2, Box::new(|state, flags| {
        let target = state.mem[state.ip+3] as usize;
        state.mem[target] = handle_flags(&state.mem, state.ip, 1, flags) * handle_flags(&state.mem, state.ip, 2, flags); return Some(4);
    }));
    op_map.insert(3, Box::new(|state, flags| { let target = state.mem[state.ip+1] as usize; state.mem[target] = input(state); Some(2) } ));
    op_map.insert(4, Box::new(|state, flags| { output(handle_flags(&state.mem, state.ip, 1, flags)); Some(2) }));
    op_map.insert(99, Box::new(|_, _| None ));

    while perform_step(&mut state, &op_map) {
    }
    println!("");
    println!("result: {}", state.mem[0]);
}
