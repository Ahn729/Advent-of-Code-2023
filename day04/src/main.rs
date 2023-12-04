use std::fs::File;
use std::io::{BufReader, prelude::*};

use regex::Regex;

#[derive(Debug)]
struct Card {
    number: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl Card {
    fn from_string(line: &str) -> Self {
        let number_regex = Regex::new(r"(\d+)").unwrap();

        let parts: Vec<&str> = line.split(":").collect();
        let number_split: Vec<&str> = parts.get(1).unwrap().split("|").collect();
        let number: u32 = number_regex.captures(parts.get(0).unwrap())
            .unwrap()
            .get(1)
            .unwrap().as_str().parse().unwrap();

        let winning = number_regex.captures_iter(number_split.get(0).unwrap())
            .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
            .collect();

        let have = number_regex.captures_iter(number_split.get(1).unwrap())
            .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
            .collect();

        Self {
            number, winning, have
        }
    }

    fn get_n_winning(&self) -> u32 {

        // This is highly inefficient (O(n²)), but fast enough
        let mut counter: u32 = 0;
        for number in &self.winning {
            for have in &self.have {
                if *have == *number {
                    counter += 1;
                    break;
                }
            }
        }
        counter
    }

    fn get_points_value(&self) -> u32 {
        let n_winning = self.get_n_winning();
        if n_winning == 0 { 0 } else { 2u32.pow(n_winning-1) }
    }
}

fn main() {
    let lines = read_lines("input.txt");
    let point_total: u32 = lines.iter()
        .map(|line| Card::from_string(line).get_points_value())
        .sum();
    println!("Point total (Part 1): {}", point_total);

    let mut card_counts = vec![1u32; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let card = Card::from_string(line);
        for j in card.number..card.number+card.get_n_winning() {
            card_counts[j as usize] += card_counts[i];
        }
    }
    let sum_cards: u32 = card_counts.iter().sum();
    println!("Number of cards (Part 2): {}", sum_cards);
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