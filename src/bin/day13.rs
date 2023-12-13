#![allow(unused_imports)]
use std::{
    cell::Ref,
    collections::{HashMap, HashSet},
    ops::Index,
    str::FromStr,
    vec,
};

use aoc2023::lib::{self, ascii_to_string, print_line, print_lines};
use regex::Regex;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Reflection {
    fn value(self: &Self) -> usize {
        match self {
            &Reflection::Horizontal(x) => x * 100,
            &Reflection::Vertical(x) => x,
        }
    }
}

fn get_reflection(grid: &Vec<Vec<u8>>, skip_reflection: Option<Reflection>) -> Reflection {
    // i is indexing the gaps
    for i in 0..grid.len() - 1 {
        let reflection = Reflection::Horizontal(i + 1);
        if Some(reflection) == skip_reflection {
            continue;
        }
        let mut delta = 0;
        let mut matched = true;
        // println!("Trying horizontal reflection at {}", i);
        while delta <= i && i + delta + 1 < grid.len() {
            // println!(
            //     "Comparing: {} and {}",
            //     ascii_to_string(&grid[i - delta]),
            //     ascii_to_string(&grid[i + delta + 1])
            // );
            if grid[i - delta]
                .iter()
                .zip(grid[i + delta + 1].iter())
                .any(|(a, b)| a != b)
            {
                matched = false;
                break;
            }
            delta += 1;
        }

        if matched {
            println!("Found horizontal reflection at: {}", i);
            return reflection;
        }
    }

    // i is indexing the gaps
    for i in 0..grid[0].len() - 1 {
        let reflection = Reflection::Vertical(i + 1);
        if Some(reflection) == skip_reflection {
            continue;
        }

        let mut delta = 0;
        let mut matched = true;
        // println!("Trying vertical reflection at {}", i);
        while delta <= i && i + delta + 1 < grid[0].len() {
            // println!(
            //     "Comparing: {} and {}",
            //     ascii_to_string(&grid.iter().map(|l| l[i - delta]).collect::<Vec<u8>>()),
            //     ascii_to_string(&grid.iter().map(|l| l[i + delta + 1]).collect::<Vec<u8>>())
            // );
            if grid
                .iter()
                .map(|l| l[i - delta])
                .zip(grid.iter().map(|l| l[i + delta + 1]))
                .any(|(a, b)| a != b)
            {
                matched = false;
                break;
            }
            delta += 1;
        }

        if matched {
            println!("Found vertical reflection at: {}", i);
            return reflection;
        }
    }

    unreachable!("Did not find any reflections!")
}

#[allow(unused)]
fn part1() {
    let sum: usize = lib::read_byte_lines()
        .split(|line| line.is_empty())
        .map(|grid| get_reflection(&grid.to_vec(), None).value())
        .sum();
    println!("{}", sum);
}

fn find_smudge(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    // i is indexing the gaps
    for i in 0..grid.len() - 1 {
        let mut delta = 0;
        let mut mismatch: Option<(usize, usize)> = None;

        'outer: while delta <= i && i + delta + 1 < grid.len() {
            for j in 0..grid[0].len() {
                if grid[i - delta][j] != grid[i + delta + 1][j] {
                    match mismatch {
                        None => mismatch = Some((i - delta, j)),
                        Some(_) => {
                            mismatch = None;
                            break 'outer;
                        }
                    }
                }
            }
            delta += 1;
        }

        if let Some((i, j)) = mismatch {
            return (i, j);
        }
    }

    // i is indexing the gaps
    for i in 0..grid[0].len() - 1 {
        let mut delta = 0;
        let mut mismatch: Option<(usize, usize)> = None;

        'outer: while delta <= i && i + delta + 1 < grid[0].len() {
            for j in 0..grid.len() {
                if grid[j][i - delta] != grid[j][i + delta + 1] {
                    match mismatch {
                        None => mismatch = Some((j, i - delta)),
                        Some(_) => {
                            mismatch = None;
                            break 'outer;
                        }
                    }
                }
            }
            delta += 1;
        }

        if let Some((i, j)) = mismatch {
            return (i, j);
        }
    }

    unreachable!("Did not find any smudges!")
}

fn process_grid2(grid: &mut Vec<Vec<u8>>) -> usize {
    println!();
    print_lines(grid);

    let orig_reflection = get_reflection(grid, None);

    let (i, j) = find_smudge(grid);
    println!("Smudge found at ({}, {})", i, j);
    grid[i][j] = match grid[i][j] {
        b'.' => b'#',
        b'#' => b'.',
        _ => unreachable!(),
    };

    let value = get_reflection(grid, Some(orig_reflection)).value();
    println!("value = {}", value);
    value
}

#[allow(unused)]
fn part2() {
    let sum: usize = lib::read_byte_lines()
        .split(|line| line.is_empty())
        .map(|grid| process_grid2(&mut grid.to_vec()))
        .sum();
    println!("{}", sum);
}

pub fn main() {
    // part1();
    part2();
}
