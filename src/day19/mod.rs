use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
};

use itertools::Itertools;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 19");
    let data = input(19, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let (towels, designs) = parse(input);

    let matches = designs
        .into_iter()
        .filter(|d| check_design(d, &towels))
        .count();
    println!("P1: {matches}");
}

fn p2(input: &str) {}

fn check_design(design: &str, towels: &Vec<String>) -> bool {
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();
    let mut buffer = design.to_string();

    loop {
        let mut match_found = false;
        for towel in towels {
            if buffer.starts_with(towel) {
                match_found = true;
                left.push_back(towel.as_str());
                buffer.replace_range(..towel.len(), "");
                break;
            }
        }
        if !match_found {
            break;
        }
    }

    if buffer.is_empty() {
        return true;
    }

    loop {
        let mut match_found = false;
        for towel in towels {
            if buffer.ends_with(towel) {
                match_found = true;
                right.push_front(towel.as_str());
                let idx = buffer.len() - towel.len();
                buffer.replace_range(idx.., "");
                break;
            }
        }
        if !match_found {
            match left.pop_back() {
                Some(piece) => buffer.insert_str(0, piece),
                None => break,
            }
        }
    }

    if buffer.is_empty() {
        return true;
    }

    false
}

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();
    let mut towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(ToString::to_string)
        .collect_vec();
    towels.sort_by_key(|towel| Reverse(towel.len()));
    lines.next().unwrap();
    let designs = lines.map(ToString::to_string).collect_vec();
    (towels, designs)
}
