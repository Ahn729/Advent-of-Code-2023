use std::char::from_digit;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    let grid: Vec<Vec<char>> = read_lines("input.txt")
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    let (dim_x, dim_y) = (grid[0].len(), grid.len());

    let grid = transpose(grid);
    let mut total_weight = 0;
    for col in grid {
        let mut current_weight = dim_y;
        for (i, char) in col.iter().enumerate() {
            match char {
                'O' => {
                    total_weight += current_weight;
                    current_weight -= 1;
                },
                '#' => current_weight = dim_y - i - 1,
                _ => continue
            }
        }
    }
    println!("{}", total_weight);

}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
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
