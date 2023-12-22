#![allow(dead_code, unused_imports)]
use std::collections::{HashMap, HashSet};

use aoc2023::lib::{self};

fn dfs(
    grid: &Vec<Vec<u8>>,
    i: isize,
    j: isize,
    budget: isize,
    visited: &mut HashSet<(isize, isize, isize)>,
) {
    let key = (i, j, budget);
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;
    let gi = i.rem_euclid(h) as usize;
    let gj = j.rem_euclid(w) as usize;
    let ch = grid[gi as usize][gj as usize];
    if ch == b'#' {
        return;
    }
    if !visited.insert(key) {
        return;
    }
    if budget == 0 {
        return;
    }
    dfs(grid, i - 1, j, budget - 1, visited);
    dfs(grid, i + 1, j, budget - 1, visited);
    dfs(grid, i, j - 1, budget - 1, visited);
    dfs(grid, i, j + 1, budget - 1, visited);
}

#[allow(dead_code)]
fn part1() {
    let grid = lib::read_byte_lines();

    // lib::print_lines(&grid);
    // println!();

    let mut si = 0;
    let mut sj = 0;
    'outer: while si < grid.len() {
        while sj < grid[0].len() {
            if grid[si][sj] == b'S' {
                break 'outer;
            }
            sj += 1;
        }
        sj = 0;
        si += 1;
    }
    println!("si = {}, sj = {}", si, sj);

    let mut visited = HashSet::new();
    dfs(&grid, si as isize, sj as isize, 6, &mut visited);
    lib::print_lines(&grid);
    println!(
        "{}",
        visited
            .iter()
            .filter(|(i, j, dist)| *dist == 0 && grid[*i as usize][*j as usize] != b'#')
            .count()
    );
}

#[allow(dead_code)]
fn part2() {
    let grid = lib::read_byte_lines();

    let mut si = 0;
    let mut sj = 0;
    'outer: while si < grid.len() {
        while sj < grid[0].len() {
            if grid[si][sj] == b'S' {
                break 'outer;
            }
            sj += 1;
        }
        sj = 0;
        si += 1;
    }

    // This is all based on hints from reading other people's solutions:
    // - The number 26501365 is a 202300 * 131 + 65
    // - The output is periodic in the width of the grid, which is 131
    // - If you plot the values for any starting offset, say 65, you will see
    // that its second derivative is a constant = 31098.

    // So, the output is described by a quadratic function f on the budget size,
    // which can be calculated using the first three values. Then the final
    // answer is f(202300).
    for budget in (65..600).step_by(131) {
        let mut visited = HashSet::new();
        dfs(&grid, si as isize, sj as isize, budget, &mut visited);
        println!(
            "{} {}",
            budget,
            visited.iter().filter(|(_, _, dist)| *dist == 0).count(),
        );
    }
}

pub fn main() {
    // part1();
    part2();
}
