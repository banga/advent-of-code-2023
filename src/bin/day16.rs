#![allow(unused_imports)]
use std::{
    cell::Ref,
    cmp::Reverse,
    collections::{hash_map::DefaultHasher, HashMap, HashSet, LinkedList},
    hash::Hasher,
    ops::Index,
    str::FromStr,
    vec,
};

use aoc2023::lib::{self, ascii_to_string, print_line, print_lines};
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn reflect(&self, mirror: &u8) -> Direction {
        match mirror {
            b'/' => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            },
            b'\\' => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
            },
            _ => unreachable!(),
        }
    }
}

fn visit(
    grid: &Vec<Vec<u8>>,
    start: (isize, isize, Direction),
    visited: &mut HashSet<(isize, isize, Direction)>,
) {
    if visited.contains(&start) {
        return;
    }
    let (i, j, dir) = start;
    if i < 0 || i >= grid.len() as isize || j < 0 || j >= grid[0].len() as isize {
        return;
    }

    // println!("Visiting {:?}", start);
    visited.insert(start);

    let ch = grid[i as usize][j as usize];
    match ch {
        b'.' => {
            let (di, dj) = dir.delta();
            visit(grid, (i + di, j + dj, dir), visited);
        }
        b'/' | b'\\' => {
            let dir = dir.reflect(&ch);
            let (di, dj) = dir.delta();
            visit(grid, (i + di, j + dj, dir), visited);
        }
        b'-' => {
            if dir == Direction::Left || dir == Direction::Right {
                let (di, dj) = dir.delta();
                visit(grid, (i + di, j + dj, dir), visited);
            } else {
                for dir in [Direction::Left, Direction::Right] {
                    let (di, dj) = dir.delta();
                    visit(grid, (i + di, j + dj, dir), visited);
                }
            }
        }
        b'|' => {
            if dir == Direction::Up || dir == Direction::Down {
                let (di, dj) = dir.delta();
                visit(grid, (i + di, j + dj, dir), visited);
            } else {
                for dir in [Direction::Up, Direction::Down] {
                    let (di, dj) = dir.delta();
                    visit(grid, (i + di, j + dj, dir), visited);
                }
            }
        }
        _ => unreachable!(),
    }
}

fn get_num_energized(grid: &Vec<Vec<u8>>, start: (isize, isize, Direction)) -> usize {
    let mut visited = HashSet::<(isize, isize, Direction)>::new();
    visit(grid, start, &mut visited);

    let mut num_energized = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let dirs = [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]
            .iter()
            .filter(|d| visited.contains(&(i as isize, j as isize, **d)))
            .collect::<Vec<_>>();

            if dirs.len() == 0 {
                print!(".");
            } else if dirs.len() == 1 {
                print!(
                    "{}",
                    match *dirs[0] {
                        Direction::Up => '^',
                        Direction::Right => '>',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                    }
                );
                num_energized += 1;
            } else {
                print!("{}", dirs.len());
                num_energized += 1;
            }
        }
        println!();
    }
    println!("{}", num_energized);
    num_energized
}

#[allow(unused)]
fn part1() {
    let grid = &lib::read_byte_lines();
    println!("{}", get_num_energized(grid, (0, 0, Direction::Right)));
}

#[allow(unused)]
fn part2() {
    let grid = &lib::read_byte_lines();
    let w = grid[0].len() as isize;
    let h = grid.len() as isize;

    let mut starts: Vec<(isize, isize, Direction)> = vec![];

    for i in 1..h - 1 {
        starts.push((i, 0, Direction::Right));
    }
    for i in 1..h - 1 {
        starts.push((i, w - 1, Direction::Left));
    }
    for j in 1..w - 1 {
        starts.push((0, j, Direction::Down));
    }
    for j in 1..w - 1 {
        starts.push((h - 1, j, Direction::Up));
    }
    starts.extend(vec![
        (0, 0, Direction::Down),
        (0, 0, Direction::Right),
        (w - 1, 0, Direction::Down),
        (w - 1, 0, Direction::Left),
        (0, h - 1, Direction::Up),
        (0, h - 1, Direction::Right),
        (w - 1, h - 1, Direction::Up),
        (w - 1, h - 1, Direction::Left),
    ]);

    let max_energized = starts
        .iter()
        .map(|start| get_num_energized(grid, *start))
        .max()
        .unwrap();
    println!("{}", max_energized);
}

pub fn main() {
    // part1();
    part2();
}
