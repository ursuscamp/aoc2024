use std::collections::{HashMap, HashSet};

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 5");
    let data = input(5, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let (sorter, updates) = parse(input);
    let mut sum = 0;
    for (idx, update) in updates.iter().enumerate() {
        let sorted = sorter.sort(update);
        if update.eq(&sorted) {
            let midpoint = update.len() / 2;
            sum += update[midpoint];
        }
    }
    println!("P1: {sum}");
}

fn p2(input: &str) {
    let (sorter, updates) = parse(input);
    let mut sum = 0;
    for (idx, update) in updates.iter().enumerate() {
        let sorted = sorter.sort(update);
        if update.ne(&sorted) {
            let midpoint = sorted.len() / 2;
            sum += sorted[midpoint];
        }
    }
    println!("P2: {sum}");
}

fn parse(input: &str) -> (Sorter, Vec<Vec<i64>>) {
    let mut pairs: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut updates: Vec<Vec<i64>> = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            let (left, right) = line.split_once('|').unwrap();
            let left = left.parse().unwrap();
            let right = right.parse().unwrap();
            pairs.entry(left).or_default().insert(right);
        }

        if line.contains(',') {
            updates.push(line.split(',').map(|l| l.parse().unwrap()).collect());
        }
    }

    (Sorter { pairs }, updates)
}

#[derive(Default, Debug)]
struct Sorter {
    pairs: HashMap<i64, HashSet<i64>>,
}

impl Sorter {
    fn sort(&self, input: &[i64]) -> Vec<i64> {
        let mut sorted = input.to_vec();
        sorted.sort_by(|a, b| {
            if self
                .pairs
                .get(a)
                .map(|set| set.contains(b))
                .unwrap_or_default()
            {
                std::cmp::Ordering::Less
            } else if self
                .pairs
                .get(b)
                .map(|set| set.contains(b))
                .unwrap_or_default()
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        sorted
    }
}
