use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::vec;

use regex::Regex;


fn register_gear(gear_registry: &mut Vec<Gear>, new_gear: Gear) {
    for mut gear in &mut *gear_registry {
        if gear.x == new_gear.x && gear.y == new_gear.y {
            gear.adjacent_numbers.push(new_gear.adjacent_numbers[0]);
            return;
        }
    }
    gear_registry.push(new_gear);

}


#[derive(Debug)]
struct Gear {
    x: usize,
    y: usize,
    adjacent_numbers: Vec<u32>
}

fn contains_symbol(string: &str) -> bool {
    for char in string.chars() {
        if char != '.' {
            return true;
        }
    }
    false
}

fn get_gear_index(string: &str) -> Option<usize> {
    string.chars().position(|c| c == '*')
}

fn is_number_adjacent_to_symbol(grid: &[String], line_no: usize, start: usize, end: usize) -> bool {

    // above / below
    let line_above = grid.get(line_no-1).unwrap();
    let slice_above = &line_above[start-1..end+1];
    if contains_symbol(slice_above) {
        return true
    }

    let line_below = grid.get(line_no+1).unwrap();
    let slice_below = &line_below[start-1..end+1];
    if contains_symbol(slice_below) {
        return true
    }

    // left / right
    let line = grid.get(line_no).unwrap();
    let char_before = &line[start-1..start];
    if contains_symbol(char_before) {
        return true;
    }

    let char_after = &line[end..end+1];
    if contains_symbol(char_after) {
        return true;
    }
    false
}

fn get_adjacent_gear(grid: &[String], line_no: usize, start: usize, end: usize, number: u32) -> Option<Gear> {
    let line = grid.get(line_no).unwrap();

    let char_before = &line[start-1..start];
    if get_gear_index(char_before).is_some() { return Some(Gear {
        x: start - 1, y: line_no, adjacent_numbers: Vec::from([number])
    }) }
    let char_after = &line[end..end+1];
    if get_gear_index(char_after).is_some() { return Some(Gear {
        x: end, y: line_no, adjacent_numbers: Vec::from([number])
    }) }

    let line_above = grid.get(line_no-1).unwrap();
    let slice_above = &line_above[start-1..end+1];
    if let Some(idx) = get_gear_index(slice_above) {return Some(Gear {
        x: start + idx - 1, y: line_no - 1, adjacent_numbers: Vec::from([number])
    })}

    let line_below = grid.get(line_no+1).unwrap();
    let slice_below = &line_below[start-1..end+1];
    if let Some(idx) = get_gear_index(slice_below) {return Some(Gear {
        x: start + idx - 1, y: line_no + 1, adjacent_numbers: Vec::from([number])
    })}


    None
}

fn frame_grid(lines: &Vec<String>) -> Vec<String> {
    let dim_x = lines[0].len();

    let mut grid: Vec<String> = Vec::new();
    grid.push(String::from_utf8(vec![b'.'; dim_x + 2]).unwrap());
    for line in lines {
        grid.push(String::from(".") + line + ".")
    }
    grid.push(String::from_utf8(vec![b'.'; dim_x + 2]).unwrap());

    grid
}

fn main() {

    let lines = read_lines("input.txt");
    let grid = frame_grid(&lines);

    let number_regex = Regex::new(r"(\d+)").unwrap();

    let mut sum_part_numbers = 0;
    let mut gears = Vec::new();

    for (line_no, line) in grid.iter().as_ref().iter().enumerate() {
        for capture in number_regex.captures_iter(line) {
            let number_match = capture.get(1).unwrap();

            if is_number_adjacent_to_symbol(
                &grid,
                line_no,
                number_match.start(),
                number_match.end()) {

                let num: u32 = number_match.as_str().parse().unwrap();
                sum_part_numbers += num;
            }

            let gear = get_adjacent_gear(
                &grid,
                line_no,
                number_match.start(),
                number_match.end(),
                number_match.as_str().parse().unwrap()
            );
            if let Some(gear) = gear {register_gear(&mut gears, gear)}
        }

    }
    let sum_gear_ratios: u32 = gears.iter().map(
        |gear| {
            if gear.adjacent_numbers.len() == 2 {
                gear.adjacent_numbers.iter().product()
            } else { 0 }
        }
    ).sum();

    dbg!(&gears);

    println!("{}", sum_gear_ratios);
    println!("{}", sum_part_numbers);


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