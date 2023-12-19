use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::ops::{Add, Sub};

use indicatif::ProgressBar;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point<T> {
    x: T,
    y: T
}

impl Point<i32> {
    fn rotate_90_deg_fs(&self) -> Self {
        // Rotates 90° in the correct direction for handling forward slashes
        Point {x: -self.y, y: -self.x}
    }

    fn rotate_90_deg_bs(&self) -> Self {
        // Rotates 90° in the correct direction for handling back slashes
        Point {x: self.y, y: self.x}
    }

    fn is_valid(&self, len_grid_y: i32, len_grid_x: i32) -> bool {
        return self.x >= 0 && self.x <= len_grid_x - 1 && self.y >= 0 && self.y <= len_grid_y - 1;
    }
}

impl Add for &Point<i32> {
    type Output = Point<i32>;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: &self.x + other.x,
            y: &self.y + other.y,
        }
    }
}

impl Sub for &Point<i32> {
    type Output = Point<i32>;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: &self.x - other.x,
            y: &self.y - other.y,
        }
    }
}

#[derive(Debug)]
struct Ray<T> {
    points: Vec<Point<T>>,
    is_alive: bool
}


impl<'a, T> Ray<T> {
    fn get_current(&'a self) -> Option<&'a Point<T>> {
        self.points.last()
    }

    fn get_previous(&'a self) -> Option<&'a Point<T>> {
        self.points.get(self.points.len() - 2)
    }

    fn new_starting_from(previous: Point<T>, current: Point<T>) -> Self {
        Self {points: vec![previous, current], is_alive: true}
    }

}

impl<'a, T: PartialEq> Ray<T> {
    fn contains_step(&self, from: &Point<T>, to: &Point<T>) -> bool {
        for (i, point) in self.points.iter().enumerate() {
            if *point == *from && i < self.points.len() - 1 && self.points[i+1] == *to {
                return true;
            }
        }
        false
    }
}

impl <'a> Ray<i32> {
    fn determine_next_positions(&'a self, grid: &[Vec<char>]) -> Vec<Point<i32>> {
        let current_pos = self.get_current().unwrap();
        let previous_pos = self.get_previous().unwrap();
        let d_step: Point<i32> = current_pos - previous_pos;
        let current_space = grid[*&current_pos.y as usize][*&current_pos.x as usize];
        let next = match current_space {
            '.' => vec![current_pos + &d_step],
            '/' => vec![current_pos + &d_step.rotate_90_deg_fs()],
            '\\' => vec![current_pos + &d_step.rotate_90_deg_bs()],
            '-' => if d_step.x != 0 { vec![current_pos + &d_step] } else {
                vec![current_pos + &Point{x: -1, y: 0}, current_pos + &Point{x: 1, y: 0} ]
            }
            '|' => if d_step.y != 0 { vec![current_pos + &d_step] } else {
                vec![current_pos + &Point{x: 0, y: -1}, current_pos + &Point{x: 0, y: 1} ]
            },
            _ => panic!("Unknown symbol detected!"),
        };
        next.into_iter().filter(
            |p| p.is_valid(grid.len() as i32, grid[0].len() as i32)
        ).collect()
    }

    fn move_one_step(&mut self, grid: &[Vec<char>]) -> Option<Ray<i32>> {
        // Moves the ray one step further, returning an optional new ray
        // if this ray was split at a splitter
        let mut next_positions = self.determine_next_positions(grid);

        match next_positions.len() {
            0 => {
                self.is_alive = false;
                None
            },
            1 => {
                self.points.push(next_positions.remove(0));
                None
            },
            2 => {
                self.points.push(next_positions.remove(0));
                Some(Ray::new_starting_from(self.get_previous().unwrap().clone(), next_positions.remove(0)))
            },
            _ => panic!("Determine next position returned more positions than expected!")
        }


    }
}

fn number_of_energized_tiles_for_starting_ray(ray: Ray<i32>, grid: &[Vec<char>]) -> u32 {
    let mut rays = vec![ray];
    for _ in 0..20000 {  // Hack :P
        let mut any_alive = false;
        let mut new_rays = Vec::new();
        for ray in rays.iter_mut() {
            if ray.is_alive {
                any_alive = true;
                if let Some(new_ray) = ray.move_one_step(&grid) {
                    new_rays.push(new_ray);
                }
            }

        }
        rays.append(&mut new_rays);

        let mut to_die = Vec::new();

        for (i, ray) in rays.iter().enumerate() {
            if ray.is_alive {
                for (j, other) in rays.iter().enumerate() {
                    if i != j && other.contains_step(ray.get_previous().unwrap(), ray.get_current().unwrap()) {
                        if ray.get_previous().unwrap() == other.get_previous().unwrap() && ray.get_current().unwrap() == other.get_current().unwrap() && i > j {
                            continue;
                        }
                        to_die.push(i);
                    }
                }
            }
        }

        for i in to_die {
            rays.get_mut(i).unwrap().is_alive = false;
        }

        if !any_alive {
            break;
        }
    }
    let mut visited_points = HashMap::new();
    for ray in rays.iter() {
        for point in ray.points.iter() {
            *visited_points.entry(point.clone()).or_insert(0) += 1;
        }
    }
    visited_points.iter().count() as u32 - 1

}

fn main() {
    let grid: Vec<Vec<char>> = read_lines("input.txt")
    .iter()
    .map(|l| l.chars().collect())
    .collect();

    let mut energized = Vec::new();

    let bar = ProgressBar::new(2 * grid.len() as u64);
    for j in 0..grid.len() {
        let ray = Ray::new_starting_from(
            Point {x:-1, y:j as i32}, Point {x: 0, y: j as i32}
        );
        energized.push(number_of_energized_tiles_for_starting_ray(ray, &grid));
        bar.inc(1);
        let ray = Ray::new_starting_from(
            Point {x:grid[0].len() as i32, y:j as i32}, Point {x: grid[0].len() as i32 - 1, y: j as i32}
        );
        energized.push(number_of_energized_tiles_for_starting_ray(ray, &grid));
        bar.inc(1);
    }

    let bar = ProgressBar::new(2 * grid[0].len() as u64);
    for i in 0..grid[0].len() {
        let ray = Ray::new_starting_from(
            Point {x:i as i32, y:-1}, Point {x: i as i32, y: 0}
        );
        energized.push(number_of_energized_tiles_for_starting_ray(ray, &grid));
        bar.inc(1);
        let ray = Ray::new_starting_from(
            Point {x:i as i32, y:grid.len() as i32}, Point {x:i as i32, y: grid.len() as i32 -1}
        );
        energized.push(number_of_energized_tiles_for_starting_ray(ray, &grid));
        bar.inc(1);

    }
    println!("{:?}", energized.iter().max());


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