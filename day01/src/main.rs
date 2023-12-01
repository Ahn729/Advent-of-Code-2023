use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::vec;

fn main() {
    let gib = read_lines("input.txt");
    let mut calibration_values = Vec::new();
    for line in gib {
        // Remove this for part 1
        let line = line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        // END: Remove this for part 1
        let mut digits = Vec::new();
        for char in line.chars() {
            if char.is_numeric() {
                digits.push(char.to_digit(10).unwrap())
            }
        }
        calibration_values.push(10*digits[0] + digits[digits.len() - 1])
    }

    dbg!(&calibration_values);
    println!("{}", calibration_values.iter().sum::<u32>())

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