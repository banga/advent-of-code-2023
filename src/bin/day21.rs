use std::collections::HashMap;

use aoc2023::lib::{self, ascii_to_string};

fn dfs(grid: &mut Vec<Vec<u8>>, i: isize, j: isize, budget: i64) {
    if i < 0 || i >= grid.len() as isize || j < 0 || j >= grid.len() as isize {
        return;
    }
    let ch = &mut grid[i as usize][j as usize];
    if *ch == b'#' {
        return;
    }
    if budget == 0 {
        *ch = b'O';
        return;
    }
    dfs(grid, i - 1, j, budget - 1);
    dfs(grid, i + 1, j, budget - 1);
    dfs(grid, i, j - 1, budget - 1);
    dfs(grid, i, j + 1, budget - 1);
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
    // lib::print_lines(&grid);

    let budget = 64;
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if dp(
                &grid,
                si as isize,
                sj as isize,
                i as isize,
                j as isize,
                budget,
                &mut HashMap::new(),
            ) {
                count += 1;
                print!("O");
            } else {
                print!("{}", ascii_to_string(&grid[i][j..j + 1]));
            }
        }
        println!();
    }
    println!("{}", count);
}

#[allow(dead_code)]
fn part2() {}

pub fn main() {
    part1();
    // part2();
}
