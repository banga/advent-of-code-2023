#![allow(dead_code, unused_imports)]
use rand::seq::IteratorRandom;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc2023::lib::{self, print_lines};

// Implements https://en.wikipedia.org/wiki/Karger%27s_algorithm, which gives a
// randomized algorithm to return a min cut
fn get_min_cut(mut vertices: HashSet<String>, mut edges: Vec<(String, String)>) -> (usize, isize) {
    let mut vertex_sizes = HashMap::new();

    while vertices.len() > 2 && edges.len() > 0 {
        let idx = rand::random::<usize>() % edges.len();
        let (a, b) = edges.swap_remove(idx);
        let ab: String = [a.as_str(), b.as_str()].join("-");
        vertices.remove(&a);
        vertices.remove(&b);
        vertices.insert(ab.clone());

        // println!("Removing edge ({}, {})", a, b);

        vertex_sizes.insert(
            ab.to_string(),
            vertex_sizes.get(&a).unwrap_or(&1) + vertex_sizes.get(&b).unwrap_or(&1),
        );
        vertex_sizes.remove(&a);
        vertex_sizes.remove(&b);

        for i in 0..edges.len() {
            if edges[i].0 == *a || edges[i].0 == *b {
                edges[i].0 = ab.clone();
            }
            if edges[i].1 == *a || edges[i].1 == *b {
                edges[i].1 = ab.clone();
            }
        }
        edges = edges
            .iter()
            .filter_map(|edge| {
                if edge.0 == edge.1 {
                    None
                } else {
                    Some(edge.clone())
                }
            })
            .collect();
    }

    (edges.len(), vertex_sizes.values().fold(1, |x, y| (x * y)))
}

#[allow(dead_code)]
fn part1(lines: Vec<String>) {
    // println!("digraph G {{");
    // for line in &lines {
    //     let (src, dests) = line.split_once(':').unwrap();
    //     for dest in dests.trim().split_ascii_whitespace() {
    //         println!("{} -> {}", src, dest);
    //     }
    // }
    // println!("}}");

    let mut edges = vec![];
    let mut vertices = HashSet::new();
    let mut vertex_sizes = HashMap::new();
    for line in &lines {
        let (src, dests) = line.split_once(':').unwrap();
        vertices.insert(src.to_string());
        vertex_sizes.insert(src.to_string(), 1);
        for dest in dests.trim().split_ascii_whitespace() {
            edges.push((src.to_string(), dest.to_string()));
            vertices.insert(dest.to_string());
            vertex_sizes.insert(dest.to_string(), 1);
        }
    }
    // println!("edges: {:?}\nvertices: {:?}", edges, vertices);

    loop {
        let (num_edges, size_product) = get_min_cut(vertices.clone(), edges.clone());
        if num_edges == 3 {
            println!("{}", size_product);
            break;
        }
        println!("num_edges = {}, retrying", num_edges);
    }
}

#[allow(dead_code)]
fn part2() {
    // println!("{}", num_fallen);
}

#[test]
fn test() {
    let input = r"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"
    .trim()
    .split('\n')
    .map(|s| s.to_string())
    .collect();
    part1(input);
}

pub fn main() {
    let input = lib::read_lines();
    part1(input);
    // part2(input);
}
