use std::{fs::read_to_string, vec};

const FILENAME: &str = "data/input.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> u64 {
    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];

    for line in read_to_string(filename).unwrap().lines() {
        let v: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        left.push(v[0]);
        right.push(v[1]);
    }
    left.sort();
    right.sort();

    let mut sum = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        sum += l.abs_diff(*r);
    }
    sum
}
