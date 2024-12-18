use std::fs::read_to_string;

const FILENAME: &str = "data/input.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> i64 {
    let mut res = 0;
    let lines: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            // Check horizontally
            if j + 3 < lines[0].len()
                && lines[i][j] == 'X'
                && lines[i][j + 1] == 'M'
                && lines[i][j + 2] == 'A'
                && lines[i][j + 3] == 'S'
            {
                res += 1;
            }
            // Check vertically
            if i + 3 < lines.len()
                && lines[i][j] == 'X'
                && lines[i + 1][j] == 'M'
                && lines[i + 2][j] == 'A'
                && lines[i + 3][j] == 'S'
            {
                res += 1;
            }
            // Check inverse horizontally
            if j + 3 < lines[0].len()
                && lines[i][j] == 'S'
                && lines[i][j + 1] == 'A'
                && lines[i][j + 2] == 'M'
                && lines[i][j + 3] == 'X'
            {
                res += 1;
            }
            // Check inverse vertically
            if i + 3 < lines.len()
                && lines[i][j] == 'S'
                && lines[i + 1][j] == 'A'
                && lines[i + 2][j] == 'M'
                && lines[i + 3][j] == 'X'
            {
                res += 1;
            }
            // Check cross
            if i + 3 < lines.len()
                && j + 3 < lines[0].len()
                && lines[i][j] == 'X'
                && lines[i + 1][j + 1] == 'M'
                && lines[i + 2][j + 2] == 'A'
                && lines[i + 3][j + 3] == 'S'
            {
                res += 1;
            }
            // Check inverse cross
            if i + 3 < lines.len()
                && j >= 3
                && lines[i][j] == 'X'
                && lines[i + 1][j - 1] == 'M'
                && lines[i + 2][j - 2] == 'A'
                && lines[i + 3][j - 3] == 'S'
            {
                res += 1;
            }
            // Check cross
            if i + 3 < lines.len()
                && j + 3 < lines[0].len()
                && lines[i][j] == 'S'
                && lines[i + 1][j + 1] == 'A'
                && lines[i + 2][j + 2] == 'M'
                && lines[i + 3][j + 3] == 'X'
            {
                res += 1;
            }
            // Check inverse cross
            if i + 3 < lines.len()
                && j >= 3
                && lines[i][j] == 'S'
                && lines[i + 1][j - 1] == 'A'
                && lines[i + 2][j - 2] == 'M'
                && lines[i + 3][j - 3] == 'X'
            {
                res += 1;
            }
        }
    }
    res
}
