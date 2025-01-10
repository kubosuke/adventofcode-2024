use std::collections::HashSet;
use std::fs::read_to_string;

const FILENAME: &str = "data/input.txt";
// const FILENAME: &str = "data/input_sample.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> usize {
    let maze: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antinodes: Vec<Vec<bool>> = maze
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();

    let antennas = {
        let mut hs = HashSet::new();
        for r in &maze {
            for &ch in r {
                if ch != '.' {
                    hs.insert(ch);
                }
            }
        }
        hs
    };

    for a in antennas {
        let mut v = vec![];
        for y in 0..maze.len() {
            for x in 0..maze[0].len() {
                if a == maze[x][y] {
                    v.push((x, y))
                }
            }
        }
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                if let Some((sx, sy)) = find_symmetric_point(v[i], v[j]) {
                    if sx < maze.len() && sy < maze[0].len() {
                        antinodes[sx][sy] = true;
                    }
                }
                if let Some((sx, sy)) = find_symmetric_point(v[j], v[i]) {
                    if sx < maze.len() && sy < maze[0].len() {
                        antinodes[sx][sy] = true;
                    }
                }
            }
        }
    }
    antinodes.iter().flatten().filter(|&&ch| ch).count()
}

fn find_symmetric_point(center: (usize, usize), point: (usize, usize)) -> Option<(usize, usize)> {
    let (cx, cy) = center;
    let (px, py) = point;

    if 2 * cx >= px && 2 * cy >= py {
        let symmetric_x = 2 * cx - px;
        let symmetric_y = 2 * cy - py;
        Some((symmetric_x, symmetric_y))
    } else {
        None
    }
}
