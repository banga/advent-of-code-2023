#![allow(dead_code, unused_imports)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc2023::lib::{self, print_lines};
use nalgebra::{
    matrix, Matrix4, Matrix5, Matrix5x1, Matrix5x6, Matrix6, Matrix6x1, Matrix6x5, SquareMatrix,
};

#[derive(Debug, PartialEq)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
        // f.write_fmt(format_args!("({},{})", self.x, self.y))
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

fn parse_lines(input: Vec<String>) -> Vec<Line> {
    input
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
        .collect()
}

#[allow(dead_code)]
fn part1(input: Vec<String>, bounds: (f64, f64)) {
    let mut count = 0;
    let lines = parse_lines(input);
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

fn solve(dx: isize, dy: isize, dz: isize, l1: &Line, l2: &Line) -> Option<Matrix5x1<f64>> {
    let m: Matrix5<f64> = matrix![
        1.0, 0.0, 0.0, dx as f64 - l1.velocity.x, 0.0;
        0.0, 1.0, 0.0, dy as f64 - l1.velocity.y, 0.0;
        0.0, 0.0, 1.0, dz as f64 - l1.velocity.z, 0.0;
        1.0, 0.0, 0.0, 0.0, dx as f64 - l2.velocity.x;
        0.0, 1.0, 0.0, 0.0, dy as f64 - l2.velocity.y;
    ];
    if let Some(inv) = m.try_inverse() {
        let n = matrix![
            l1.position.x;
            l1.position.y;
            l1.position.z;
            l2.position.x;
            l2.position.y;
        ];
        // Contains [x; y; z; t1; t2]
        let soln = inv * n;
        if soln[3] <= 0.0 || soln[4] <= 0.0 {
            return None;
        }
        return Some(soln);
    }
    None
}

#[allow(dead_code)]
fn part2(input: Vec<String>) {
    let lines = parse_lines(input);
    let mut min_err = 1000_000.0;
    let mut min_pos = (0.0, 0.0, 0.0);
    for dx in 0..1000 {
        for dy in 0..1000 {
            for dz in 0..1000 {
                if let Some(sol1) = solve(dx, dy, dz, &lines[0], &lines[1]) {
                    if let Some(sol2) = solve(dx, dy, dz, &lines[1], &lines[2]) {
                        let err = [sol1[0] - sol2[0], sol1[1] - sol2[1], sol1[2] - sol2[2]]
                            .iter()
                            .map(|v| v.abs())
                            .sum();
                        if err < min_err {
                            min_err = err;
                            min_pos = (sol1[0], sol1[1], sol1[2]);
                            println!("{:?} err={}", min_pos, min_err);
                            println!("{} {} {}", dx, dy, dz);
                            println!(
                                "{} {} {}",
                                sol1[0] + dx as f64 * sol1[3],
                                sol1[1] + dy as f64 * sol1[3],
                                sol1[2] + dz as f64 * sol1[3],
                            );
                        }
                    }
                }
            }
        }
    }
    println!("min_pos={:?} err={}", min_pos, min_err);
    println!(
        "{}",
        min_pos.0.round() + min_pos.1.round() + min_pos.2.round()
    );
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
    // part1(input, (7.0, 27.0));
    part2(input);
}

pub fn main() {
    let input = lib::read_lines();
    // part1(input, (200000000000000.0, 400000000000000.0));
    part2(input);
}
