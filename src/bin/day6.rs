use aoc2023::lib;

fn count_wins(time: i64, distance: i64) -> i64 {
    let mut wins = 0;
    println!("t={} d={}", time, distance);
    for x in 1..=time - 1 {
        if x * (time - x) > distance {
            // println!(" x = {}, d = {}", x, x * (time - x));
            wins += 1;
        }
    }
    wins
}

pub fn part1() {
    let lines = lib::read_lines();
    let times: Vec<i64> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let distances: Vec<i64> = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut product = 1;
    for (&time, &distance) in times.iter().zip(distances.iter()) {
        product *= count_wins(time, distance);
    }

    println!("{}", product);
}

pub fn part2() {
    let lines = lib::read_lines();
    let time: i64 = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();
    let distance: i64 = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();
    let product = count_wins(time, distance);

    println!("{}", product);
}

pub fn main() {
    // part1();
    part2();
}
