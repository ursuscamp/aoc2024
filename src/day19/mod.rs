use std::{cmp::Reverse, collections::HashMap};

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

    let mut cache = HashMap::new();
    let matches = designs
        .into_iter()
        .filter(|d| count_combinations(d, &towels, &mut cache) > 0)
        .count();
    println!("P1: {matches}");
}

fn p2(input: &str) {
    let (towels, designs) = parse(input);

    let mut cache = HashMap::new();
    let sum: usize = designs
        .into_iter()
        .map(|design| count_combinations(&design, &towels, &mut cache))
        .sum();
    println!("P2: {sum}");
}

fn count_combinations(
    design: &str,
    towels: &Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(answer) = cache.get(design) {
        return *answer;
    }

    let mut count = 0;
    for towel in towels {
        if design.starts_with(towel) {
            count += count_combinations(&design[towel.len()..], towels, cache);
        }
    }

    cache.insert(design.to_string(), count);

    count
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
