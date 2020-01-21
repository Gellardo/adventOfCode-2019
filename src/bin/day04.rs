fn num2vec(mut num: i64) -> Vec<i64> {
    let mut v = Vec::new();
    while num > 0 {
        v.push(num % 10);
        num = num / 10;
    }
    v.reverse();
    return v;
}

fn vec2num(vec: &Vec<i64>) -> i64 {
    let mut num = 0;
    for d in vec {
        num *= 10;
        num += d;
    }
    return num;
}

fn iterate_possible_passwords<F>(num_digits: i64, digits: &mut Vec<i64>, correct: &F) -> i64 where F: Fn(Vec<i64>) -> bool {
    if num_digits <= 0 {
        if correct(digits.to_vec()) {
            println!("{:?}", vec2num(digits));
            return 1;
        }
        return 0;
    }
    let mut sum = 0;
    for i in 1..10 {
        digits.push(i);
        sum += iterate_possible_passwords(num_digits - 1, digits, correct);
        digits.pop();
    }
    return sum;
}

fn correct_1(digits: &Vec<i64>) -> bool {
    let mut last = 0;
    let mut double = false;
    for d in digits {
        if *d < last { return false; }
        double = double || *d == last;
        last = *d
    }
    return double;
}

fn main() {
    // x <= x+1, 1 double, double!=triple
    //digits2num(x) < 172851 or digits2num(x) > 675869
    println!("{:?}", num2vec(123456));
    println!("{:?}", vec2num(&num2vec(123456)));
    println!("{:?}", (1..10).map(|x| x));

    println!("{}", iterate_possible_passwords(2, &mut Vec::new(), &|x| vec2num(&x) % 10 == 1));
    println!("day 4 part 1: {}", iterate_possible_passwords(6, &mut Vec::new(), &|x| vec2num(&x) > 172851 && vec2num(&x) < 675869 && correct_1(&x)));

    println!("day 4 part 2: {}", 0);
}
