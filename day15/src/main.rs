use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use indexmap::IndexMap;

struct LensLibrary<'a> {
    boxes: HashMap<i32, IndexMap<&'a str, u32>>
}

impl<'a> LensLibrary<'a> {
    fn perform_single_operation(&mut self, op: &'a str) {
        if op.contains('=') {
            let mut iter = op.split('=');
            let label = iter.next().unwrap();
            let focal_length: u32 = iter.next().unwrap().parse().unwrap();
            let box_no = compute_hash(label);
            let relevant_box = self.boxes.get_mut(&box_no).unwrap();
            *relevant_box.entry(label).or_insert(focal_length) = focal_length;
        } else {
            let mut iter = op.split('-');
            let label = iter.next().unwrap();
            let box_no = compute_hash(label);
            let relevant_box = self.boxes.get_mut(&box_no).unwrap();
            relevant_box.shift_remove(label);
        }

    }

    fn get_focusing_power(self) -> i32 {
        self.boxes.iter().flat_map(
            |(box_no, lenses)| lenses.iter().enumerate().map(
                |(i, (_, lens))| (*box_no + 1) * (i as i32 + 1) * *lens as i32
            )
        ).sum()
    }

    fn new() -> Self {
        let mut boxes: HashMap<i32, IndexMap<&str, u32>>  = HashMap::new();
        for i in 0..256 {
            boxes.insert(i, IndexMap::new());
        }
    Self { boxes }
    }
}

fn compute_hash(string: &str) -> i32 {
    let mut current = 0;
    for char in string.chars() {
        current += char as i32;
        current *= 17;
        current %= 256;
    }
    current
}

fn main() {
    let line = read_lines("input.txt");
    let mut total: u32 = 0;
    for string in line[0].split(',') {
        total += compute_hash(string) as u32;
    }
    println!("Total hash (Part 1): {}", total);

    let mut lens_library = LensLibrary::new();
    for operation in line[0].split(',') {
        lens_library.perform_single_operation(operation);
    }

    println!("Lens config focusing power: {}", lens_library.get_focusing_power());


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