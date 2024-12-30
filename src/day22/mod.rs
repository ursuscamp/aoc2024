use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 22");
    let data = input(22, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let input = parse(input);
    let result = input
        .into_iter()
        .map(|secret_num| secret_num.nth(2000))
        .map(|s| s.0)
        .sum::<i64>();
    println!("P1: {result}");
}

fn p2(input: &str) {
    let input = parse(input);
    let mut cache = HashMap::new();
    for num in input {
        cache_best_price_changes_for_secret_number(num, 2000, &mut cache);
    }
    // let sn = find_best_price_changes(&cache);
    let sn = cache.values().max().unwrap();
    println!("{sn:#?}")
}

fn parse(input: &str) -> Vec<SecretNum> {
    input
        .lines()
        .map(|line| SecretNum(line.parse().unwrap()))
        .collect()
}

fn cache_best_price_changes_for_secret_number(
    sn: SecretNum,
    n: usize,
    cache: &mut HashMap<(i64, i64, i64, i64), i64>,
) {
    let sn = secret_num_prices(sn, n);
    let sn = secret_num_price_changes(&sn);
    cache_quadruplet_changes(&sn, cache);
}

fn cache_quadruplet_changes(
    price_changes: &[(i64, i64)],
    cache: &mut HashMap<(i64, i64, i64, i64), i64>,
) {
    let mut visited = HashSet::new();
    price_changes
        .iter()
        .copied()
        .tuple_windows()
        .for_each(|(f, s, t, fo)| {
            let key = (f.1, s.1, t.1, fo.1);
            // We only want to record the first time we see a quadruplet for every buyer
            if !visited.insert(key) {
                return;
            }
            *cache.entry(key).or_default() += fo.0;
        });
}

fn secret_num_price_changes(prices: &[i64]) -> Vec<(i64, i64)> {
    prices
        .iter()
        .copied()
        .tuple_windows()
        .map(|(f, s)| (s, s - f))
        .collect_vec()
}

fn secret_num_prices(sn: SecretNum, n: usize) -> Vec<i64> {
    let mut sns = Vec::from([sn.price()]);
    sns.extend_from_slice(
        &sn.evolutions(n)
            .into_iter()
            .map(|s| s.price())
            .collect_vec(),
    );
    sns
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct SecretNum(i64);

impl SecretNum {
    fn mix(&self, other: i64) -> SecretNum {
        SecretNum(self.0 ^ other)
    }

    fn prune(&self) -> SecretNum {
        SecretNum(self.0 % 16777216)
    }

    fn evolve(&self) -> SecretNum {
        let step1 = self.mix(self.0 * 64).prune();
        let step2 = step1.mix(step1.0 / 32).prune();

        step2.mix(step2.0 * 2048).prune()
    }

    fn nth(&self, n: usize) -> SecretNum {
        (0..n).fold(*self, |acc, _| acc.evolve())
    }

    fn evolutions(&self, n: usize) -> Vec<SecretNum> {
        (0..n)
            .scan(*self, |acc, _| {
                let next = acc.evolve();
                *acc = next;
                Some(next)
            })
            .collect()
    }

    fn price(&self) -> i64 {
        self.0
            .to_string()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(SecretNum(42).mix(15), SecretNum(37));
    }

    #[test]
    fn test_prune() {
        assert_eq!(SecretNum(100000000).prune(), SecretNum(16113920));
    }

    #[test]
    fn test_evolve() {
        assert_eq!(SecretNum(123).evolve(), SecretNum(15887950));
        assert_eq!(SecretNum(15887950).evolve(), SecretNum(16495136));
    }
}
