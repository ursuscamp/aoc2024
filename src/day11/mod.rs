use std::collections::HashMap;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 11");
    let data = input(11, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let nums = parse(input);
    let mut cache = HashMap::new();
    let result = nums
        .into_iter()
        .map(|n| process_stone(n, 25, &mut cache))
        .sum::<usize>();
    println!("P1: {result:?}");
}

fn p2(input: &str) {
    let nums = parse(input);
    let mut cache = HashMap::new();
    let result = nums
        .into_iter()
        .map(|n| process_stone(n, 75, &mut cache))
        .sum::<usize>();
    println!("P2: {result:?}");
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn process_stone(stone: u64, blinks: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    // Recursive base case
    if blinks == 0 {
        return 1;
    }

    if let Some(val) = cache.get(&(stone, blinks)) {
        return *val;
    }

    let val = if stone == 0 {
        process_stone(1, blinks - 1, cache)
    } else if count_digits(stone) % 2 == 0 {
        let (left, right) = split_number(stone);
        process_stone(left, blinks - 1, cache) + process_stone(right, blinks - 1, cache)
    } else {
        process_stone(stone * 2024, blinks - 1, cache)
    };
    cache.insert((stone, blinks), val);
    val
}

fn count_digits(num: u64) -> u32 {
    if num == 0 {
        return 1;
    }

    num.ilog10() + 1
}

fn split_number(num: u64) -> (u64, u64) {
    let digit_count = count_digits(num);
    let half_digit_count = digit_count / 2;

    let left_half = num / 10u64.pow(half_digit_count);
    let right_half = num % 10u64.pow(half_digit_count);

    (left_half, right_half)
}
