use std::collections::HashMap;
use std::io;

pub fn test() {
    println!("import intcode worked")
}

fn indir(memory: &Vec<i32>, ip: usize) -> usize{
    return memory[ip] as usize;
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

//trait StepFunction: Fn(&mut Vec<i32>, usize, i32) -> Option<usize>;

pub fn perform_step(mut memory: &mut Vec<i32>, ip: &mut usize, op_map: &HashMap<i32, Box<dyn Fn(&mut Vec<i32>, usize, i32) -> Option<usize>>>) -> bool {
    let op = memory[*ip] % 100;
    let flags = memory[*ip] / 100;
    //println!("before {} {:?}", op, memory);
    let delta_ip = op_map[&op](&mut memory, *ip, flags);
    //println!("after {:?}", memory);
    if delta_ip.is_none() {
        return false; // break
    }
    *ip += delta_ip.unwrap();
    return true; // continue
}

fn input() -> i32 {
    //let mut line = String::new();
    //let stdin = io::stdin();
    //stdin.lock().read_line(&mut line).expect("Could not read line");
    //println!("{}", line);
    return 1;
}
fn output(value: i32) {
    print!("{}", value);
}

pub fn run_until_halt(mut memory: &mut Vec<i32>){//, op_map: HashMap<i32, Box<dyn Fn(&mut Vec<i32>, usize) -> Option<usize>>>){
    let mut op_map: HashMap<i32, Box<dyn Fn(&mut Vec<i32>, usize, i32) -> Option<usize>>> = HashMap::new();
    op_map.insert(1, Box::new(|mem, ip, flags| {
        let target = mem[ip+3] as usize;
        mem[target] = handle_flags(&mem, ip, 1, flags) + handle_flags(&mem, ip, 2, flags); return Some(4);
    }));
    op_map.insert(2, Box::new(|mem, ip, flags| {
        let target = mem[ip+3] as usize;
        mem[target] = handle_flags(&mem, ip, 1, flags) * handle_flags(&mem, ip, 2, flags); return Some(4);
    }));
    op_map.insert(3, Box::new(|mem, ip, flags| { let target = mem[ip+1] as usize; mem[target] = input(); Some(2) } ));
    op_map.insert(4, Box::new(|mem, ip, flags| { output(handle_flags(&mem, ip, 1, flags)); Some(2) }));
    op_map.insert(99, Box::new(|_, _, _| None ));

    let mut ip = 0;
    while perform_step(&mut memory, &mut ip, &op_map) {
    }
    println!("");
    println!("result: {}", memory[0]);
}
