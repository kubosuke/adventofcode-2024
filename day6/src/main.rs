use std::fs::read_to_string;

const FILENAME: &str = "data/input.txt";
// const FILENAME: &str = "data/input_sample.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> i64 {
    let mut maze: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let h: i32 = maze.len() as i32;
    let w: i32 = maze[0].len() as i32;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut di: usize = 0;
    let dy: Vec<i32> = vec![0, 1, 0, -1];
    let dx: Vec<i32> = vec![-1, 0, 1, 0];
    for (i, row) in maze.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            x = i as i32;
            y = j as i32;
            break;
        }
    }

    loop {
        maze[x as usize][y as usize] = 'X';
        let nx = x + (dx[di] * 1);
        let ny = y + (dy[di] * 1);
        if nx < 0 || ny < 0 || nx >= w || ny >= h {
            break;
        }
        if maze[nx as usize][ny as usize] == '#' {
            di = (di + 1) % 4;
            continue;
        }
        x = nx;
        y = ny;
    }

    let mut res = 0;
    for row in &maze {
        for &cell in row {
            if cell == 'X' {
                res += 1;
            }
        }
    }
    res
}
