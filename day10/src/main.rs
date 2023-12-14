use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn find_start(grid: &Vec<Vec<char>>, start_marker: char) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == start_marker {
                return (i, j);
            }
        }
    }
    panic!("Could not find start marker!");
}

fn find_next(grid:  &Vec<Vec<char>>, position: (usize, usize), come_from: (usize, usize)) -> (usize, usize) {
    let current_pipe_part = grid[position.0][position.1];
    let dydx: (i32, i32) = match current_pipe_part {
        '|' => (position.0 as i32 - come_from.0 as i32 , 0),
        '-' => (0, position.1 as i32 - come_from.1 as i32),
        'L' => if come_from.0 != position.0 { (0, 1) } else { (-1, 0) },
        'F' => if come_from.0 != position.0 { (0, 1) } else { (1, 0) },
        'J' => if come_from.0 != position.0 { (0, -1) } else { (-1, 0) },
        '7' => if come_from.0 != position.0 { (0, -1) } else { (1, 0) },
        _ => (0, 1), // Manual hack to avoid handling 2e42 cases
    };
    ((position.0 as i32 + dydx.0) as usize, (position.1 as i32 + dydx.1) as usize)
}

fn main() {
    let grid: Vec<Vec<char>> = read_lines("input.txt").iter().map(
        |s| s.chars().collect()
    ).collect();

    let start = find_start(&grid, 'S');
    let mut current = find_next(&grid, start, start);
    let mut previous = start;

    let mut the_loop = HashSet::from([current]);

    while current != start {
        let next = find_next(&grid, current, previous);
        previous = current;
        current = next;
        the_loop.insert(next);
    }

    // Part 1
    println!("Farthest part of the loop is at distance {}.", the_loop.len() / 2);

    // Part 2
    // Idea: Iterate over the tiles in the grid, row by row.
    // A specific tile is inside the loop if the number of intersections
    // with the loop (counted from the beginning of the row) is odd.
    let mut n_inside = 0;

    for (i, line) in grid.iter().enumerate() {
        let mut n_intersections = 0;
        let mut prev_opening_char = 'x';

        for (j, char) in line.iter().enumerate() {
            if !the_loop.contains(&(i, j)) {
                if n_intersections % 2 == 1 {
                    n_inside += 1;
                }
            } else {
                match char {
                    '|' => n_intersections += 1,
                    'F' | 'L' => prev_opening_char = *char,
                    '7' => if prev_opening_char == 'L' {
                        n_intersections += 1;
                    },
                    'J' => if prev_opening_char == 'F' {
                        n_intersections += 1
                    }
                    _ => continue, // Includes '-' and 'S' (which is a '-' in my input)
                }
            }
        }
    }

    println!("Number of tiles inside: {}", n_inside)


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
