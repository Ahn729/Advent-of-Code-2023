use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use itertools::Itertools;


fn find_empty_rows_and_cols(lines: Vec<String>) -> (HashSet<usize>, HashSet<usize>) {
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    // Determine empty rows and columns
    let mut empty_rows = HashSet::new();
    let mut empty_cols = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        if !line.contains('#') {
            empty_rows.insert(i);
        }
    }
    for i in 0..grid[0].len() {
        if grid.iter().all(|l| l[i] == '.') {
            empty_cols.insert(i);
        }
    }
    println!("Empty rows: {:?}, empty cols: {:?}", empty_rows, empty_cols);
    (empty_rows, empty_cols)

}

fn find_galaxies(universe: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut galaxies = HashSet::new();
    for (i, line) in universe.iter().enumerate() {
        for (j, char )in line.iter().enumerate() {
            if *char == '#' {
                galaxies.insert((i, j));
            }
        }
    }
    galaxies
}

fn dist(
    galaxy_1: &(usize, usize), galaxy_2: &(usize, usize),
    empty_rows: &HashSet<usize>, empty_cols: &HashSet<usize>,
    empty_space_length: i64) -> i64 {

    let mut ys =vec![galaxy_1.0 as i64, galaxy_2.0 as i64];
    let mut xs =vec![galaxy_1.1 as i64, galaxy_2.1 as i64];
    ys.sort();
    xs.sort();

    let (y0, y1) = (ys[0], ys[1]);
    let (x0, x1) = (xs[0], xs[1]);

    let mut dx = x1-x0;
    for empty_col in empty_cols {
        if x0 < *empty_col as i64 && (*empty_col as i64) < x1 {
            dx += empty_space_length - 1;
        }
    }

    let mut dy = y1-y0;
    for empty_row in empty_rows {
        if y0 < *empty_row as i64 && (*empty_row as i64) < y1 {
            dy += empty_space_length - 1;
        }
    }

    dx + dy


}


fn main() {
    let lines = read_lines("input.txt");
    let universe: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let (empty_rows, empty_cols) = find_empty_rows_and_cols(lines);
    let galaxies = find_galaxies(&universe);

    let sum_dist: i64 = galaxies.iter().tuple_combinations().map(
        |(g1, g2)|  dist(g1, g2, &empty_rows, &empty_cols, 2)
    ).sum();
    println!("Part 1: Sum of all pairwise distances, empty space length = 2: {}", sum_dist);

    let sum_dist: i64 = galaxies.iter().tuple_combinations().map(
        |(g1, g2)|  dist(g1, g2, &empty_rows, &empty_cols, 1_000_000)
    ).sum();

    println!("Part 2: Sum of all pairwise distances, empty space length = 1_000_000: {}", sum_dist);

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
