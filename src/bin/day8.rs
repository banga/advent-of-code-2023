use std::collections::HashMap;

use aoc2023::lib;
use num::Integer;

pub fn part1() {
    let lines = lib::read_lines();
    let instr = &lines[0];

    let mut nodes = HashMap::new();
    let mut start = "AAA";
    let end = "ZZZ";
    for (_, line) in lines.iter().skip(2).enumerate() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        nodes.insert(parts[0], (&parts[2][1..4], &parts[3][0..3]));
    }
    println!("{} {} {}", start, end, instr);

    let mut step = 0;
    while start != end {
        let v = nodes.get(start).unwrap();
        start = match instr.as_bytes()[step % instr.len()] {
            b'L' => v.0,
            b'R' => v.1,
            _ => panic!("Unexpected {}", instr),
        };
        step += 1;
        println!("{} {}", step, start);
    }

    println!("{}", step);
}

fn get_min_cycle_length(nodes: &HashMap<&str, (&str, &str)>, instr: &[u8], start: &str) -> usize {
    let mut step = 0;
    let mut cur = start;
    while !cur.ends_with('Z') {
        let next = nodes.get(cur).unwrap();
        // println!("{} {} {:?}", cur, instr[step % instr.len()] as char, v);
        cur = match instr[step % instr.len()] {
            b'L' => next.0,
            b'R' => next.1,
            _ => panic!("Unexpected {:?}", instr),
        };
        step += 1;
    }
    step
}

pub fn part2() {
    let lines = lib::read_lines();
    let instr = lines[0].as_bytes();

    let mut nodes = HashMap::new();
    let mut starts = Vec::<&str>::new();
    for (_, line) in lines.iter().skip(2).enumerate() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        nodes.insert(parts[0], (&parts[2][1..4], &parts[3][0..3]));
        if parts[0].ends_with('A') {
            starts.push(parts[0]);
        }
    }

    print!("{:?}", nodes.get("DKB"));

    let mut cycle_lengths = Vec::<i64>::new();
    for start in starts {
        println!("start = {}", start);
        let cycle_length = get_min_cycle_length(&nodes, instr, start);
        cycle_lengths.push(cycle_length as i64);
    }
    println!("{:?}", cycle_lengths);

    let mut min_length = cycle_lengths[0];
    for l in cycle_lengths {
        min_length = min_length.lcm(&l);
    }
    println!("{}", min_length);
}

pub fn main() {
    // part1();
    part2();
}
