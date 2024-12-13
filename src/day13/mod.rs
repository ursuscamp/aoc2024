use std::{cell::OnceCell, sync::OnceLock};

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
    let machines = parse(input);
    let cost: usize = machines.iter().flat_map(Machine::solve).sum();
    println!("P1: {cost:?}");
}

fn p2(input: &str) {}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .tuples()
        .map(|(a, b, p, _)| Machine::parse(a, b, p))
        .collect()
}

static BUTTON_A: OnceLock<Regex> = OnceLock::new();
static BUTTON_B: OnceLock<Regex> = OnceLock::new();
static PRIZE: OnceLock<Regex> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

impl Machine {
    fn parse(button_a: &str, button_b: &str, prize: &str) -> Machine {
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
        let prize = (caps[1].parse().unwrap(), caps[2].parse().unwrap());

        Machine { a, b, prize }
    }

    fn solve(&self) -> Option<usize> {
        (0usize..100)
            .flat_map(|a| (0usize..100).map(move |b| (a, b)))
            .flat_map(|(a, b)| {
                let cost = a * 3 + b;
                let x = self.a.0 * a + self.b.0 * b;
                let y = self.a.1 * a + self.b.1 * b;
                if (x, y) == self.prize {
                    // println!("Prize found: {x},{y} => {cost}");
                    Some(cost)
                } else {
                    None
                }
            })
            .min()
    }
}
