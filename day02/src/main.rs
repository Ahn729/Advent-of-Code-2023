use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::vec;

use regex::Regex;

#[derive(Debug)]
struct Drawing {
    reds: u32,
    greens: u32,
    blues: u32
}

impl Drawing {
    fn power(&self) -> u32 {
        self.reds * self.blues * self.greens
    }
}

#[derive(Debug)]
struct Game {
    number: u32,
    drawings: Vec<Drawing>
}

impl Game {
    fn from_string(line: &str) -> Game {
        let number_regex = Regex::new(r"^Game (\d*)").unwrap();
        let green_regex = Regex::new(r"(\d*) green").unwrap();
        let blue_regex = Regex::new(r"(\d*) blue").unwrap();
        let red_regex = Regex::new(r"(\d*) red").unwrap();

        let caps = number_regex.captures(line).unwrap();
        let game_number: u32 = caps[1].parse().expect("Unable to parse game numer");

        let mut drawings: Vec<Drawing> = Vec::new();
        for drawing in line.split(";") {
            let green_caps = green_regex.captures(drawing);
            let n_green = match green_caps {
                Some(capture) => capture[1].parse::<u32>().expect("Unable to parse n_green"),
                None => 0u32
            };
            let red_caps = red_regex.captures(drawing);
            let n_red = match red_caps {
                Some(capture) => capture[1].parse::<u32>().expect("Unable to parse n_red"),
                None => 0u32
            };
            let blue_caps = blue_regex.captures(drawing);
            let n_blue = match blue_caps {
                Some(capture) => capture[1].parse::<u32>().expect("Unable to parse n_blue"),
                None => 0u32
            };
            drawings.push(Drawing {
                reds: n_red,
                greens: n_green,
                blues: n_blue,
            })
        }
        Game {
            drawings,
            number: game_number
        }
    }

    fn is_valid_for(&self, n_green: u32, n_red: u32, n_blue: u32) -> bool {
        for drawing in &self.drawings {
            if drawing.greens > n_green || drawing.reds > n_red || drawing.blues > n_blue {
                return false;
            }
        }
        true
    }

    fn min_possible(&self) -> Drawing {
        let blues = self.drawings.iter().map(|drawing| drawing.blues).max().unwrap();
        let greens = self.drawings.iter().map(|drawing| drawing.greens).max().unwrap();
        let reds = self.drawings.iter().map(|drawing| drawing.reds).max().unwrap();
        Drawing {
            greens, blues, reds
        }
    }

    fn min_power(&self) -> u32 {
        self.min_possible().power()
    }
}

fn main() {
    let lines = read_lines("input.txt");
    let mut sum_valid_game_ids = 0;
    let mut total_power_needed = 0;

    for line in &lines {
        let game = Game::from_string(&line);
        if game.is_valid_for(13, 12, 14) {
            sum_valid_game_ids += game.number;
        }
        total_power_needed += game.min_power();
    }
    println!("Sum of valid game ids: {sum_valid_game_ids}");
    println!("Total power needed: {total_power_needed}");

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