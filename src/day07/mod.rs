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
    let equations = parse(input);
    let total = solve_all_solution(&equations, false);
    println!("P1: {total}");
}

fn p2(input: &str) {
    let equations = parse(input);
    let total = solve_all_solution(&equations, true);
    println!("P2: {total}");
}

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

    fn potential_solutions(&self, with_concat: bool) -> Vec<Vec<EqPart>> {
        let mut v: Vec<Vec<EqPart>> = Vec::new();
        for ops in Ops::permutations(self.op_count(), with_concat) {
            v.push(
                self.nums
                    .iter()
                    .copied()
                    .map(EqPart::Num)
                    .interleave(ops.into_iter().map(EqPart::Op))
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
    Concat,
}

impl Ops {
    fn all_without_concat() -> impl Iterator<Item = Ops> + Clone {
        [Ops::Add, Ops::Mult].into_iter()
    }

    fn all() -> impl Iterator<Item = Ops> + Clone {
        [Ops::Add, Ops::Mult, Ops::Concat].into_iter()
    }

    fn permutations(count: usize, with_concat: bool) -> Vec<Vec<Ops>> {
        if with_concat {
            repeat_n(Ops::all(), count)
                .multi_cartesian_product()
                .collect()
        } else {
            repeat_n(Ops::all_without_concat(), count)
                .multi_cartesian_product()
                .collect()
        }
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
            (Some(Ops::Concat), EqPart::Num(n)) => {
                let mut next = val.to_string();
                next += n.to_string().as_str();
                val = next.parse().unwrap();
                op = None;
            }
            (Some(_), EqPart::Op(_ops)) => unimplemented!(),
        }
    }

    val
}

fn solve_all_solution(equations: &Vec<Equation>, with_concat: bool) -> u64 {
    let mut total: u64 = 0;
    for equation in equations {
        for solution in equation.potential_solutions(with_concat) {
            let result = solve_solution(&solution);
            if result == equation.result {
                total += result;
                break;
            }
        }
    }
    total
}
