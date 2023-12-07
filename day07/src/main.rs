use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use itertools::{Itertools};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandValues {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand<'a> {
    cards: &'a str,
    card_counts: HashMap<char, u32>,
    value: HandValues,
    bid: u32,
}

impl<'a> Hand<'_> {
    fn from_string(line: &'a String) -> Hand<'a> {
        let (cards, bid) = if let Some((cards, bid)) = line.split_whitespace().collect_tuple() { (cards, bid) } else { panic!("Expected two items!") };

        let mut card_counts = HashMap::new();
        for card in cards.chars() {
            *card_counts.entry(card).or_insert(0) += 1;
        }

        let n_jokers = card_counts.remove(&'*').unwrap_or(0);

        let mut n_equal: Vec<u32> = card_counts.values().cloned().collect::<Vec<u32>>();
        n_equal.sort();
        n_equal.reverse();

        if n_equal.len() == 0 {
            n_equal.push(0)
        }
        // Take all jokers to be the value of the card where
        // we own the most copies. Since we don't have
        // straights or flushes, this is always the best option.
        n_equal[0] += n_jokers;


        let hand_value = match n_equal[0] {
            1 => HandValues::HighCard,
            2 => if n_equal[1] == 2 {HandValues::TwoPairs} else {HandValues::OnePair},
            3 => if n_equal[1] == 2 {HandValues::FullHouse} else {HandValues::ThreeOfAKind},
            4 => HandValues::FourOfAKind,
            5 => HandValues::FiveOfAKind,
            _ => { panic!("Cannot determine hand value") }
        };

        Hand {
            cards,
            card_counts,
            bid: bid.parse().unwrap(),
            value: hand_value
        }
    }
}


impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value == other.value {
            for (card_s, card_o) in self.cards.chars().zip(other.cards.chars()) {
                let ps = "*23456789TJQKA".find(card_s).unwrap();
                let po = "*23456789TJQKA".find(card_o).unwrap();
                if ps != po {
                    return ps.cmp(&po);
                }

            }
        }
        self.value.cmp(&other.value)
    }
}


fn main() {
    let lines = read_lines("input.txt");
    let mut hands: Vec<Hand> = lines.iter().map(|l| Hand::from_string(l)).collect();
    hands.sort();

    let mut total_value = 0u32;
    for (i, hand) in hands.iter().enumerate() {
        total_value += (i+1) as u32 * hand.bid;
    }
    println!("Total value is {}", total_value);


    // Part 2: We replace J's by *'s which we define as Jokers
    // to handle everything in one go
    let lines: Vec<String> = lines.iter().map(|l| l.replace('J', "*")).collect();
    let mut hands: Vec<Hand> = lines.iter().map(|l| Hand::from_string(l)).collect();
    hands.sort();

    let mut total_value_with_jokers = 0u32;
    for (i, hand) in hands.iter().enumerate() {
        total_value_with_jokers += (i+1) as u32 * hand.bid;
    }
    println!("Total value with jokers {}", total_value_with_jokers)

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