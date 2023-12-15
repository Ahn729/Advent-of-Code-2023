use std::fs::File;
use std::io::{BufReader, prelude::*};

use itertools::Itertools;


#[derive(Debug)]
struct ConditionRecord<'a> {
    bin_part: &'a str,
    consecutive_damaged_springs: Vec<usize>
}

impl<'a> ConditionRecord<'a> {
    fn from_string(line: &'a str) -> ConditionRecord {
        let mut records = line.split_whitespace();
        let bin_part = records.next().unwrap();
        let consecutive_damaged_springs = records.next().unwrap()
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect();
        Self { bin_part, consecutive_damaged_springs }
    }

    fn matches_possibility(&self, possibility: &str) -> bool {
        let consecutive_damaged: Vec<usize> = possibility.split('.')
            .filter(|c| !c.is_empty())
            .map(|c| c.len())
            .collect();
        consecutive_damaged.len() == self.consecutive_damaged_springs.len() &&
            consecutive_damaged.iter()
            .zip(self.consecutive_damaged_springs.iter())
            .all(|a| *a.0 == *a.1)
    }

    fn get_n_possibilities(&self) -> u32 {
        let mut n_found = 0;
        let n_wildcards = self.bin_part.matches('?').count();
        for combination in vec![[".", "#"]; n_wildcards].iter().multi_cartesian_product() {
            let mut known_parts = self.bin_part.split('?');
            let mut test_string = String::from(known_parts.next().unwrap());
            for (c, k) in combination.iter().zip(known_parts) {
                test_string += c;
                test_string += k;
            }
            if self.matches_possibility(&test_string) {
                n_found += 1;
            }
        }
        n_found

    }
}

fn main() {
    let lines = read_lines("input.txt");

    let sum_n_possibilities: u32 = lines.iter()
        .map(|l| ConditionRecord::from_string(l).get_n_possibilities())
        .sum();

    println!("{}", sum_n_possibilities);
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
