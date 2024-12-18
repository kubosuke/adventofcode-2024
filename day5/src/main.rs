use std::collections::HashMap;
use std::fs::read_to_string;

const FILENAME: &str = "data/input.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> u32 {
    let mut res = 0;
    let mut left: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut right: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in read_to_string(filename).unwrap().lines() {
        if line == "" {
            continue;
        }
        if line.contains("|") {
            let (l, r) = line.split_once("|").unwrap();
            let r_int: u32 = r.trim().parse().unwrap();
            let l_int: u32 = l.trim().parse().unwrap();
            left.entry(r_int).or_insert_with(Vec::new).push(l_int);
            right.entry(l_int).or_insert_with(Vec::new).push(r_int);
        } else {
            if is_valid(&left, &right, line) {
                res += middle(line)
            }
        }
    }
    res
}

fn is_valid(left: &HashMap<u32, Vec<u32>>, right: &HashMap<u32, Vec<u32>>, line: &str) -> bool {
    let l: Vec<u32> = line
        .split(",")
        .map(|e| e.trim().parse::<u32>().unwrap())
        .collect();
    for i in 0..l.len() {
        let before: Vec<u32> = l[..i].to_vec();
        let x = l[i];
        let after = l[i + 1..].to_vec();
        for b in before {
            if !left.get(&x).unwrap().contains(&b) {
                return false;
            }
        }
        for a in after {
            if !right.get(&x).unwrap().contains(&a) {
                return false;
            }
        }
    }
    true
}

fn middle(line: &str) -> u32 {
    let numbers: Vec<u32> = line
        .split(",")
        .map(|e| e.trim().parse::<u32>().unwrap())
        .collect();
    numbers[numbers.len() / 2]
}
