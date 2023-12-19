use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, ops::Deref};

use aoc2023::lib;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum Operand {
    x,
    m,
    a,
    s,
}

impl From<char> for Operand {
    fn from(b: char) -> Operand {
        match b {
            'x' => Operand::x,
            'm' => Operand::m,
            'a' => Operand::a,
            's' => Operand::s,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Operator {
    Lesser,
    Greater,
}

impl From<char> for Operator {
    fn from(b: char) -> Operator {
        match b {
            '<' => Operator::Lesser,
            '>' => Operator::Greater,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
enum Result {
    Accept,
    Reject,
    Workflow(String),
}

impl From<&str> for Result {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            name => Self::Workflow(name.to_string()),
        }
    }
}

#[derive(Debug)]
struct Condition {
    operand: Operand,
    operator: Operator,
    value: i64,
}

impl Condition {
    fn matches(&self, part: &Part) -> bool {
        let part_value = match self.operand {
            Operand::x => part.x,
            Operand::m => part.m,
            Operand::a => part.a,
            Operand::s => part.s,
        };
        match self.operator {
            Operator::Lesser => part_value < self.value,
            Operator::Greater => part_value > self.value,
        }
    }
}

impl From<&str> for Condition {
    fn from(s: &str) -> Condition {
        // e.g. "a<2006" or "b>537"
        let mut chars = s.chars();
        let operand: Operand = chars.next().unwrap().into();
        let operator: Operator = chars.next().unwrap().into();
        let value = chars.collect::<String>().parse().unwrap();
        Condition {
            operand,
            operator,
            value,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    conditions: Vec<(Condition, Result)>,
    result: Result, // if none of the conditions match
}

impl Workflow {
    fn apply(&self, part: Part) -> &Result {
        let conditions = &self.conditions;
        for (condition, result) in conditions {
            if condition.matches(&part) {
                return result;
            }
        }
        &self.result
    }
}

impl From<&str> for Workflow {
    fn from(s: &str) -> Workflow {
        let (name, s) = s.split_once('{').unwrap();

        let mut parts = s.split(',').collect::<Vec<_>>();
        let last = parts.pop().unwrap();
        let conditions = parts
            .iter()
            .map(|part| {
                let (condition, result) = part.split_once(':').unwrap();
                (Condition::from(condition), Result::from(result))
            })
            .collect::<Vec<_>>();
        let result: Result = last[0..last.len() - 1].into();

        Workflow {
            name: name.to_string(),
            conditions,
            result,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn value(&self) -> i64 {
        return self.x + self.m + self.a + self.s;
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        // e.g. "{x=2461,m=1339,a=466,s=291}"
        let mut parts = value[1..value.len() - 1].split(',');
        // Assume that parts are always in order x, m, a, s
        Part {
            x: parts.next().unwrap()[2..].parse().unwrap(),
            m: parts.next().unwrap()[2..].parse().unwrap(),
            a: parts.next().unwrap()[2..].parse().unwrap(),
            s: parts.next().unwrap()[2..].parse().unwrap(),
        }
    }
}

fn is_accepted(part: Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut workflow = workflows.get("in").unwrap();
    loop {
        match workflow.apply(part) {
            Result::Accept => return true,
            Result::Reject => return false,
            Result::Workflow(next_workflow) => workflow = workflows.get(next_workflow).unwrap(),
        }
    }
}

pub fn part1() {
    let lines = lib::read_lines();
    let inputs = lines.split(|l| l.len() == 0).collect::<Vec<_>>();

    let mut workflows = HashMap::new();
    for workflow in inputs[0] {
        let w: Workflow = workflow.as_str().into();
        // println!("{:?}", w);
        assert!(workflows.insert(w.name.clone(), w).is_none());
    }

    let mut total = 0;
    for part_str in inputs[1] {
        let part: Part = part_str.as_str().into();
        println!("{:?} {}", part, part.value());
        if is_accepted(part, &workflows) {
            println!("  Accepted");
            total += part.value();
        }
    }

    println!("{}", total);
}

pub fn part2() {
    let lines = lib::read_lines();
    let inputs = lines.split(|l| l.len() == 0).collect::<Vec<_>>();

    let mut workflows = HashMap::new();
    for workflow in inputs[0] {
        let w: Workflow = workflow.as_str().into();
        // println!("{:?}", w);
        assert!(workflows.insert(w.name.clone(), w).is_none());
    }

    let mut ranges: HashMap<Operand, Vec<i64>> = [
        (Operand::x, vec![]),
        (Operand::m, vec![]),
        (Operand::a, vec![]),
        (Operand::s, vec![]),
    ]
    .into();

    for workflow in workflows.values() {
        for (condition, _) in &workflow.conditions {
            ranges
                .get_mut(&condition.operand)
                .unwrap()
                .push(match condition.operator {
                    Operator::Lesser => condition.value - 1,
                    Operator::Greater => condition.value,
                });
        }
    }

    for (_, range) in ranges.iter_mut() {
        range.push(0);
        range.push(4000);
        range.sort();
        range.dedup();
    }

    for (op, range) in &ranges {
        println!("{:?} {}", op, range.len());
    }

    let xr = ranges.get(&Operand::x).unwrap();
    let mr = ranges.get(&Operand::m).unwrap();
    let ar = ranges.get(&Operand::a).unwrap();
    let sr = ranges.get(&Operand::s).unwrap();

    let mut total = 0;
    let mut part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };

    for xi in 1..xr.len() {
        let xw = xr[xi] - xr[xi - 1];
        part.x = xr[xi];
        for mi in 1..mr.len() {
            let mw = mr[mi] - mr[mi - 1];
            part.m = mr[mi];
            for ai in 1..ar.len() {
                let aw = ar[ai] - ar[ai - 1];
                part.a = ar[ai];
                for si in 1..sr.len() {
                    let sw = sr[si] - sr[si - 1];
                    part.s = sr[si];
                    if is_accepted(part, &workflows) {
                        // println!("{:?} {} {} {} {}", part, xw, mw, aw, sw);
                        total += xw * mw * aw * sw;
                    }
                }
            }
        }
        println!("{}", total);
    }

    println!("{}", total);
}

pub fn main() {
    // part1();
    part2();
}
