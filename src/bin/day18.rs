#![allow(unused_imports)]
use std::{
    collections::{hash_map::DefaultHasher, BinaryHeap, HashMap, HashSet},
    fmt::{Display, Write},
    vec,
};

use aoc2023::lib::{self, ascii_to_string, print_line, print_lines, Grid, Point};
use num::{complex::Complex32, Complex};
use regex::Regex;

fn get_instructions1(grid: &Vec<Vec<u8>>) -> Vec<(u8, isize)> {
    grid.iter()
        .map(|line| {
            let parts = line.split(|c| *c == b' ').collect::<Vec<&[u8]>>();
            let direction = match parts[0][0] {
                b'R' => 0,
                b'D' => 1,
                b'L' => 2,
                b'U' => 3,
                _ => unreachable!(),
            };

            let count = ascii_to_string(parts[1]).parse::<usize>().unwrap() as isize;
            (direction, count)
        })
        .collect()
}

fn get_hole_size1(instructions: &Vec<(Point<isize>, isize)>) -> usize {
    let mut p: Point<isize> = Point::from(0, 0);
    let mut min = p.clone();
    let mut max = p.clone();
    for (direction, count) in instructions {
        p += &(direction * count);

        min = min.min(&p);
        max = max.max(&p);
    }

    let bounds = &(&max - &min) + &Point::from(1, 1);
    println!("{} -> {} = {}", min, max, bounds);

    let mut grid = Grid::new(bounds.x, bounds.y, b'.');
    p = Point::from(0, 0);
    for (direction, count) in instructions {
        for _ in 0..*count {
            p += direction;
            grid.set(&(&p - &min), b'#');
        }
    }

    println!("{}", grid);

    let mut count = 0;
    for (y, line) in grid.values.iter().enumerate() {
        let mut inside = false;
        let mut edge_count = 0;
        let mut inside_count = 0;
        let mut x = 0;
        while x < line.len() {
            if line[x] == b'#' {
                let start_x = x;
                while x < line.len() - 1 && line[x + 1] == b'#' {
                    x += 1;
                }

                if x == start_x {
                    inside = !inside;
                } else {
                    // Horizontal edge
                    let prev_line = if y > 0 {
                        &grid.values[y - 1]
                    } else {
                        &grid.values[y + 1]
                    };
                    if prev_line[x] != prev_line[start_x] {
                        inside = !inside;
                    }
                }

                edge_count += x - start_x + 1;
            } else if inside {
                inside_count += 1;
            }

            x += 1;
        }
        count += edge_count + inside_count;
        // println!("{} {}", edge_count, inside_count);
    }
    count
}

#[allow(unused)]
fn part1() {
    // let count = get_hole_size1(&get_instructions1(&lib::read_byte_lines()));
    // println!("{}", count);
}

#[test]
fn test() {
    let count = get_hole_size2(&get_instructions1(
        &r"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"
            .trim()
            .split('\n')
            .map(|l| l.trim().as_bytes().to_vec())
            .collect(),
    ));
    assert_eq!(count, 62);
}

fn get_instructions2(grid: &Vec<Vec<u8>>) -> Vec<(u8, isize)> {
    grid.iter()
        .map(|line| {
            let color = &line.rsplit(|c| *c == b' ').next().unwrap();
            let direction = color[color.len() - 2] - b'0';

            let color = &color[2..color.len() - 2];
            let distance = isize::from_str_radix(&ascii_to_string(color), 16).unwrap();

            (direction, distance)
        })
        .collect()
}

#[derive(Debug)]
struct Edge {
    // Keep start < end
    start: isize,
    end: isize,
    cross_axis: isize,
}

impl Edge {
    fn len(&self) -> isize {
        self.end - self.start + 1
    }

    // Assumes self and other are perpendicular
    // fn intersects(&self, other: &Edge) -> bool {
    //     (self.cross_axis >= other.start && self.cross_axis <= other.end)
    //         && (other.cross_axis >= self.start && other.cross_axis <= self.end)
    // }

    fn format_horizontal(&self) -> String {
        format!(
            "({}, {}) -> ({}, {})",
            self.start, self.cross_axis, self.end, self.cross_axis
        )
    }
    fn format_vertical(&self) -> String {
        format!(
            "({}, {}) -> ({}, {})",
            self.cross_axis, self.start, self.cross_axis, self.end
        )
    }
}

fn get_hole_size2(instructions: &Vec<(u8, isize)>) -> usize {
    let mut vertical_edges = vec![];
    let mut horizontal_edges = vec![];
    let mut x = 0;
    let mut y = 0;

    for (direction, distance) in instructions {
        match direction {
            0 => horizontal_edges.push(Edge {
                start: x,
                end: x + distance,
                cross_axis: y,
            }), // R
            1 => vertical_edges.push(Edge {
                start: y,
                end: y + distance,
                cross_axis: x,
            }), // D
            2 => horizontal_edges.push(Edge {
                start: x - distance,
                end: x,
                cross_axis: y,
            }), // L
            3 => vertical_edges.push(Edge {
                start: y - distance,
                end: y,
                cross_axis: x,
            }), // U
            _ => unreachable!(),
        };

        match direction {
            0 => x += distance, // R
            1 => y += distance, // D
            2 => x -= distance, // L
            3 => y -= distance, // T
            _ => unreachable!(),
        }
    }

    horizontal_edges.sort_by_key(|e| e.cross_axis);
    vertical_edges.sort_by_key(|e| e.cross_axis);

    for e in horizontal_edges.iter().chain(vertical_edges.iter()) {
        assert!(e.start < e.end);
    }

    let mut vertical_xs: Vec<isize> = vertical_edges.iter().map(|e| e.cross_axis).collect();
    vertical_xs.dedup();

    let mut count = 0;
    for (&x, &next_x) in vertical_xs.iter().zip(vertical_xs.iter().skip(1)) {
        let intersecting_edges = horizontal_edges
            .iter()
            .filter(|horizontal_edge| horizontal_edge.start <= x && horizontal_edge.end > x)
            .collect::<Vec<_>>();

        println!("x = {}, next_x = {}:", x, next_x);
        for e in &intersecting_edges {
            println!("  {}", e.format_horizontal());
        }

        let mut total_height = 0;
        for pairs in (&intersecting_edges).chunks(2) {
            let height = pairs[1].cross_axis - pairs[0].cross_axis + 1;
            println!("height = {}", height);
            total_height += height;
        }

        let width = next_x - x;
        count += width * total_height;
        println!(
            "width = {}, total_height = {}, count = {}",
            width, total_height, count
        );
        println!();
    }

    count as usize
}

fn get_hole_size3(instructions: &Vec<(u8, isize)>) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut xs = vec![];
    let mut ys = vec![];

    let mut perimeter = 0;

    for (direction, distance) in instructions {
        xs.push(x);
        ys.push(y);
        perimeter += distance;

        match direction {
            0 => x += distance, // R
            1 => y += distance, // D
            2 => x -= distance, // L
            3 => y -= distance, // U
            _ => unreachable!(),
        }
    }

    assert_eq!(x, 0);
    assert_eq!(y, 0);

    // "Shoelace" formula
    let mut inner_area = 0;
    for i in 0..xs.len() {
        let prev = (i + xs.len() - 1) % xs.len();
        let next = (i + 1) % xs.len();
        inner_area += ys[i] * (xs[prev] - xs[next]);
    }
    inner_area = inner_area / 2;

    // Pick's theorem. TODO: try to understand this better
    let outer_area = inner_area + perimeter / 2 + 1;

    outer_area as usize
}

#[allow(unused)]
fn part2() {
    let count = get_hole_size3(&get_instructions1(&lib::read_byte_lines()));
    println!("{}", count);
}

pub fn main() {
    // part1();
    part2();
}
