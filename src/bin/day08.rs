use std::fs;

fn checksum(line: &String, size: usize) -> (i64, i64) {
    let line_b = line.as_bytes();
    let mut image: [char; 25*6] = ['8'; 25*6];
    let mut min_z = 2000000000;
    let mut mult_one_two = 0;
    let mut i = 0;
    loop {
        if i >= line.len() - size {break;}
        let mut zs = 0;
        let mut os = 0;
        let mut ts = 0;
        for j in 0..size {
            if line_b[i+j] == ('0' as u8) {zs +=1;}
            if line_b[i+j] == ('1' as u8) {os +=1;}
            if line_b[i+j] == ('2' as u8) {ts +=1;}
            if image[j] == '8' || image[j] == '2' {
                image[j] = line_b[i+j] as char
            }
        }
        if zs < min_z {
            min_z = zs;
            mult_one_two = os * ts;
        }
        i+=size
    }
    for i in 0..6 {
        println!("{:?}", &image[i*25..i*25+25]);
    }
    (min_z, mult_one_two)
}

fn main() {
    let line = fs::read_to_string("inputs/day08").unwrap();
    let size = 25*6;

    let (min_z, mult_one_two) = checksum(&line, size);

    println!("min:{}, mult:{}", min_z, mult_one_two);
    println!("day 8 part 1: done");

    println!("day 8 part 2: done");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_root_test() {
    }

    #[test]
    fn find_min_transit() {
    }
}
