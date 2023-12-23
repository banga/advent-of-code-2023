#![allow(dead_code, unused_imports)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc2023::lib::{self};

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Brick {
    id: usize,
    start: Point,
    end: Point,
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} -> {}", self.start, self.end))
    }
}

impl Brick {
    fn shadows(&self, other: &Brick) -> bool {
        assert!(self.start.x < self.end.x);
        assert!(self.start.y < self.end.y);
        assert!(self.start.z < self.end.z);
        assert!(other.start.x < other.end.x);
        assert!(other.start.y < other.end.y);
        assert!(other.start.z < other.end.z);

        if self.start.z < other.end.z {
            return false;
        }

        if other.start.x >= self.end.x {
            return false;
        }
        if other.start.y >= self.end.y {
            return false;
        }
        if other.end.x <= self.start.x {
            return false;
        }
        if other.end.y <= self.start.y {
            return false;
        }

        return true;
    }

    fn supported_by(&self, other: &Brick) -> bool {
        return self.start.z == other.end.z && self.shadows(other);
    }
}

#[allow(dead_code)]
fn part1() {
    let grid = lib::read_lines();

    let mut bricks: Vec<Brick> = vec![];
    for line in grid.iter() {
        let (start, end) = line.split_once('~').unwrap();
        let start: Vec<usize> = start.split(',').map(|n| n.parse().unwrap()).collect();
        let end: Vec<usize> = end.split(',').map(|n| n.parse().unwrap()).collect();

        bricks.push(Brick {
            id: bricks.len() + 1,
            start: Point {
                x: start[0],
                y: start[1],
                z: start[2],
            },
            end: Point {
                x: end[0] + 1,
                y: end[1] + 1,
                z: end[2] + 1,
            },
        });
    }

    // Lowest first
    bricks.sort_by_key(|brick| brick.start.z);

    // Make the bricks fall
    let mut moved_bricks: Vec<Brick> = vec![];
    for (i, brick) in bricks.iter().enumerate() {
        let shadowed_brick_z = (0..i)
            .map(|j| &moved_bricks[j])
            .filter(|b| brick.shadows(b))
            .map(|b| b.end.z)
            .max()
            // If not above any brick, it should be at z = 1
            .unwrap_or(1);
        let delta = brick.start.z - shadowed_brick_z;
        let moved_brick = Brick {
            start: Point {
                z: brick.start.z - delta,
                ..brick.start
            },
            end: Point {
                z: brick.end.z - delta,
                ..brick.end
            },
            ..bricks[i]
        };
        // println!("{} moved to {}", brick, moved_brick);
        moved_bricks.push(moved_brick);
    }

    // println!();

    let mut supported_by: Vec<Vec<bool>> = vec![vec![false; bricks.len()]; bricks.len()];
    // supports[i][j] = true if i is supported by j
    for i in 0..moved_bricks.len() {
        for j in 0..i {
            supported_by[i][j] = moved_bricks[i].supported_by(&moved_bricks[j]);
        }
    }

    // print!(" ");
    // for i in 0..supported_by[0].len() {
    //     print!(" {}", i + 1);
    // }
    // println!();
    // for (i, s) in supported_by.iter().enumerate() {
    //     print!("{} ", i + 1);
    //     for t in s {
    //         print!("{} ", if *t { 'x' } else { '.' });
    //     }
    //     println!();
    // }

    let support_counts: Vec<usize> = supported_by
        .iter()
        .map(|row| row.iter().filter(|x| **x).count())
        .collect();
    // for (i, count) in support_counts.iter().enumerate() {
    //     println!("{} {}", i, count);
    // }

    let mut count = 0;
    for j in 0..supported_by[0].len() {
        let supporting = (j..supported_by.len())
            .filter(|&i| supported_by[i][j] && support_counts[i] == 1)
            .collect::<Vec<_>>();
        if supporting.len() > 0 {
            // println!(
            //     "{} cannot be removed, as it supports {:?}",
            //     j + 1,
            //     supporting.iter().map(|i| i + 1).collect::<Vec<_>>()
            // );
        } else {
            // println!("{} can be removed", j + 1);
            count += 1;
        }
    }

    println!("{}", count);
}

#[allow(dead_code)]
fn part2() {}

pub fn main() {
    part1();
    part2();
}
