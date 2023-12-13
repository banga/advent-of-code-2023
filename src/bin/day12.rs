#![allow(unused_imports)]
use std::{
    collections::{HashMap, HashSet},
    ops::Index,
    str::FromStr,
    vec,
};

use aoc2023::lib::{self, ascii_to_string};
use regex::Regex;

fn generate_arrangements(pattern: &[u8]) -> Vec<Vec<u8>> {
    let mut q = vec![pattern.to_vec()];
    let mut arrangements: Vec<Vec<u8>> = vec![];
    while let Some(p) = q.pop() {
        let mut found = true;
        for i in 0..p.len() {
            if p[i] == b'?' {
                found = false;

                let mut p1 = p.clone();
                p1[i] = b'#';

                let mut p2 = p.clone();
                p2[i] = b'.';

                // println!(
                //     "p={} p1={} p2={}",
                //     ascii_to_string(&p),
                //     ascii_to_string(&p1),
                //     ascii_to_string(&p2)
                // );

                q.push(p1);
                q.push(p2);
                break;
            }
        }

        if found {
            arrangements.push(p.to_vec());
        }
    }

    arrangements
}

fn is_valid_arragement(arrangement: &[u8], expected_counts: &[usize]) -> bool {
    let counts: Vec<usize> = arrangement
        .split(|c| *c == b'.')
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect();
    counts == expected_counts
}

#[allow(unused)]
fn part1() {
    let mut sum = 0;
    for line in lib::read_lines() {
        let (s, c) = line.split_once(' ').unwrap();
        let pattern: &[u8] = s.as_bytes();
        let counts: Vec<usize> = c.split(',').map(|s| s.parse().unwrap()).collect();

        println!("{:?} {:?}", ascii_to_string(pattern), &counts,);

        let mut pat_sum = 0;
        for arrangement in generate_arrangements(pattern) {
            if is_valid_arragement(&arrangement, &counts) {
                // println!("{}", ascii_to_string(&arrangement),);
                pat_sum += 1;
            }
        }
        println!("{}", pat_sum);
        sum += pat_sum;
    }
    println!("{}", sum);
}

fn count_arrangements(
    pattern: &[u8],
    pi: usize,
    counts: &[usize],
    ci: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(value) = cache.get(&(pi, ci)) {
        return *value;
    }
    let value = count_arrangements_inner(pattern, pi, counts, ci, cache);
    cache.insert((pi, ci), value);
    value
}

fn count_arrangements_inner(
    pattern: &[u8],
    pi: usize,
    counts: &[usize],
    ci: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if pi == pattern.len() {
        return if ci == counts.len() { 1 } else { 0 };
    }
    if ci == counts.len() {
        // The rest of the pattern must be empty
        return if pattern[pi..].iter().all(|c| *c != b'#') {
            1
        } else {
            0
        };
    }

    let ch = pattern[pi];
    let count = counts[ci];
    let choices: &[u8] = match ch {
        b'.' => &[b'.'],
        b'#' => &[b'#'],
        b'?' => &[b'.', b'#'],
        _ => unreachable!(),
    };

    choices
        .iter()
        .map(|choice| {
            match choice {
                b'.' => count_arrangements(pattern, pi + 1, counts, ci, cache),
                b'#' => {
                    // Consume the next # block
                    let mut i = 0;
                    loop {
                        if i == count {
                            // Finished the block
                            break;
                        }
                        if pi + i == pattern.len() {
                            // Reached the end of the pattern
                            return 0;
                        }
                        if pattern[pi + i] == b'.' {
                            // Reached a '.', the block must end
                            break;
                        }
                        i += 1;
                    }

                    if i < count {
                        // Did not see enough #s, fail
                        return 0;
                    }
                    if pi + i == pattern.len() {
                        if ci == counts.len() - 1 {
                            // Pattern matched entirely
                            return 1;
                        } else {
                            return 0;
                        }
                    }
                    if pattern[pi + i] == b'#' {
                        // The block is longer than `count`, fail
                        return 0;
                    }
                    // Force the next character to be a '.'
                    count_arrangements(pattern, pi + i + 1, counts, ci + 1, cache)
                }
                _ => unreachable!(),
            }
        })
        .sum()
}

#[allow(unused)]
fn part2() {
    let mut sum = 0;
    for line in lib::read_lines() {
        let (s, c) = line.split_once(' ').unwrap();
        let orig_pattern: &[u8] = s.as_bytes();
        let orig_counts: Vec<usize> = c.split(',').map(|s| s.parse().unwrap()).collect();

        let mut pattern = vec![];
        let mut counts = vec![];
        for i in 0..5 {
            pattern.extend(orig_pattern.clone());
            pattern.push(b'?');
            counts.extend(orig_counts.clone());
        }
        pattern.pop();

        println!("{:?} {:?}", ascii_to_string(&pattern), &counts,);

        let mut cache = HashMap::new();
        let pat_sum = count_arrangements(&pattern, 0, &counts, 0, &mut cache);
        println!("{}", pat_sum);
        sum += pat_sum;
    }
    println!("{}", sum);
}

pub fn main() {
    // part1();
    part2();
}
