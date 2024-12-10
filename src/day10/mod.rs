use itertools::Itertools;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 10");
    let data = input(10, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let board = Board(parse(input));
    let result: usize = board
        .trailheads()
        .into_iter()
        .map(|th| board.find_trails(&th))
        .map(|th| th.iter().unique_by(|path| path.last().copied()).count())
        .sum();
    println!("P1: {result:#?}");
}

fn p2(input: &str) {}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn next(&self) -> Vec<Coord> {
        [
            self.up(),
            Some(self.right()),
            Some(self.down()),
            self.left(),
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }

    fn up(&self) -> Option<Coord> {
        Some(Coord {
            x: self.x,
            y: self.y.checked_sub(1)?,
        })
    }

    fn down(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Option<Coord> {
        Some(Coord {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }

    fn right(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board(Vec<Vec<u32>>);

impl Board {
    fn get(&self, coord: &Coord) -> Option<u32> {
        self.0.get(coord.y).and_then(|l| l.get(coord.x).copied())
    }

    fn trailheads(&self) -> Vec<Coord> {
        let mut coords = Vec::new();
        for (y, line) in self.0.iter().enumerate() {
            for (x, n) in line.iter().enumerate() {
                if *n == 0 {
                    coords.push(Coord { x, y });
                }
            }
        }
        coords
    }

    fn find_trails(&self, trailhead: &Coord) -> Vec<Vec<Coord>> {
        let next = trailhead.next();
        next.into_iter()
            .flat_map(|n| self.find_next_step(&n, 1, vec![*trailhead]))
            .collect_vec()
    }

    fn find_next_step(
        &self,
        coord: &Coord,
        expected: u32,
        mut path: Vec<Coord>,
    ) -> Vec<Vec<Coord>> {
        let value = match self.get(coord) {
            Some(n) => n,
            None => return Vec::new(),
        };

        if value != expected {
            return Vec::new();
        }

        path.push(*coord);

        if value == 9 {
            return vec![path];
        }

        coord
            .next()
            .into_iter()
            .flat_map(|next| self.find_next_step(&next, value + 1, path.clone()))
            .collect_vec()
    }
}
