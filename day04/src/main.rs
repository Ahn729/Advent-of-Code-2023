use std::collections::HashMap;
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

        let mut parts = line.split(':');

        let number: u32 = number_regex.captures(parts.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap().as_str().parse().unwrap();

        let mut number_split = parts.next().unwrap().split('|');

        let winning = number_regex.captures_iter(number_split.next().unwrap())
            .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
            .collect();

        let have = number_regex.captures_iter(number_split.next().unwrap())
            .map(|c| c.get(1).unwrap().as_str().parse::<u32>().unwrap())
            .collect();

        Self {
            number, winning, have
        }
    }

    fn get_n_winning(&self) -> u32 {
        let winning: HashMap<u32, u32> = HashMap::from_iter(
            self.winning.iter().map(|num| (*num, 1))
        );
        self.have.iter().map(
            |num| *winning.get(num).unwrap_or(&0)
        ).sum()
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