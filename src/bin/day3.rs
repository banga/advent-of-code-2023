use std::collections::{HashMap, HashSet};

use aoc2023::lib;

pub fn part1() {
    let lines = lib::read_byte_lines();
    let mut sum = 0;
    for (lineno, line) in lines.iter().enumerate() {
        let mut is_part_no = false;
        let mut cur_no = String::new();
        for (charno, &char) in line.iter().enumerate() {
            match char {
                b'0'..=b'9' => {
                    cur_no.push(char.into());
                    if lineno > 0 {
                        let prev_line = &lines[lineno - 1];
                        is_part_no = is_part_no
                            || is_symbol(prev_line[charno])
                            || (charno > 0 && is_symbol(prev_line[charno - 1]))
                            || (charno < prev_line.len() - 1 && is_symbol(prev_line[charno + 1]));
                    }
                    is_part_no = is_part_no
                        || (charno > 0 && is_symbol(line[charno - 1]))
                        || (charno < line.len() - 1 && is_symbol(line[charno + 1]));
                    if lineno < lines.len() - 1 {
                        let next_line = &lines[lineno + 1];
                        is_part_no = is_part_no
                            || is_symbol(next_line[charno])
                            || (charno > 0 && is_symbol(next_line[charno - 1]))
                            || (charno < next_line.len() - 1 && is_symbol(next_line[charno + 1]));
                    }
                }
                _ => {
                    if cur_no.len() > 0 {
                        if is_part_no {
                            sum += cur_no.parse::<u32>().unwrap();
                        };
                        cur_no = String::new();
                        is_part_no = false;
                    }
                }
            }
        }
        if cur_no.len() > 0 {
            if is_part_no {
                sum += cur_no.parse::<u32>().unwrap();
            };
        }
    }
    println!("{}", sum);
}

pub fn part2() {
    let lines = lib::read_byte_lines();
    let mut numbers = Vec::new();
    let mut num_map = HashMap::<(usize, usize), usize>::new();

    for (lineno, line) in lines.iter().enumerate() {
        let mut cur_number = 0u32;
        for (charno, &char) in line.iter().enumerate() {
            match char {
                b'0'..=b'9' => {
                    cur_number = cur_number * 10 + (char - b'0') as u32;
                    num_map.insert((lineno, charno), numbers.len());
                }
                _ => {
                    if cur_number > 0 {
                        numbers.push(cur_number);
                        cur_number = 0;
                    }
                }
            }
        }
        if cur_number > 0 {
            numbers.push(cur_number);
        }
    }

    let mut sum = 0;
    for (lineno, line) in lines.iter().enumerate() {
        for (charno, &char) in line.iter().enumerate() {
            if char == b'*' {
                let mut num_indexes = HashSet::new();
                for i in lineno.saturating_sub(1)..=lineno + 1 {
                    for j in charno.saturating_sub(1)..=charno + 1 {
                        if let Some(num_index) = num_map.get(&(i, j)) {
                            num_indexes.insert(num_index);
                        }
                    }
                }
                if num_indexes.len() > 1 {
                    assert_eq!(num_indexes.len(), 2);
                    let mut product = 1;
                    for num_index in num_indexes {
                        product *= numbers[*num_index];
                    }
                    sum += product;
                }
            }
        }
    }

    println!("{}", sum);
}

fn is_symbol(char: u8) -> bool {
    match char {
        b'.' => false,
        b'0'..=b'9' => false,
        _ => true,
    }
}

pub fn main() {
    // part1();
    part2();
}
