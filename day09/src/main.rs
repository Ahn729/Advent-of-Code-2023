use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    let lines = read_lines("input.txt");
    let histories: Vec<Vec<i32>> = lines
        .iter().map(
        |l| l.split_whitespace().map(
            |d| d.parse::<i32>().unwrap()
        ).collect()
    ).collect();

    let result: i32 = histories.iter().map(
        process_pt1
    ).sum();
    println!("{}", result);

    let result_pt2: i32 = histories.iter().map(
        process_pt2
    ).sum();
    println!("{}", result_pt2);
}

fn process_pt1(history: &Vec<i32>) -> i32 {
    let mut result = 0;
    let mut current_sequence = history;
    let mut sequence;
    while !current_sequence.iter().all(|d| *d == 0) {
        result += current_sequence[current_sequence.len() - 1];
        sequence = process_one_step(current_sequence);
        current_sequence = &sequence;
    }
    result

}

fn process_pt2(history: &Vec<i32>) -> i32 {
    let mut result = 0;
    let mut step_number = 0;
    let mut current_sequence = history;
    let mut sequence;
    while !current_sequence.iter().all(|d| *d == 0) {
        let sign = if step_number % 2 == 0 { 1 } else {-1 };
        result += current_sequence[0] * sign;
        sequence = process_one_step(current_sequence);
        current_sequence = &sequence;
        step_number += 1;
    }
    result

}

fn process_one_step(history: &[i32]) -> Vec<i32> {
    history.windows(2)
        .map(|w| w[1]-w[0]).collect()
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