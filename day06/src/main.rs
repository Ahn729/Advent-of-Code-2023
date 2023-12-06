use std::fs::File;
use std::io::{BufReader, prelude::*};

use itertools::{Itertools};

fn boat_travels_far_enough(time_available: u64, wind_time: u64, distance_needed: u64) -> bool {
    let speed = wind_time;
    let travel_time = time_available - wind_time;
    let distance_travelled = travel_time * speed;
    return distance_needed < distance_travelled;
}

// This is inefficient, but fortunately, the input is not large enough
// to make it unusable.
fn get_n_winning_possibilities(times: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    times.iter().zip(distances.iter()).map({
        |(time_available, distance_needed)| {
            let mut n_winning = 0;
            for wind_time in 0..*time_available {
                if boat_travels_far_enough(*time_available, wind_time, *distance_needed) {
                    n_winning += 1;
                }
            }
            n_winning
        }
    }).product::<u64>()

}

fn main() {
    let lines = read_lines("input.txt");
    let (times, distances)  = lines.iter().map({
        |x| x.split(':').next_back()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    }).collect_tuple().unwrap();

    dbg!(get_n_winning_possibilities(&times, &distances));
    dbg!(get_n_winning_possibilities(
        &vec![times.iter().map(|s| s.to_string()).join("").parse::<u64>().unwrap()],
        &vec![distances.iter().map(|s| s.to_string()).join("").parse::<u64>().unwrap()]
    ));


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