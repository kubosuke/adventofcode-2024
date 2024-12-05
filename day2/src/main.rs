use std::fs::read_to_string;

const FILENAME: &str = "data/input.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> i64 {
    let mut res = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let v: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        if safe(v) {
            res += 1;
        }
    }
    res
}

fn safe(v: Vec<i64>) -> bool {
    let mut prev = -1;
    let mut increasing = false;
    let mut decreasing = false;

    for vv in v {
        if prev == -1 {
            prev = vv;
            continue;
        }
        if vv > prev {
            increasing = true;
        } else if vv < prev {
            decreasing = true;
        }
        if vv.abs_diff(prev) < 1 || vv.abs_diff(prev) > 3 {
            return false;
        };
        prev = vv;
    }

    if increasing && decreasing {
        return false;
    }
    true
}
