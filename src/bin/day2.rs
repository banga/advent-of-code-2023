use aoc2023::lib;

pub fn part1() {
    let lines = lib::read_lines();
    let mut sum = 0;
    for line in lines {
        let (prefix, suffix) = line.split_once(':').unwrap();
        let mut is_valid = true;
        for subset in suffix.split(';') {
            for cube in subset.split(',') {
                let (count, color) = cube.trim().split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();
                is_valid = is_valid
                    && match color {
                        "red" => count <= 12,
                        "green" => count <= 13,
                        "blue" => count <= 14,
                        _ => panic!("Unexpected color {}", color),
                    };
            }
        }
        if is_valid {
            let (_, id) = prefix.split_once(' ').unwrap();
            let id = id.parse::<u32>().unwrap();
            sum += id;
        }
    }
    println!("{}", sum);
}

pub fn part2() {
    let lines = lib::read_lines();
    let mut sum = 0;
    for line in lines {
        let (_, suffix) = line.split_once(':').unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for subset in suffix.split(';') {
            for cube in subset.split(',') {
                let (count, color) = cube.trim().split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();
                match color {
                    "red" => red = red.max(count),
                    "green" => green = green.max(count),
                    "blue" => blue = blue.max(count),
                    _ => panic!("Unexpected color {}", color),
                };
            }
        }
        sum += red * green * blue;
    }
    println!("{}", sum);
}

pub fn main() {
    part2();
}
