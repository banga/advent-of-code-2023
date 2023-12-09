use aoc2023::lib;
use num::Integer;

fn get_next_in_series(series: &Vec<i64>) -> i64 {
    let mut deltas: Vec<i64> = series.clone();
    let mut length = deltas.len();

    while deltas[0..length-1].iter().any(|&d| d != 0) {
        for i in 0..=length - 2 {
            deltas[i] = deltas[i + 1] - deltas[i];
        }
        length -= 1;
    }

    println!("{:?}", deltas);
    
    deltas.iter().sum()
}

fn part1() {
    let mut sum = 0;
    for line in lib::read_lines() {
        let series: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        sum += get_next_in_series(&series);
    }
    println!("{}", sum);
}

fn part2() {
    let mut sum = 0;
    for line in lib::read_lines() {
        let mut series: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        series.reverse();
        sum += get_next_in_series(&series);
    }
    println!("{}", sum);
}

pub fn main() {
    // part1();
    part2();
}
