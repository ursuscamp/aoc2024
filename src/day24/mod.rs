use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 24");
    let data = input(24, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let mut wires = parse(input);
    execute(&mut wires);
    let result = calc_decimal('z', &wires);
    println!("P1: {:#?}", result);
}

fn p2(input: &str) {
    let mut wires = parse(input);
    execute(&mut wires);
    faulty_wires(&wires);
    // println!("P2: {:#?}", result);
}

fn faulty_wires(wires: &HashMap<String, Value>) {
    println!("Checking for faulty wires");
    let x = calc_decimal('x', wires);
    let y = calc_decimal('y', wires);
    let z = calc_decimal('z', wires);
    let expected = x + y;
    println!("Expected: {:b}", expected);
    println!("Actual  : {:b}", z);

    for i in 0i64..64 {
        let mask = 1 << i;
        let z = z & mask;
        let expected = expected & mask;
        if expected != z {
            print!("z{:02},", i);
        }
    }
    println!();
}

fn check_result(wires: &HashMap<String, Value>) -> bool {
    let x = calc_decimal('x', wires);
    let y = calc_decimal('y', wires);
    let z = calc_decimal('z', wires);
    x + y == z
}

fn calc_decimal(prefix: char, wires: &HashMap<String, Value>) -> i64 {
    wires
        .keys()
        .filter(|k| k.starts_with(prefix))
        .sorted()
        .rev()
        .map(|k| wires[k].unwarp_num())
        .fold(0, |acc, n| (acc << 1) | n)
}

fn execute(wires: &mut HashMap<String, Value>) {
    let mut queue: VecDeque<String> = wires.keys().cloned().collect();
    while let Some(wire) = queue.pop_front() {
        let value = wires[&wire].clone();
        match value {
            Value::Op(op) => {
                op.exec(&wire, wires);
            }
            Value::Num(_) => {}
        }
    }
}

fn parse(input: &str) -> HashMap<String, Value> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let s = if line.contains(":") {
                let (wire, value) = line.split_once(": ").unwrap();
                (wire.to_string(), Value::Num(value.parse().unwrap()))
            } else {
                let (op, wire) = line.split_once(" -> ").unwrap();
                let op = Op::parse(op);
                (wire.to_string(), Value::Op(op))
            };
            Some(s)
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Op(Op),
    Num(i64),
}

impl Value {
    fn unwarp_num(&self) -> i64 {
        match self {
            Value::Num(i) => *i,
            _ => panic!("Expected Num"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl Op {
    fn exec(&self, label: &str, wires: &mut HashMap<String, Value>) -> i64 {
        let (l, r) = match self {
            Op::And(l, r) => (l, r),
            Op::Or(l, r) => (l, r),
            Op::Xor(l, r) => (l, r),
        };
        let l = match wires.get(l).unwrap().clone() {
            Value::Op(op) => op.exec(l, wires),
            Value::Num(i) => i,
        };
        let r = match wires.get(r).unwrap().clone() {
            Value::Op(op) => op.exec(r, wires),
            Value::Num(i) => i,
        };

        let result = match self {
            Op::And(_, _) => l & r,
            Op::Or(_, _) => l | r,
            Op::Xor(_, _) => l ^ r,
        };
        wires.insert(label.to_string(), Value::Num(result));
        result
    }

    // fn extract_nums(&self, wires: &mut HashMap<String, Value>) -> Option<i64, i64> {
    //     let (l,r) = match self {
    //         Op::And(l,r) => (l,r),
    //         Op::Or(l,r,) => (l,r),
    //         Op::Xor(l,r,) => (l,r),
    //     };
    //     wires.get(l).and
    // }

    fn parse(input: &str) -> Op {
        let (left, op, right) = input.split(" ").collect_tuple().unwrap();
        match op {
            "AND" => Op::And(left.to_string(), right.to_string()),
            "OR" => Op::Or(left.to_string(), right.to_string()),
            "XOR" => Op::Xor(left.to_string(), right.to_string()),
            _ => panic!("Unknown op: {}", op),
        }
    }
}
