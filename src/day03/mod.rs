use regex::Regex;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 3");
    let data = input(3, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let ins = Ins::parse(input);
    let total: u64 = ins.iter().map(Ins::eval).sum();
    println!("Part 1: {total}");
}

fn p2(input: &str) {}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Ins {
    Mul(u64, u64),
}

impl Ins {
    fn parse(text: &str) -> Vec<Self> {
        let r = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();
        let mut ins = Vec::new();

        let mut text = text;
        while !text.is_empty() {
            match r.captures(text) {
                Some(caps) => {
                    let n1: u64 = caps[1].parse().unwrap();
                    let n2: u64 = caps[2].parse().unwrap();
                    ins.push(Ins::Mul(n1, n2));
                    text = &text[caps[0].len()..];
                }
                None => text = &text[1..],
            }
        }

        ins
    }

    fn eval(&self) -> u64 {
        match self {
            Ins::Mul(l, r) => *l * *r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ins_parse() {
        assert_eq!(Ins::parse("mul(3,4)"), &[Ins::Mul(3, 4)]);
        assert_eq!(Ins::parse("%mul(3,4)"), &[Ins::Mul(3, 4)]);
        assert_eq!(
            Ins::parse("%mul(3,4)&Mul(4,5)mul(7,6)"),
            &[Ins::Mul(3, 4), Ins::Mul(7, 6)]
        );
    }
}
