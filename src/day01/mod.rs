use std::collections::HashMap;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 1");
    let data = input(1, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let (left, right) = sort(input);
    let dist = distances(left, right);
    let total: u64 = dist.into_iter().sum();
    println!("Sum: {total}");
}

fn p2(input: &str) {
    let (left, right) = sort(input);
    let ss: u64 = similarity_scores(left, right).iter().sum();
    println!("Similary Score: {ss}");
}

fn sort(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();
        let l = l.trim();
        let r = r.trim();
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    }

    left.sort();
    right.sort();
    (left, right)
}

fn distances(left: Vec<u64>, right: Vec<u64>) -> Vec<u64> {
    let mut d = Vec::new();
    for (l, r) in left.into_iter().zip(right.into_iter()) {
        let min = l.min(r);
        let max = l.max(r);
        d.push(max - min);
    }

    d
}

fn similarity_scores(left: Vec<u64>, right: Vec<u64>) -> Vec<u64> {
    let mut ss = Vec::new();
    let mut counter = HashMap::<u64, u64>::new();

    for r in right.iter() {
        *counter.entry(*r).or_default() += 1;
    }

    for l in left.iter() {
        ss.push(*l * counter.get(l).copied().unwrap_or_default());
    }

    ss
}
