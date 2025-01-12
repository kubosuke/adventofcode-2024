use std::fs::read_to_string;
const FILENAME: &str = "data/input.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> usize {
    let mut v: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|line| line.chars().collect())
        .collect();
    for _ in 0..25 {
        let mut nv = vec![];
        for vv in &v {
            if vv.len() % 2 == 0 {
                let half = vv.len() / 2;
                nv.push(cn(vv[..half].to_vec()));
                nv.push(cn(vv[half..].to_vec()));
            } else {
                let num: u64 = vv.iter().collect::<String>().parse().unwrap_or(0);
                if num == 0 {
                    nv.push(vec!['1']);
                } else {
                    nv.push((num * 2024).to_string().chars().collect());
                }
            }
        }
        v = nv;
    }
    v.len()
}

fn cn(v: Vec<char>) -> Vec<char> {
    let num: u64 = v.iter().collect::<String>().parse().unwrap_or(0);
    num.to_string().chars().collect()
}
