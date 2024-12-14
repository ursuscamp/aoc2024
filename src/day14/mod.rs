use itertools::Itertools;
use regex::Regex;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 14");
    let data = input(14, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let values = parse(input);
    let width = 101;
    let height = 103;
    let values = process_values(&values, 100, width, height)
        .into_iter()
        .flat_map(|pair| as_quadrant(pair, width, height))
        .counts()
        .values()
        .fold(1, |acc, value| *value * acc);
    println!("P1: {values:?}");
}

fn p2(input: &str) {}

type Pair = (isize, isize);
type Value = (Pair, Pair);

fn parse(input: &str) -> Vec<Value> {
    let r = Regex::new(r#"p=(\d+),(\d+) v=(-?\d+),(-?\d+)"#).unwrap();
    input
        .lines()
        .map(|line| {
            let caps = r.captures(line).unwrap();
            (
                (caps[1].parse::<isize>().unwrap(), caps[2].parse().unwrap()),
                (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            )
        })
        .collect()
}

fn as_quadrant((x, y): Pair, width: isize, height: isize) -> Option<isize> {
    let mid_x = width / 2;
    let mid_y = height / 2;
    match (x.cmp(&mid_x), y.cmp(&mid_y)) {
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(1),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(3),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(2),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(4),
        _ => None,
    }
}

fn process_values(values: &[Value], seconds: isize, width: isize, height: isize) -> Vec<Pair> {
    calculate_raw_positions(values, seconds)
        .into_iter()
        .map(|pair| teleport(&pair, width, height))
        .collect()
}

fn calculate_raw_positions(values: &[Value], seconds: isize) -> Vec<Pair> {
    values
        .iter()
        .copied()
        .map(|((x, y), (vx, vy))| (x + vx * seconds, y + vy * seconds))
        .collect()
}

fn teleport(pair: &Pair, width: isize, height: isize) -> Pair {
    let (mut x, mut y) = *pair;

    while x < 0 {
        x += width;
    }

    while x >= width {
        x -= width;
    }

    while y < 0 {
        y += height;
    }

    while y >= height {
        y -= height;
    }

    (x, y)
}
