use std::fs::File;
use std::io::{BufReader, prelude::*};

fn test_reflection_row(grid: &[Vec<char>], row_no: usize) -> bool {
    let (mut above, mut below) = (row_no - 1, row_no);
    while below < grid.len() {
        if grid[above] == grid[below] {
            if above == 0 {
                break;
            }
            above -= 1;
            below += 1;
        } else {
            return false
        }
    }
    true
}

fn test_reflection_col(grid: & [Vec<char>], col_no: usize) -> bool {
    let (mut left, mut right) = (col_no - 1, col_no);
    while right < grid[0].len() {
        if grid.iter().all(
            |v| v[left] == v[right]
        ) {
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        } else {
            return false
        }
    }
    true
}

fn grid_value(grid: &[Vec<char>]) -> usize {
    for i in 1..grid[0].len() {
        if test_reflection_col(grid, i) {
            return i;
        }
    }
    for i in 1..grid.len() {
        if test_reflection_row(grid, i) {
            return 100*i;
        }
    }
    0
}


fn main() {
    let grid: Vec<Vec<char>> = read_lines("input.txt")
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    let mut current_subgrid;
    let mut total_value = 0;

    let mut last_white_line = 0;
    for (i, line) in grid.iter().enumerate() {
        if line.len() == 0 {
            current_subgrid = &grid[last_white_line+1..i];
            let value = grid_value(current_subgrid);
            total_value += value;
            last_white_line = i;
        }
    }
    current_subgrid = &grid[last_white_line+1..];

    total_value += grid_value(current_subgrid);

    println!("{}", total_value)
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut res = vec![];

    let file = File::open(filename).expect("Cannot open file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => res.push(line),
            Err(e) => println!("{e}")
        }
    }
    res
}
