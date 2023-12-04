use std::collections::HashSet;

use aoc2023::lib;

pub fn part1() {
    let lines = lib::read_lines();
    let mut sum = 0;
    for line in lines {
        let (_, suffix) = line.split_once(':').unwrap();
        let (winning_numbers_str, ticket_numbers_str) = suffix.split_once('|').unwrap();
        let mut winning_numbers = HashSet::<i32>::new();
        for winning_number in winning_numbers_str.split_ascii_whitespace() {
            winning_numbers.insert(winning_number.parse().unwrap());
        }
        let mut matches = 0;
        for ticket_number in ticket_numbers_str.split_ascii_whitespace() {
            if winning_numbers.contains(&ticket_number.parse::<i32>().unwrap()) {
                matches += 1;
            }
        }
        if matches > 0 {
            sum += 1i32 << (matches - 1);
        }
    }
    println!("{}", sum);
}

pub fn part2() {
    let lines = lib::read_lines();
    let mut card_counts = Vec::new();
    for _ in 0..lines.len() {
        card_counts.push(1);
    }
    card_counts.fill(1);
    for (card_no, line) in lines.iter().enumerate() {
        let (_, suffix) = line.split_once(':').unwrap();
        let (winning_numbers_str, ticket_numbers_str) = suffix.split_once('|').unwrap();
        let mut winning_numbers = HashSet::<i32>::new();
        for winning_number in winning_numbers_str.split_ascii_whitespace() {
            winning_numbers.insert(winning_number.parse().unwrap());
        }
        let mut matches = 0;
        for ticket_number in ticket_numbers_str.split_ascii_whitespace() {
            if winning_numbers.contains(&ticket_number.parse::<i32>().unwrap()) {
                matches += 1;
            }
        }
        for i in 1..=matches {
            card_counts[card_no + i] += card_counts[card_no];
        }
    }
    let sum: i32 = card_counts.iter().sum();
    println!("{}", sum);
}

pub fn main() {
    // part1();
    part2();
}
