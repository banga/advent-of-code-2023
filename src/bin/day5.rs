use aoc2023::lib;

pub fn part1() {
    let lines = lib::read_lines();
    let mut values: Vec<i64> = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut is_mapped: Vec<bool> = values.iter().map(|_| false).collect();

    for line in lines.iter().skip(1) {
        if line.len() == 0 {
            continue;
        }
        if line.starts_with(|b: char| b.is_alphabetic()) {
            is_mapped.fill(false);
            println!("{} {:?}", line, values);
            continue;
        }
        let parts: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let (to, from, len) = (parts[0], parts[1], parts[2]);
        for (index, value) in values.iter_mut().enumerate() {
            if is_mapped[index] {
                continue;
            }
            if *value >= from && *value < from + len {
                println!("{} -> {}", *value, to + (*value - from));
                *value = to + (*value - from);
                is_mapped[index] = true;
            }
        }
    }

    println!("{:?}", values);

    let min = values.iter().min().unwrap();
    println!("{}", min);
}

// Inclusive range
#[derive(Debug)]
struct Range(i64, i64);

impl Range {
    // [ (  ) ]
    // [ (  ] )
    // ( [  ) ]
    // ( )  [ ]
    // [ ]  ( )
    fn map<F>(&mut self, map_range: &Range, map_fn: F) -> Option<Vec<Range>>
    where
        F: Fn(i64) -> i64,
    {
        let min = self.0.max(map_range.0);
        let max = self.1.min(map_range.1);
        if max <= min {
            return None;
        }

        let mut non_overlapping_ranges = Vec::new();
        if self.0 < min {
            non_overlapping_ranges.push(Range(self.0, min - 1));
        }
        if self.1 > max {
            non_overlapping_ranges.push(Range(max + 1, self.1));
        }

        self.0 = map_fn(min);
        self.1 = map_fn(max);

        Some(non_overlapping_ranges)
    }
}

#[test]
fn test_split() {
    let mut r = Range(10, 20);
    println!("{:?} {:?}", r.map(&Range(0, 5), |i| i), r);
    println!("{:?} {:?}", r.map(&Range(25, 30), |i| i), r);
    println!("{:?} {:?}", r.map(&Range(10, 20), |i| i), r);
    println!("{:?} {:?}", r.map(&Range(11, 20), |i| i), r);
    println!("{:?} {:?}", r.map(&Range(11, 19), |i| i), r);
    println!("{:?} {:?}", r.map(&Range(10, 18), |i| i), r);
    println!("{:?} {:?}", r.map(&Range(12, 20), |i| i), r);
    println!("{:?} {:?}", r.map(&Range(14, 16), |i| i), r);
}

pub fn part2() {
    let lines = lib::read_lines();
    let values: Vec<i64> = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let mut ranges: Vec<Range> = Vec::new();
    for i in (0..values.len()).step_by(2) {
        ranges.push(Range(values[i], values[i] + values[i + 1] - 1));
    }
    println!("Ranges: {:?}", ranges);
    let mut is_mapped: Vec<bool> = ranges.iter().map(|_| false).collect();

    for line in lines.iter().skip(1) {
        if line.len() == 0 {
            continue;
        }
        if line.starts_with(|b: char| b.is_alphabetic()) {
            is_mapped.fill(false);
            continue;
        }
        let parts: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let (to, from, len) = (parts[0], parts[1], parts[2]);
        let map_range = Range(from, from + len - 1);
        let map_fn = |value: i64| to + (value - from);
        for index in 0..ranges.len() {
            if is_mapped[index] {
                continue;
            }
            let range = &mut ranges[index];
            if let Some(non_overlapping_ranges) = range.map(&map_range, map_fn) {
                is_mapped[index] = true;
                ranges.extend(non_overlapping_ranges);
                is_mapped.resize(ranges.len(), false);
            }
        }
    }

    println!("{:?}", ranges);

    let min = ranges.iter().map(|r| r.0).min().unwrap();
    println!("{}", min);
}

pub fn main() {
    // part1();
    part2();
}
