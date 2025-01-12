use std::collections::HashSet;
use std::fs::read_to_string;
const FILENAME: &str = "data/input.txt";

fn main() {
    println!("{:?}", run(FILENAME));
}

fn run(filename: &str) -> usize {
    let content = read_to_string(filename).unwrap();
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    calculate_area_perimeter_sum(&grid)
}

fn calculate_area_perimeter_sum(grid: &Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();
    let mut total_sum = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if !visited.contains(&(i, j)) {
                let target = grid[i][j];
                let (area, perimeter) = explore_region(grid, i, j, target, &mut visited);
                total_sum += area * perimeter;
            }
        }
    }

    total_sum
}

fn explore_region(
    grid: &Vec<Vec<char>>,
    start_i: usize,
    start_j: usize,
    target: char,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut stack = vec![(start_i, start_j)];
    let mut area = 0;
    let mut perimeter = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    while let Some((i, j)) = stack.pop() {
        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));
        area += 1;

        // Check top
        if i == 0 || grid[i - 1][j] != target {
            perimeter += 1;
        } else if !visited.contains(&(i - 1, j)) {
            stack.push((i - 1, j));
        }

        // Check bottom
        if i == rows - 1 || grid[i + 1][j] != target {
            perimeter += 1;
        } else if !visited.contains(&(i + 1, j)) {
            stack.push((i + 1, j));
        }

        // Check left
        if j == 0 || grid[i][j - 1] != target {
            perimeter += 1;
        } else if !visited.contains(&(i, j - 1)) {
            stack.push((i, j - 1));
        }

        // Check right
        if j == cols - 1 || grid[i][j + 1] != target {
            perimeter += 1;
        } else if !visited.contains(&(i, j + 1)) {
            stack.push((i, j + 1));
        }
    }

    (area, perimeter)
}
