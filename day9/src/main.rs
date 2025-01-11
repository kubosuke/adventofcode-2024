use std::{fs::read_to_string, vec};

const FILENAME: &str = "data/input.txt";
// const FILENAME: &str = "data/input_sample.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> usize {
    // parse

    let v: Vec<u32> = read_to_string(filename)
        .unwrap()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect();

    let mut cnt = 0;
    let mut disk = vec![];
    for (i, &vv) in v.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..vv {
                disk.push(Some(cnt))
            }
            cnt += 1;
        } else {
            for _ in 0..vv {
                disk.push(None);
            }
        }
    }

    // swap
    let limit = disk.iter().filter(|&&x| x.is_none()).count();
    let mut idx = disk.len() - 1;
    for i in 0..disk.len() - limit {
        if disk[i].is_none() {
            loop {
                if disk[idx].is_some() {
                    break;
                }
                idx -= 1;
            }
            disk.swap(i, idx);
        }
    }

    // calc checksum
    let mut ans = 0;
    for i in 0..disk.len() {
        if let Some(value) = disk[i] {
            ans += i * value;
        }
    }
    ans
}
