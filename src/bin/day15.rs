#![allow(unused_imports)]
use std::{
    cell::Ref,
    cmp::Reverse,
    collections::{hash_map::DefaultHasher, HashMap, HashSet, LinkedList},
    hash::Hasher,
    ops::Index,
    str::FromStr,
    vec,
};

use aoc2023::lib::{self, ascii_to_string, print_line, print_lines};
use regex::Regex;

fn hash(s: &[u8]) -> usize {
    let mut value: usize = 0;
    for c in s {
        value = ((value + *c as usize) * 17) % 256;
    }
    value
}

#[allow(unused)]
fn part1() {
    let line = &lib::read_byte_lines()[0];
    let sum: usize = line.split(|c| *c == b',').map(|part| hash(part)).sum();
    println!("{}", sum);
}

#[allow(unused)]
fn part2() {
    let line = &lib::read_byte_lines()[0];
    let mut boxes: [Vec<(&[u8], u8)>; 256] = [0; 256].map(|_| Vec::new());
    for part in line.split(|c| *c == b',') {
        let mut op;
        let mut lens_name;
        let mut box_no;
        let mut focal_length: u8 = 0;
        match part.last().unwrap() {
            b'-' => {
                lens_name = &part[0..part.len() - 1];
                op = b'-';
            }
            c => {
                lens_name = &part[0..part.len() - 2];
                focal_length = c - b'0';
                op = b'=';
            }
        };
        box_no = hash(lens_name);
        println!(
            "\n{}: Box {} op='{}' f={}",
            ascii_to_string(part),
            box_no,
            ascii_to_string(&[op]),
            focal_length
        );

        let mut lenses = &mut boxes[box_no];
        match op {
            b'-' => {
                if let Some(index) = lenses.iter().position(|lens| lens.0 == lens_name) {
                    lenses.remove(index);
                }
            }
            b'=' => {
                if let Some(index) = lenses.iter().position(|lens| lens.0 == lens_name) {
                    lenses[index].1 = focal_length;
                } else {
                    lenses.push((&lens_name, focal_length));
                }
            }
            _ => unreachable!(),
        }
        for (i, lenses) in boxes.iter().enumerate() {
            if !lenses.is_empty() {
                println!(
                    "{}. {:?}",
                    i,
                    lenses
                        .iter()
                        .map(|lens| (ascii_to_string(lens.0), lens.1))
                        .collect::<Vec<_>>()
                );
            }
        }
    }

    let total: usize = boxes
        .iter()
        .enumerate()
        .map(|(i, lenses)| {
            (i + 1)
                * lenses
                    .iter()
                    .enumerate()
                    .map(|(j, lens)| (j + 1) * lens.1 as usize)
                    .sum::<usize>()
        })
        .sum();
    println!("{}", total);
}

pub fn main() {
    // part1();
    part2();
}
