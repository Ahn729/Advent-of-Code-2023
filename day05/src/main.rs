use std::fs::File;
use std::io::{BufReader, prelude::*};

#[derive(Debug)]
struct Interval {
    start: u64,
    end: u64
}

#[derive(Debug)]
struct Range {
    dest_start_range: u64,
    source_start_range: u64,
    range_length: u64,
}

impl Range {
    fn from_string(string: &String) -> Self {
        let mut parts = string.split_whitespace();
        Self {
            dest_start_range: parts.next().unwrap().parse().unwrap(),
            source_start_range: parts.next().unwrap().parse().unwrap(),
            range_length: parts.next().unwrap().parse().unwrap()
        }

    }

    // (unapplied, applied)
    fn apply(&self, interval: &Interval) -> (Vec<Interval>, Vec<Interval>) {
        if interval.start >= self.source_start_range + self.range_length || interval.end <= self.source_start_range {
            return (vec![Interval { start: interval.start, end: interval.end }], Vec::new())
        }
        if interval.start < self.source_start_range + self.range_length && interval.start >= self.source_start_range {
              if interval.end <= self.source_start_range + self.range_length {
                  return (Vec::new(), vec![Interval {
                      start: interval.start - self.source_start_range + self.dest_start_range,
                      end: interval.end - self.source_start_range + self.dest_start_range
                  }])
              } else {
                  return (vec![
                      Interval { start: self.source_start_range + self.range_length, end: interval.end }
                  ], vec![Interval {
                      start: interval.start - self.source_start_range + self.dest_start_range,
                      end: self.dest_start_range + self.range_length
                  }])
              }
        }
        // interval.start < self.source_start_range
        else {
            if interval.end <= self.source_start_range + self.range_length {
                return (vec![
                    Interval { start: interval.start, end: self.source_start_range }
                ], vec![Interval {
                    start: self.dest_start_range,
                    end: interval.end - self.source_start_range + self.dest_start_range
                }])
            } else {
                return (vec![
                    Interval { start: interval.start, end: self.source_start_range },
                    Interval { start: self.source_start_range + self.range_length, end: interval.end },
                ], vec![Interval {
                    start: self.dest_start_range,
                    end: self.dest_start_range + self.range_length
                }])
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>
}

impl Map {
    fn apply(&self, input: u64) -> u64 {
        for range in &self.ranges {
            if range.source_start_range <= input && input < range.source_start_range + range.range_length {
                return input - range.source_start_range + range.dest_start_range
            }
        }
        input
    }

    fn apply_to_intervals(&self, intervals: Vec<Interval>) -> Vec<Interval> {
        let mut applied = Vec::new();
        let mut unapplied = intervals;

        for range in &self.ranges {
            let mut new_applied = Vec::new();
            let mut still_unapplied = Vec::new();
            for interval in &unapplied {
                let (mut unappl, mut appl) = range.apply(interval);
                new_applied.append(&mut appl);
                still_unapplied.append(&mut unappl);
            }
            applied.extend(new_applied);
            unapplied = still_unapplied;
        }
        // Values not matched remain as they are
        applied.extend(unapplied);
        applied
    }
}

fn apply_all_intervals(maps: &Vec<Map>, mut intervals: Vec<Interval>) -> Vec<Interval> {
    let mut resource_name = String::from("seed");
    while resource_name != "location" {
        for map in maps {
            if map.from == resource_name {
                resource_name = map.to.clone();
                intervals = map.apply_to_intervals(intervals);
            }
        }
    }
    intervals

}

fn apply_all(maps: &Vec<Map>, seed_no: u64) -> u64 {
    let mut resource_name = String::from("seed");
    let mut respurce_no: u64 = seed_no;
    while resource_name != "location" {
        for map in maps {
            if map.from == resource_name {
                resource_name = map.to.clone();
                respurce_no = map.apply(respurce_no);
            }
        }
    }
    respurce_no
}

// seeds, maps
fn parse_input(lines: Vec<String>) -> (Vec<u64>, Vec<Map>) {
    let seeds: Vec<u64> = lines[0].split(':')
        .next_back().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut maps: Vec<Map> = Vec::new();

    let mut i = 1;

    'maps: loop {
        i += 1;
        let mut map_name_split = lines[i].split_whitespace().next().unwrap().split('-');
        let map_from = map_name_split.next().unwrap();
        let map_to = map_name_split.next_back().unwrap();

        let mut map = Map {from: String::from(map_from), to: String::from(map_to), ranges: Vec::new()};

        'ranges: loop {
            i += 1;
            if i >= lines.len() {
                maps.push(map);
                break 'maps;
            } else if lines[i] == "" {
                maps.push(map);
                break 'ranges
            } else {
                map.ranges.push(Range::from_string(&lines[i]))
            }

        }
    }

    (seeds, maps)

}


fn main() {
    let lines = read_lines("input.txt");
    let (seeds, maps) = parse_input(lines);


    dbg!(seeds.iter().map(|s| apply_all(&maps, *s)).min());

    let seed_intervals = seeds.chunks(2).map(
        |w| Interval {start: w[0], end: w[0] + w[1]}
    ).collect();

    dbg!(apply_all_intervals(&maps, seed_intervals).iter().map(|s| s.start).min());



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