extern crate env_logger;
#[macro_use]
extern crate log;

use std::fs;

use env_logger::Env;

/// Returns a Vec of (x, y) tuples, one for each astroid/'#'
fn scan_field_for_astroids(scan: Vec<String>) -> Vec<(usize, usize)> {
    let mut asteroids = vec![];
    for i in 0..scan.len() {
        let line_b = scan[i].as_bytes();
        for j in 0..line_b.len() {
            if line_b[j] == '#' as u8 {
                asteroids.push((i, j))
            }
        }
    }
    asteroids
}

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let lines = fs::read_to_string("inputs/day10").unwrap();
    let rows: Vec<String> = lines.split("\n").map(|x| { x.to_string() }).collect();
//    println!("{}",lines);
//    println!("{:?}",rows);
    let astroids = scan_field_for_astroids(rows);
    let out1 = astroids;
    info!("day 10 part 1: {:?}", out1);

    let out2 = "1";
    info!("day 10 part 2: {:?}", out2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_astroids() {
        let input = "#..\n.#.\n.##".split("\n").map(String::from).collect();
        assert_eq!(scan_field_for_astroids(input), vec![(0, 0), (1, 1), (2, 1), (2, 2)]);
    }
}
