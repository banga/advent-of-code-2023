use std::collections::HashMap;

use aoc2023::lib;
use regex::Regex;

pub fn part1() {
    let lines: Vec<String> = lib::read_inputs();
    let mut sum = 0;
    for line in lines {
        let mut first_char: Option<char> = None;
        let mut last_char: Option<char> = None;
        for char in line.chars() {
            if char.is_numeric() {
                if first_char.is_none() {
                    first_char = Some(char);
                }
                last_char = Some(char);
            }
        }
        let digit = format!("{}{}", first_char.unwrap(), last_char.unwrap())
            .parse::<i32>()
            .unwrap();
        println!("digit: {}", digit);
        sum += digit;
    }
    println!("{}", sum);
}

pub fn part2() {
    let re_first = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    // Need to scan in reverse to handle overlapping matches like "twone"
    let re_last = Regex::new(r"([0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let char_to_digit = HashMap::from([
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("eno", '1'),
        ("owt", '2'),
        ("eerht", '3'),
        ("ruof", '4'),
        ("evif", '5'),
        ("xis", '6'),
        ("neves", '7'),
        ("thgie", '8'),
        ("enin", '9'),
    ]);

    let lines: Vec<String> = lib::read_inputs();
    let mut sum = 0;
    for line in lines {
        println!("{}", line);
        let reversed_line = line.chars().rev().collect::<String>();
        let first_char = re_first.find(&line).take().unwrap().as_str();
        let last_char = re_last.find(&reversed_line).take().unwrap().as_str();
        println!("  first: {} last: {}", first_char, last_char);
        let digit = format!(
            "{}{}",
            char_to_digit.get(first_char).unwrap(),
            char_to_digit.get(last_char).unwrap()
        )
        .parse::<i32>()
        .unwrap();
        println!("digit: {}", digit);
        sum += digit;
    }
    println!("{}", sum);
}

pub fn main() {
    part2();
}
