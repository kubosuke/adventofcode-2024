use std::fs::read_to_string;

const FILENAME: &str = "data/input.txt";
// const FILENAME: &str = "data/input_sample.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> u64 {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let left_part = parts.next().unwrap().parse::<u64>().unwrap();
            let right_vec = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            if ok(left_part, right_vec) {
                left_part
            } else {
                0
            }
        })
        .sum()
}

fn ok(r: u64, v: Vec<u64>) -> bool {
    let n = v.len();
    for i in 0..(1 << n) {
        let mut sum = 0;
        for j in 0..n {
            if (i & (1 << j)) != 0 {
                sum += v[j];
            } else {
                sum *= v[j];
            }
        }
        if sum == r {
            return true;
        }
    }
    false
}
