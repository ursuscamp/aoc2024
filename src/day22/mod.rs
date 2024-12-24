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
        .sum::<u64>();
    println!("P1: {result}");
}

fn p2(input: &str) {}

fn parse(input: &str) -> Vec<SecretNum> {
    input
        .lines()
        .map(|line| SecretNum(line.parse().unwrap()))
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct SecretNum(u64);

impl SecretNum {
    fn mix(&self, other: u64) -> SecretNum {
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
