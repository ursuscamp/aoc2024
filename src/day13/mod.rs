use std::sync::OnceLock;

use itertools::Itertools;
use regex::Regex;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 13");
    let data = input(13, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let machines = parse(input, 0);
    let cost: isize = machines.iter().flat_map(Machine::solve).sum();
    println!("P1: {cost:?}");
}

fn p2(input: &str) {
    let machines = parse(input, 10000000000000);
    let cost: isize = machines.iter().flat_map(Machine::solve).sum();
    println!("P2: {cost:?}");
}

fn parse(input: &str, correction: isize) -> Vec<Machine> {
    input
        .lines()
        .tuples()
        .map(|(a, b, p, _)| Machine::parse(a, b, p, correction))
        .collect()
}

static BUTTON_A: OnceLock<Regex> = OnceLock::new();
static BUTTON_B: OnceLock<Regex> = OnceLock::new();
static PRIZE: OnceLock<Regex> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    fn parse(button_a: &str, button_b: &str, prize: &str, correction: isize) -> Machine {
        let regex_a =
            BUTTON_A.get_or_init(|| Regex::new(r#"Button A: X\+(\d+), Y\+(\d+)$"#).unwrap());
        let regex_b =
            BUTTON_B.get_or_init(|| Regex::new(r#"Button B: X\+(\d+), Y\+(\d+)$"#).unwrap());
        let regex_p = PRIZE.get_or_init(|| Regex::new(r#"Prize: X=(\d+), Y=(\d+)$"#).unwrap());

        let caps = regex_a.captures(button_a).expect("button A");
        let a = (caps[1].parse().unwrap(), caps[2].parse().unwrap());

        let caps = regex_b.captures(button_b).expect("button B");
        let b = (caps[1].parse().unwrap(), caps[2].parse().unwrap());

        let caps = regex_p.captures(prize).expect("prize");
        let prize = (
            caps[1].parse::<isize>().unwrap() + correction,
            caps[2].parse::<isize>().unwrap() + correction,
        );

        Machine { a, b, prize }
    }

    fn solve(&self) -> Option<isize> {
        let b = (self.a.0 * self.prize.1 - self.a.1 * self.prize.0)
            / (self.a.0 * self.b.1 - self.a.1 * self.b.0);
        let a = (self.prize.0 - self.b.0 * b) / self.a.0;
        let f = (self.a.0 * a + self.b.0 * b, self.a.1 * a + self.b.1 * b);
        if f != self.prize {
            return None;
        }
        Some(3 * a + b)
    }
}
