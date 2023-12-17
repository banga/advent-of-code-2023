#![allow(unused_imports)]
use std::{
    collections::{hash_map::DefaultHasher, BinaryHeap, HashMap, HashSet},
    vec,
};

use aoc2023::lib::{self, ascii_to_string, print_line, print_lines};
use num::{complex::Complex32, Complex};
use regex::Regex;

#[derive(Hash, PartialEq, Eq, Debug)]
struct State {
    pos: Complex<isize>,
    dir: Complex<isize>,
    heat: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.heat.partial_cmp(&self.heat)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pos.re.cmp(&other.pos.re)
    }
}

fn get_min_heat(grid: &Vec<Vec<u8>>, min_dist: isize, max_dist: isize) {
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    let mut visited = HashMap::<(Complex<isize>, Complex<isize>), usize>::new();

    let mut q = BinaryHeap::new();
    q.push(State {
        pos: Complex::<isize>::new(0, 0),
        dir: Complex::<isize>::new(1, 0),
        heat: 0,
    });
    q.push(State {
        pos: Complex::<isize>::new(0, 0),
        dir: Complex::<isize>::new(0, 1),
        heat: 0,
    });

    while let Some(state) = q.pop() {
        let State { pos, dir, heat } = state;
        // println!("pos={} dir={} heat={}", pos, dir, heat);

        if let Some(prev_heat) = visited.get(&(pos, dir)) {
            if *prev_heat <= heat {
                continue;
            }
        }
        visited.insert((pos, dir), heat);

        let next_dir = dir * Complex::i();
        let mut next_pos = pos;
        let mut next_heat = heat;
        for dist in 1..=max_dist {
            if (dir.re == -1 && next_pos.re <= 0)
                || (dir.re == 1 && next_pos.re >= h - 1)
                || (dir.im == -1 && next_pos.im <= 0)
                || (dir.im == 1 && next_pos.im >= w - 1)
            {
                break;
            }
            next_pos += dir;
            next_heat += grid[next_pos.re as usize][next_pos.im as usize] as usize;
            if dist >= min_dist {
                q.push(State {
                    pos: next_pos,
                    dir: next_dir,
                    heat: next_heat,
                });
                q.push(State {
                    pos: next_pos,
                    dir: -next_dir,
                    heat: next_heat,
                });
            }
        }
        // println!("  {} {}", next_pos, next_heat);
    }

    let min_heat = visited
        .iter()
        .filter_map(|((pos, _), v)| {
            if pos.re == h - 1 && pos.im == w - 1 {
                Some(*v)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    println!("{}", min_heat);
}

#[allow(unused)]
fn part1() {
    let grid: Vec<Vec<u8>> = lib::read_byte_lines()
        .iter()
        .map(|line| line.iter().map(|&c| c - b'0').collect())
        .collect();

    get_min_heat(&grid, 0, 3);
}

#[allow(unused)]
fn part2() {
    let grid: Vec<Vec<u8>> = lib::read_byte_lines()
        .iter()
        .map(|line| line.iter().map(|&c| c - b'0').collect())
        .collect();

    get_min_heat(&grid, 4, 10);
}

pub fn main() {
    // part1();
    part2();
}
