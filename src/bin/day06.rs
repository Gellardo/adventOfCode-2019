use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

/**
 * As hinted in the exercise, the input forms a tree.
 * Each objects orbits every object on the path to the root of that tree (COM).
 * So we just walk the edges backwards until there are no more options and count the steps.
 * Use basic caching to improve the runtime.
 */
fn find_root_caching(edges: &Vec<Vec<String>>, val: &String, known_dist: &HashMap<String, i32>) -> i32 {
    let mut steps = 0;
    let mut current = val;
    {
        let mut last;
        loop {
            last = steps;
            if known_dist.contains_key(current) {
                let known = known_dist.get(current).unwrap();
                return known + steps;
            }
            for edge in edges {
                if edge[1] == *current {
                    current = &edge[0];
                    steps += 1;
                }
            }
            if last == steps { break; }
        }
    }
//    println!("{:?}", (val, steps));
    steps
}

/**
 * Walk from both points to the source, until we find an intersection.
 * Since we work on a tree, this is the shortest path
 */
fn find_min_transits(edges: &Vec<Vec<String>>, from: String, to: String) -> i32 {
    let mut known_transits: HashMap<String, i32> = HashMap::new();
    let mut steps = 0;
    let mut current = &from;
    {
        let mut last;
        loop {
            last = steps;
            for edge in edges {
                if edge[1] == *current {
                    current = &edge[0];
                    steps += 1;
                    known_transits.insert(current.clone(), steps);
                }
            }
            if last == steps { break; }
        }
    }
    steps = 0;
    current = &to;
    {
        let mut last;
        loop {
            last = steps;
            for edge in edges {
                if edge[1] == *current {
                    current = &edge[0];
                    steps += 1;
                    break;// have to break to check if we found an intersection
                }
            }
            if last == steps { break; }
            if known_transits.contains_key(current) { break; }
        }
    }
    known_transits[current] + steps - 2
}

fn count_all_steps_to_root(edges: &Vec<Vec<String>>) -> (i32, i32) {
    let nodes: HashSet<String> = edges.iter()
        .filter(|x| x.len() > 1)
        .flat_map(|x| x.iter().map(|y| y.to_string()))
        .collect();
    let mut steps = 0;
    let num_nodes = nodes.len();
    let mut known_distances: HashMap<String, i32> = HashMap::new();
    for node in nodes {
        let node_steps = find_root_caching(&edges, &node, &mut known_distances);
        known_distances.insert(node, node_steps);
        steps += node_steps;
    }
    (steps, steps - num_nodes as i32)
}

fn process_input(input: String) -> Vec<Vec<String>> {
    input.split("\n")
        .filter(|x| x.len() > 1)
        .map(
            |x| x.split(")").map(|y| y.to_string()).collect()
        ).collect()
}

fn main() {
    let line = fs::read_to_string("inputs/day06").unwrap();
    let edges: Vec<Vec<String>> = process_input(line);

    println!("{:?}", count_all_steps_to_root(&edges));
    println!("day 6 part 1: done");

    println!("{:?}", find_min_transits(&edges, "SAN".to_string(), "YOU".to_string()));
    println!("day 6 part 2: done");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_root_test() {
        let edges = process_input(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L".to_string()
        );
        assert_eq!(find_root_caching(&edges, &"L".to_string(), &HashMap::new()), 7);
        assert_eq!(count_all_steps_to_root(&edges).0, 42);
    }

    #[test]
    fn find_min_transit() {
        let edges = process_input(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN".to_string()
        );
        assert_eq!(find_min_transits(&edges, "YOU".to_string(), "SAN".to_string()), 4);
    }
}
