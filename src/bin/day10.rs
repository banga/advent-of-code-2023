use std::collections::{HashMap, HashSet};

use aoc2023::lib;

fn follow_path<F>(grid: &Vec<Vec<u8>>, start: &(i32, i32), delta: &(i32, i32), visitor: &mut F)
where
    F: FnMut((i32, i32), (i32, i32)) -> (),
{
    let (mut x, mut y) = start;
    let (mut dx, mut dy) = delta;

    println!(
        "Starting path at {}",
        String::from_utf8(vec![grid[(y + dy) as usize][(x + dx) as usize]]).unwrap()
    );

    loop {
        visitor((x, y), (dx, dy));

        x += dx;
        y += dy;
        let ch = grid[y as usize][x as usize];
        let ch_str = String::from_utf8(vec![ch]).unwrap();
        (dx, dy) = match ch {
            b'S' => return,
            b'-' => (dx, dy),
            b'|' => (dx, dy),
            b'F' => (-dy, -dx), // (-1, 0) -> (0, +1) and (0, -1) -> (+1, 0)
            b'J' => (-dy, -dx), // (+1, 0) -> (0, -1) and (0, +1) -> (-1, 0)
            b'7' => (dy, dx),   // (-1, 0) -> (0, -1) and (0, +1) -> (+1, 0)
            b'L' => (dy, dx),   // (-1, 0) -> (0, -1) and (0, +1) -> (+1, 0)
            _ => unreachable!("Unexpected character '{}' at {:?}", ch_str, (x, y),),
        };
    }
}

fn get_start(grid: &Vec<Vec<u8>>) -> (i32, i32) {
    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == b'S' {
                return (x as i32, y as i32);
            }
        }
    }
    unreachable!()
}

fn get_path_candidates(grid: &Vec<Vec<u8>>, start: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut deltas = Vec::new();
    let (width, height) = (grid[0].len() as i32, grid.len() as i32);
    for (delta, allowed_dirs) in [
        ((-1, 0), "-LF"),
        ((1, 0), "-7J"),
        ((0, -1), "|7F"),
        ((0, 1), "|LJ"),
    ] {
        let (x, y) = (start.0 + delta.0, start.1 + delta.1);
        if x >= 0 && x < width && y >= 0 && y < height {
            let c = grid[y as usize][x as usize] as char;
            if allowed_dirs.contains(c) {
                deltas.push(delta);
            }
        }
    }
    deltas
}

#[allow(dead_code)]
fn part1() {
    let grid = lib::read_byte_lines();
    let start = get_start(&grid);

    let mut distances = HashMap::new();
    for delta in get_path_candidates(&grid, &start) {
        let mut dist = 0;
        follow_path(&grid, &start, &delta, &mut |pos, _| {
            distances.insert(pos, (*distances.get(&pos).unwrap_or(&dist)).min(dist));
            dist += 1;
        });
    }

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if let Some(_) = distances.get(&(x as i32, y as i32)) {
                print!("*");
            } else {
                print!("{}", String::from_utf8(vec![*char]).unwrap());
            }
        }
        println!("");
    }

    let max_dist = distances.values().max();
    println!("{:?}", max_dist);
}

fn part2() {
    let grid = lib::read_byte_lines();
    let start = get_start(&grid);

    let mut path = HashSet::new();
    let delta = get_path_candidates(&grid, &start).first().unwrap().clone();
    let mut deltas = HashMap::new();
    follow_path(&grid, &start, &delta, &mut |pos, delta| {
        path.insert(pos);
        deltas.insert(pos, delta);
    });

    let mut candidates = HashSet::new();
    for (y, line) in grid.iter().enumerate() {
        println!("");
        let mut winding_number = 0;
        for (x, char) in line.iter().enumerate() {
            if path.contains(&(x as i32, y as i32)) {
                if "|LJ".contains(*char as char) {
                    winding_number += 1;
                }
            } else if winding_number % 2 == 1 {
                candidates.insert((x, y));
            }
            print!("{} ", winding_number);
        }
    }
    println!("");

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if candidates.contains(&(x, y)) {
                print!("I");
            } else if path.contains(&(x as i32, y as i32)) {
                print!("{}", String::from_utf8(vec![*char]).unwrap());
            } else {
                print!("O");
            }
        }
        println!("");
    }
    println!("{}", candidates.len());
}

pub fn main() {
    // part1();
    part2();
}
