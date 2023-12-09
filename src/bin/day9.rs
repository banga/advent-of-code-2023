use aoc2023::lib;
use num::Integer;

fn get_next_in_series(series: &Vec<i64>) -> i64 {
    let mut deltas: Vec<Vec<i64>> = [series.clone()].into();
    while deltas.last().unwrap().iter().any(|&d| d != 0) {
        let current = deltas.last().unwrap();
        let next: Vec<i64> = current
            .iter()
            .zip(current.iter().skip(1))
            .map(|(x, y)| y - x)
            .collect();
        deltas.push(next);
    }

    println!("{:?}", deltas);

    deltas.last_mut().unwrap().push(0);
    for i in (0..deltas.len()-2).rev() {
        let prev = *deltas[i].last().unwrap();
        let delta = *deltas[i + 1].last().unwrap();
        deltas[i].push(prev + delta);
    }

    println!("{:?}", deltas);
    
    *deltas[0].last().unwrap()
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
