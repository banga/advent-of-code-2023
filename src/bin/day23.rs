#![allow(dead_code, unused_imports)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc2023::lib::{self, print_lines};

fn dfs(grid: &mut Vec<Vec<u8>>, (i, j): (isize, isize), end: (isize, isize)) -> Option<isize> {
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    if !(0..h).contains(&i) || !(0..w).contains(&j) {
        return None;
    }

    if (i, j) == end {
        // print_lines(&grid);
        // println!();
        return Some(0);
    }

    let max_distance = match grid[i as usize][j as usize] {
        b'#' | b'O' => None,
        b'.' | b'^' | b'>' | b'v' | b'<' => {
            grid[i as usize][j as usize] = b'O';
            let neighbors = [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
            let result = neighbors
                .iter()
                .filter_map(|&n| dfs(grid, n, end))
                .reduce(|a, b| a.max(b));
            grid[i as usize][j as usize] = b'.';
            result
        }
        // b'^' => dfs(grid, (i - 1, j), end),
        // b'>' => dfs(grid, (i, j + 1), end),
        // b'v' => dfs(grid, (i + 1, j), end),
        // b'<' => dfs(grid, (i, j - 1), end),
        c => unreachable!("'{}'", c as char),
    };

    max_distance.map(|distance| distance + 1)
}

#[allow(dead_code)]
fn part1(mut grid: Vec<Vec<u8>>) {
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    // Hardcoded based on the input file
    let start = (0, 1);
    let end = (h - 1, w - 2);

    println!("{}", dfs(&mut grid, start, end).unwrap());
}

#[allow(dead_code)]
fn part2() {
    // println!("{}", num_fallen);
}

#[test]
fn test() {
    let input = r"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
    "
    .trim()
    .as_bytes()
    .split(|c| *c == b'\n')
    .map(|v| v.to_vec())
    .collect();
    part1(input);
}

pub fn main() {
    let input = lib::read_byte_lines();
    part1(input);
    // part2(input);
}
