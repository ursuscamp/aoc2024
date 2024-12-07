use itertools::{repeat_n, Itertools};

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 7");
    let data = input(7, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let mut total: u64 = 0;
    let equations = parse(input);
    for equation in equations {
        for solution in equation.potential_solutions() {
            let result = solve_solution(&solution);
            if result == equation.result {
                total += result;
                break;
            }
        }
    }
    println!("P1: {total}");
}

fn p2(input: &str) {}

fn parse(input: &str) -> Vec<Equation> {
    input.lines().map(Equation::parse).collect()
}

#[derive(Debug, Clone, Default)]
struct Equation {
    result: u64,
    nums: Vec<u64>,
}

impl Equation {
    fn parse(input: &str) -> Equation {
        let (result, rest) = input.split_once(':').unwrap();
        let result: u64 = result.parse().unwrap();
        let nums: Vec<u64> = rest
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Equation { result, nums }
    }

    fn op_count(&self) -> usize {
        self.nums.len() - 1
    }

    fn potential_solutions(&self) -> Vec<Vec<EqPart>> {
        let mut v: Vec<Vec<EqPart>> = Vec::new();
        for ops in Ops::permutations(self.op_count()) {
            v.push(
                self.nums
                    .iter()
                    .copied()
                    .map(|n| EqPart::Num(n))
                    .interleave(ops.into_iter().map(|o| EqPart::Op(o)))
                    .collect(),
            );
        }
        v
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Ops {
    Add,
    Mult,
}

impl Ops {
    fn all() -> impl Iterator<Item = Ops> + Clone {
        [Ops::Add, Ops::Mult].into_iter()
    }

    fn permutations(count: usize) -> impl Iterator<Item = Vec<Ops>> {
        repeat_n(Ops::all(), count).multi_cartesian_product()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EqPart {
    Num(u64),
    Op(Ops),
}

fn solve_solution(solution: &Vec<EqPart>) -> u64 {
    let mut val = 0;
    let mut op: Option<Ops> = None;

    for &next in solution {
        match (op, next) {
            (None, EqPart::Num(n)) => val = n,
            (None, EqPart::Op(ops)) => op = Some(ops),
            (Some(Ops::Add), EqPart::Num(n)) => {
                val += n;
                op = None;
            }
            (Some(Ops::Mult), EqPart::Num(n)) => {
                val *= n;
                op = None;
            }
            (Some(_), EqPart::Op(ops)) => unimplemented!(),
        }
    }

    val
}
