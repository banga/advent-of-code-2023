#![allow(unused_imports)]
use std::{
    cell::Ref,
    cmp::Reverse,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::Hasher,
    ops::Index,
    str::FromStr,
    vec,
};

use aoc2023::lib::{self, ascii_to_string, print_line, print_lines};
use regex::Regex;

#[allow(unused)]
fn part1() {
    let grid = lib::read_byte_lines();
    let mut sum = 0;
    let get_round_rock_run_value = |num_round_rocks, cube_rock_height| {
        num_round_rocks * cube_rock_height - (num_round_rocks * (num_round_rocks + 1)) / 2
    };

    for j in 0..grid[0].len() {
        let mut k = 1;
        let mut num_round_rocks = 0;
        while k <= grid.len() {
            match grid[grid.len() - k][j] {
                b'O' => num_round_rocks += 1,
                b'#' => {
                    sum += get_round_rock_run_value(num_round_rocks, k);
                    num_round_rocks = 0
                }
                b'.' => {}
                _ => unreachable!(),
            }
            k += 1;
        }
        sum += get_round_rock_run_value(num_round_rocks, k);
    }
    println!("{}", sum);
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn get_char_at(grid: &mut Vec<Vec<u8>>, direction: Direction, i: isize, j: isize) -> &mut u8 {
    match direction {
        Direction::North | Direction::South => &mut grid[i as usize][j as usize],
        Direction::East | Direction::West => &mut grid[j as usize][i as usize],
    }
}

fn tilt(grid: &mut Vec<Vec<u8>>, direction: Direction) {
    let (width, height) = (grid[0].len() as isize, grid.len() as isize);
    let (start, end, delta) = match direction {
        Direction::North => (height - 1, -1, -1),
        Direction::South => (0, height, 1),
        Direction::East => (0, width, 1),
        Direction::West => (width - 1, -1, -1),
    };
    let cross_end = match direction {
        Direction::North | Direction::South => width,
        Direction::East | Direction::West => height,
    };

    // println!("{:?}", (start, end, delta));

    for j in 0..cross_end {
        let mut num_round_rocks = 0;
        let mut i = start;
        while i != end {
            let ch: &mut u8 = get_char_at(grid, direction, i, j);
            match *ch {
                b'O' => {
                    num_round_rocks += 1;
                    *ch = b'.';
                }
                b'#' => {
                    for k in 0..num_round_rocks {
                        *get_char_at(grid, direction, i - (k + 1) * delta, j) = b'O';
                    }
                    num_round_rocks = 0;
                }
                b'.' => {}
                _ => unreachable!(),
            }
            i += delta;
        }

        for k in 0..num_round_rocks {
            *get_char_at(grid, direction, i - (k + 1) * delta, j) = b'O';
        }
    }
}

fn get_grid_hash(grid: &Vec<Vec<u8>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    let width = grid[0].len();
    for i in 0..grid.len() {
        for j in 0..width {
            if grid[i][j] == b'O' {
                hasher.write_usize(i);
                hasher.write_usize(j);
            }
        }
    }
    hasher.finish()
}

fn get_load(grid: &Vec<Vec<u8>>) -> usize {
    let mut load = 0;
    for (i, line) in grid.iter().enumerate() {
        let height = grid.len() - i;
        load += line.iter().filter(|&&ch| ch == b'O').count() * height;
    }
    load
}

#[allow(unused)]
fn part2() {
    let mut grid = lib::read_byte_lines();
    let mut seen: HashMap<u64, (u64, usize, usize)> = HashMap::new();
    let mut round = 1;
    let mut prev_hash = get_grid_hash(&grid);
    let rounds = 1000_000_000;
    while round <= rounds {
        for direction in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            // println!();
            tilt(&mut grid, direction);
            // print_lines(&grid);
        }

        let hash = get_grid_hash(&grid);
        let load = get_load(&grid);
        println!("{}. load = {} hash = {}", round, load, hash);

        if let Some((_, cycle_start, _)) = seen.insert(prev_hash, (hash, round, load)) {
            println!("Cycle found on round {}, hash {}", round, hash);
            println!("{:?}", seen);
            let cycle_length = round - cycle_start;
            let offset = (rounds - cycle_start) % cycle_length;
            let final_round = cycle_start + offset;
            let (_, _, final_load) = seen.values().find(|(_, r, _)| *r == final_round).unwrap();
            println!("{} {} {}", cycle_length, final_round, final_load);
            break;
        }

        prev_hash = hash;
        round += 1;
    }

    // print_lines(&grid);
    // println!("{}", get_load(&grid));
}

pub fn main() {
    // part1();
    part2();
}
