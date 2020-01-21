use std::cmp::min;
use std::collections::HashSet;
use std::io;

fn step(p: (i64, i64), s: String) -> (i64, i64) {
    if s == "U" {
        return (p.0, p.1 + 1);
    }
    if s == "D" {
        return (p.0, p.1 - 1);
    }
    if s == "L" {
        return (p.0 - 1, p.1);
    }
    if s == "R" {
        return (p.0 + 1, p.1);
    }
    return (0, 0);
}

fn walk_wire(wire: &String) -> HashSet<(i64, i64)> {
    let line = wire.trim().split(",");
    let mut current = (0, 0);
    let mut points = HashSet::new();
    for op in line {
        //print!("{:?}",&op[..1]);
        for _ in 0..op[1..].parse().unwrap() {
            //print!(".");
            current = step(current, op[..1].to_string());
            points.insert(current);
        }
        //println!("| {:?}", current);
    }
    return points;
}

fn find_distance_closest_intersection(wire1: &String, wire2: &String) -> i64 {
    let p1 = walk_wire(wire1);
    let p2 = walk_wire(wire2);
    let intersections = p1.intersection(&p2);
    let mut min_distance = i64::max_value();
    for (x, y) in intersections {
        let distance = x.abs() + y.abs();
        min_distance = min(min_distance, distance)
    }
    return min_distance;
}

fn find_steps_closest_interseciont(_wire1: &String, _wire2: &String) -> i64 {
    // TODO implement
    return 0;
}

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} byte line: {}", n, input);
        }
        Err(e) => println!("error: {}", e),
    }
    return input;
}

fn main() {
    println!("Hello, day3!");

    let wire1 = read_line();
    let wire2 = read_line();
    println!("day 3 part 1: {}", find_distance_closest_intersection(&wire1, &wire2));

    println!("day 3 part 2: {}", find_steps_closest_interseciont(&wire1, &wire2));
}
