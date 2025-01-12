use std::collections::VecDeque;
use std::fs::read_to_string;
const FILENAME: &str = "data/input.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> usize {
    let m: Vec<Vec<u32>> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let mut starts = vec![];
    for x in 0..m.len() {
        for y in 0..m[0].len() {
            if m[x][y] == 0 {
                starts.push((x, y));
            }
        }
    }
    let dx = vec![1, 0, -1, 0];
    let dy = vec![0, 1, 0, -1];

    let mut ans = 0;

    for &(start_x, start_y) in &starts {
        let mut count = 0;
        let mut visited = vec![vec![false; m[0].len()]; m.len()];
        let mut queue = VecDeque::new();
        queue.push_back((start_x, start_y, 0));

        while let Some((x, y, current_value)) = queue.pop_front() {
            for i in 0..4 {
                let nx = x as isize + dx[i];
                let ny = y as isize + dy[i];

                if nx >= 0 && nx < m.len() as isize && ny >= 0 && ny < m[0].len() as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if m[nx][ny] == (current_value + 1) {
                        if m[nx][ny] == 9 && !visited[nx][ny] {
                            count += 1;
                            visited[nx][ny] = true
                        } else {
                            queue.push_back((nx, ny, m[nx][ny]));
                        }
                    }
                }
            }
        }
        println!("start: ({}, {}), count: {}", start_x, start_y, count);
        ans += count
    }
    ans
}
