use std::collections::HashSet;

use aoc2023::lib;

#[allow(dead_code)]
fn part1() {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in lib::read_byte_lines() {
        grid.push(line.clone());
        // Duplicate empty rows
        if line.iter().all(|c| *c == b'.') {
            grid.push(line.clone());
        }
    }

    // Duplicate empty columns
    let mut i = 0;
    while i < grid[0].len() {
        if grid.iter().all(|line| line[i] == b'.') {
            for line in grid.iter_mut() {
                line.insert(i, b'.');
            }
            i += 1;
        }
        i += 1;
    }

    lib::print_lines(&grid);

    let mut galaxies = HashSet::<(usize, usize)>::new();
    for (i, line) in grid.iter().enumerate() {
        galaxies.extend(
            line.iter()
                .enumerate()
                .filter(|(_, &char)| char == b'#')
                .map(|(j, _)| (i, j)),
        );
    }

    let mut sum: u32 = 0;
    for start in &galaxies {
        for end in &galaxies {
            sum += start.1.abs_diff(end.1) as u32 + start.0.abs_diff(end.0) as u32;
        }
    }

    println!("{}", sum / 2);
}

fn part2() {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in lib::read_byte_lines() {
        grid.push(line);
    }
    lib::print_lines(&grid);

    let empty_rows: Vec<usize> = (0..=grid.len() - 1)
        .filter(|i| grid[*i].iter().all(|c| *c == b'.'))
        .collect();
    let empty_cols: Vec<usize> = (0..=grid[0].len() - 1)
        .filter(|j| grid.iter().all(|line| line[*j] == b'.'))
        .collect();
    let empty_width = 1000_000;

    let mut galaxies = Vec::<(usize, usize)>::new();
    for (i, line) in grid.iter().enumerate() {
        galaxies.extend(
            line.iter()
                .enumerate()
                .filter(|(_, &char)| char == b'#')
                .map(|(j, _)| (i, j)),
        );
    }
    println!("empty_rows = {:?}", empty_rows);
    println!("empty_cols = {:?}", empty_cols);

    let mut sum: u64 = 0;
    for (i, start) in galaxies.iter().enumerate() {
        for end in &galaxies[i + 1..] {
            let dist = end.0.abs_diff(start.0) + end.1.abs_diff(start.1);
            let min = (start.0.min(end.0), start.1.min(end.1));
            let max = (start.0.max(end.0), start.1.max(end.1));
            let num_empty_rows = empty_rows
                .iter()
                .filter(|&&i| i > min.0 && i < max.0)
                .count();
            let num_empty_cols = empty_cols
                .iter()
                .filter(|&&j| j > min.1 && j < max.1)
                .count();
            // println!(
            //     "{:?} -> {:?}, dist = {}, empty_rows = {}, empty_cols = {}",
            //     start, end, dist, num_empty_rows, num_empty_cols
            // );

            sum += dist as u64 + ((num_empty_rows as u64 + num_empty_cols as u64) * (empty_width - 1));
        }
    }

    println!("{}", sum);
}

pub fn main() {
    // part1();
    part2();
}
