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

fn p2(input: &str) {
    let total = Machine::run(input);
    println!("Part 2: {total}");
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Ins {
    Mul(u64, u64),
    Dont,
    Do,
}

impl Ins {
    fn parse(text: &str) -> Vec<Self> {
        let r = Regex::new(r"^mul\((\d+),(\d+)\)|^do\(\)|^don't\(\)").unwrap();
        let mut ins = Vec::new();

        let mut text = text;
        while !text.is_empty() {
            match r.captures(text) {
                Some(caps) => {
                    let full = &caps[0];
                    if full.starts_with("mul") {
                        let n1: u64 = caps[1].parse().unwrap();
                        let n2: u64 = caps[2].parse().unwrap();
                        ins.push(Ins::Mul(n1, n2));
                    } else if full.starts_with("don't") {
                        ins.push(Ins::Dont);
                    } else if full.starts_with("do") {
                        ins.push(Ins::Do);
                    }
                    text = &text[full.len()..];
                }
                None => text = &text[1..],
            }
        }

        ins
    }

    fn eval(&self) -> u64 {
        match self {
            Ins::Mul(l, r) => *l * *r,
            _ => 0,
        }
    }
}

struct Machine(Vec<Ins>);

impl Machine {
    fn run(input: &str) -> u64 {
        let ins = Ins::parse(input);
        let machine = Machine(ins);
        machine._run()
    }

    fn _run(self) -> u64 {
        let mut sum = 0u64;
        let mut exe = true;
        for ins in self.0 {
            match ins {
                Ins::Mul(l, r) if exe => sum += l * r,
                Ins::Dont => exe = false,
                Ins::Do => exe = true,
                _ => {}
            }
        }
        sum
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
        assert_eq!(
            Ins::parse("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            &[
                Ins::Mul(2, 4),
                Ins::Dont,
                Ins::Mul(5, 5),
                Ins::Mul(11, 8),
                Ins::Do,
                Ins::Mul(8, 5)
            ]
        );
    }
}
