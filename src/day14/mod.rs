use itertools::Itertools;
use regex::Regex;

use crate::utils::input;

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 14");
    let data = input(14, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let values = parse(input);
    let values = process_values(values.into_iter(), 100)
        .flat_map(|pair| as_quadrant(pair))
        .counts()
        .values()
        .fold(1, |acc, value| *value * acc);
    println!("P1: {values:?}");
}

fn p2(input: &str) {
    let mut values = parse(input);
    for i in 0..10_000 {
        values = process_values(values.iter().copied(), 1)
            .zip(values.iter())
            .map(|(np, (op, ov))| (np, *ov))
            .collect_vec();
        if possible_tree(&values) {
            println!("==== {} possible tree", i + 1);
            print_board(&values);
        }
    }
    print_board(&values);
}

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

fn as_quadrant((x, y): Pair) -> Option<isize> {
    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;
    match (x.cmp(&mid_x), y.cmp(&mid_y)) {
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(1),
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(3),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(2),
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(4),
        _ => None,
    }
}

fn process_values(
    values: impl Iterator<Item = Value>,
    seconds: isize,
) -> impl Iterator<Item = Pair> {
    calculate_raw_positions(values, seconds).map(move |pair| teleport(&pair))
}

fn calculate_raw_positions(
    values: impl Iterator<Item = Value>,
    seconds: isize,
) -> impl Iterator<Item = Pair> {
    values.map(move |((x, y), (vx, vy))| (x + vx * seconds, y + vy * seconds))
}

fn teleport(pair: &Pair) -> Pair {
    let (mut x, mut y) = *pair;

    while x < 0 {
        x += WIDTH;
    }

    while x >= WIDTH {
        x -= WIDTH;
    }

    while y < 0 {
        y += HEIGHT;
    }

    while y >= HEIGHT {
        y -= HEIGHT;
    }

    (x, y)
}

type Board = [[char; WIDTH as usize]; HEIGHT as usize];

fn as_board(values: &[Value]) -> Board {
    let mut board = [['.'; WIDTH as usize]; HEIGHT as usize];
    for ((px, py), _) in values {
        board[*py as usize][*px as usize] = '*';
    }
    board
}

fn print_board(values: &[Value]) {
    let board = as_board(values);

    for line in board {
        for ch in line {
            print!("{ch}");
        }
        println!()
    }
}

fn possible_tree(values: &[Value]) -> bool {
    let board = as_board(values);
    for line in board {
        if line
            .into_iter()
            .chunk_by(|d| *d)
            .into_iter()
            .any(|(ch, chunk)| ch == '*' && chunk.count() >= 10)
        {
            return true;
        }
    }
    false
}
