#![allow(dead_code, unused_imports)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc2023::lib::{self, print_lines};

#[derive(Debug, PartialEq)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    position: Vector,
    velocity: Vector,
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} @ {}", self.position, self.velocity))
    }
}

impl Line {
    fn intersect2d(&self, other: &Self) -> Option<Vector> {
        assert!(self.velocity.x != 0.0);
        assert!(other.velocity.x != 0.0);

        let a = self.velocity.y / self.velocity.x;
        let b = other.velocity.y / other.velocity.x;

        if a == b {
            return None;
        }

        let c = self.position.y - a * self.position.x;
        let d = other.position.y - b * other.position.x;

        let x = (d - c) / (a - b);
        let y = (a * d - b * c) / (a - b);

        // Skip points in the "past"
        if (x - self.position.x) * self.velocity.x < 0.0 {
            return None;
        }
        if (x - other.position.x) * other.velocity.x < 0.0 {
            return None;
        }

        Some(Vector { x, y, z: 0.0 })
    }
    // fn intersect(&self, other: &Self) -> Option<Vector> {}
}

#[allow(dead_code)]
fn part1(lines: Vec<String>, bounds: (f64, f64)) {
    let lines: Vec<_> = lines
        .iter()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let pi = pos
                .split(",")
                .map(|s| s.trim().parse().unwrap())
                .collect::<Vec<_>>();
            let vi = vel
                .split(",")
                .map(|s| s.trim().parse().unwrap())
                .collect::<Vec<_>>();
            Line {
                position: Vector {
                    x: pi[0],
                    y: pi[1],
                    z: pi[2],
                },
                velocity: Vector {
                    x: vi[0],
                    y: vi[1],
                    z: vi[2],
                },
            }
        })
        .collect();

    let mut count = 0;
    for i in 0..lines.len() {
        let line1 = &lines[i];
        for j in 0..i {
            let line2 = &lines[j];
            if let Some(p) = line1.intersect2d(&line2) {
                if p.x >= bounds.0 && p.x <= bounds.1 && p.y >= bounds.0 && p.y <= bounds.1 {
                    println!("A: {}", line1);
                    println!("B: {}", line2);
                    println!("Intersect at {}", p);
                    println!();
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

#[allow(dead_code)]
fn part2() {
    // println!("{}", num_fallen);
}

#[test]
fn test() {
    let input = r"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"
    .trim()
    .split('\n')
    .map(|s| s.to_string())
    .collect();
    part1(input, (7.0, 27.0));
}

pub fn main() {
    let input = lib::read_lines();
    part1(input, (200000000000000.0, 400000000000000.0));
    // part2(input);
}
