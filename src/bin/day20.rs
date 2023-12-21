use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use aoc2023::lib;

#[derive(Debug)]
struct Pulse {
    source: Option<String>,
    target: String,
    is_high: bool,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(source) = &self.source {
            f.write_fmt(format_args!(
                "{}--{}-->{}",
                source, self.is_high, self.target
            ))
        } else {
            f.write_fmt(format_args!("--{}-->{}", self.is_high, self.target))
        }
    }
}

#[derive(Debug)]
enum ModuleState {
    FlipFlop { is_on: bool },
    Conjunction { memory: HashMap<String, bool> },
    Broadcaster,
    Sink,
}

#[derive(Debug)]
struct Module {
    name: String,
    state: ModuleState,
    targets: Vec<String>,
}

impl Module {
    fn initiaze_in_edges(&mut self, sources: &Vec<String>) {
        match &mut self.state {
            ModuleState::Conjunction { memory } => {
                for source in sources {
                    memory.insert(source.clone(), false);
                }
            }
            _ => {}
        }
    }

    fn apply_pulse(&mut self, source: Option<String>, is_high: bool, bus: &mut VecDeque<Pulse>) {
        let output_pulse = match &mut self.state {
            ModuleState::FlipFlop { is_on } => {
                if is_high {
                    // ignore
                    None
                } else {
                    // Flip state
                    *is_on = !*is_on;
                    Some(*is_on)
                }
            }
            ModuleState::Conjunction { memory } => {
                memory.insert(source.unwrap().clone(), is_high);
                // If all inputs are high, send low
                Some(!memory.values().all(|&value| value))
            }
            ModuleState::Broadcaster => Some(is_high),
            ModuleState::Sink => None,
        };

        // Send pulse matching the new state
        if let Some(output_pulse) = output_pulse {
            for target in &self.targets {
                bus.push_back(Pulse {
                    source: Some(self.name.to_string()),
                    target: target.clone(),
                    is_high: output_pulse,
                });
            }
        }
    }
}

pub fn part1() {
    let lines = lib::read_lines();

    let mut in_edges = HashMap::<String, Vec<String>>::new();
    let mut out_edges = HashMap::<String, Vec<String>>::new();
    let mut modules = HashMap::<String, Module>::new();
    let mut broadcaster: String = "".to_string();

    for line in &lines {
        let parts = line.split(" -> ").collect::<Vec<_>>();
        let module_type = parts[0].chars().next().unwrap();

        let name = match module_type {
            '%' | '&' => &parts[0][1..],
            _ => {
                broadcaster = parts[0].to_string();
                &parts[0]
            }
        }
        .to_string();

        in_edges.entry(name.clone()).or_insert(vec![]);

        let state = match module_type {
            '%' => ModuleState::FlipFlop { is_on: false },
            '&' => ModuleState::Conjunction {
                memory: HashMap::new(),
            },
            _ => ModuleState::Broadcaster,
        };

        let targets: Vec<String> = parts[1].trim().split(", ").map(|s| s.to_string()).collect();
        for target in &targets {
            out_edges
                .entry(name.clone())
                .or_insert(vec![])
                .push(target.clone());
            in_edges
                .entry(target.clone())
                .or_insert(vec![])
                .push(name.clone());
        }

        let module = Module {
            name: name.to_string(),
            state,
            targets,
        };
        modules.insert(name, module);
    }

    for name in in_edges.keys() {
        modules.entry(name.clone()).or_insert(Module {
            name: name.clone(),
            state: ModuleState::Sink,
            targets: vec![],
        });
    }

    // Initialize the memories for conjunction modules
    for (name, module) in &mut modules {
        module.initiaze_in_edges(in_edges.get(name).unwrap());
        println!("{:?}", module);
    }

    // Begin emulation
    let mut num_low = 0;
    let mut num_high = 0;

    for _ in 0..1000 {
        let mut bus = VecDeque::<Pulse>::new();

        // Push button
        bus.push_back(Pulse {
            source: None,
            target: broadcaster.to_string(),
            is_high: false,
        });

        while let Some(pulse) = bus.pop_front() {
            println!("{}", pulse);
            let Pulse {
                source,
                target,
                is_high,
            } = pulse;

            if is_high {
                num_high += 1;
            } else {
                num_low += 1;
            }

            let module = modules.get_mut(&target).unwrap();
            module.apply_pulse(source, is_high, &mut bus);
        }
    }

    println!("{}x{} = {}", num_low, num_high, num_low * num_high);
}

pub fn part2() {}

pub fn main() {
    part1();
    // part2();
}
