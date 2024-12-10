use std::fs::read_to_string;

const FILENAME: &str = "data/input.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> i64 {
    let mut res = 0;
    for line in read_to_string(filename).unwrap().lines() {
        for cap in line.match_indices("mul(") {
            let start = cap.0 + 4; // position after "mul("
            if let Some(end) = line[start..].find(')') {
                let args: Vec<&str> = line[start..start + end].split(',').collect();
                if args.len() == 2 {
                    if let (Ok(x), Ok(y)) =
                        (args[0].trim().parse::<i64>(), args[1].trim().parse::<i64>())
                    {
                        res += x * y
                    }
                }
            }
        }
    }
    res
}
