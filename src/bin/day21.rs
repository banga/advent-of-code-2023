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
    if i < 0 || i >= grid.len() as isize || j < 0 || j >= grid.len() as isize {
        return;
    }
    let key = (i, j, budget);
    if !visited.insert(key) {
        return;
    }
    let ch = grid[i as usize][j as usize];
    if ch == b'#' {
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

// Check if (si, sj) can be reached from (i, j) with a path of length exactly `length`
fn dp(
    grid: &Vec<Vec<u8>>,
    si: isize,
    sj: isize,
    i: isize,
    j: isize,
    length: isize,
    cache: &mut HashMap<(isize, isize, isize), bool>,
) -> bool {
    // assert!(length >= 0);
    if i < 0 || i >= grid.len() as isize || j < 0 || j >= grid[0].len() as isize {
        return false;
    }
    if grid[i as usize][j as usize] == b'#' {
        return false;
    }
    if length == 0 {
        return i == si && j == sj;
    }
    let key = (i, j, length);
    if let Some(value) = cache.get(&key) {
        return *value;
    }
    let neighbors = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
    let value = neighbors
        .iter()
        .any(|(ni, nj)| dp(grid, si, sj, *ni, *nj, length - 1, cache));
    cache.insert(key, value);
    value
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
fn part2() {}

pub fn main() {
    part1();
    // part2();
}
