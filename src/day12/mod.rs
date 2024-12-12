use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::Itertools;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 12");
    let data = input(12, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let board = parse(input);
    let cost = calculate_cost(&board);
    println!("P1: {cost:?}");
}

fn p2(input: &str) {
    let board = parse(input);
}

fn parse(input: &str) -> HashMap<Coord, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, ch)| {
                (
                    Coord {
                        x: x as isize,
                        y: y as isize,
                    },
                    ch,
                )
            })
        })
        .collect()
}

fn calculate_cost(board: &HashMap<Coord, char>) -> usize {
    let mut queue = board.iter().map(|(coord, _)| *coord).collect_vec();
    let mut regions = Vec::new();
    while let Some(coord) = queue.pop() {
        if !board.contains_key(&coord) {
            continue;
        }
        let ch = board[&coord];
        let mut region = HashSet::new();
        let mut reg_queue = vec![coord];
        while let Some(next_coord) = reg_queue.pop() {
            if board.get(&next_coord) == Some(&ch) {
                region.insert(next_coord);
                reg_queue.extend(
                    next_coord
                        .borders()
                        .into_iter()
                        .filter(|c| !region.contains(c)),
                );
            }
        }
        queue.retain(|c| !region.contains(c));
        regions.push(region);
    }
    regions.iter().map(region_cost).sum()
}

fn region_cost(region: &HashSet<Coord>) -> usize {
    region
        .iter()
        .map(|c| {
            c.borders()
                .into_iter()
                .filter(|c| region.contains(c))
                .count()
        })
        .map(|c| 4 - c)
        .sum::<usize>()
        * region.len()
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn borders(&self) -> [Coord; 4] {
        [self.up(), self.down(), self.left(), self.right()]
    }

    fn up(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_cost() {
        let region = (0..4).map(|x| Coord { x, y: 0 }).collect();
        assert_eq!(region_cost(&region), 40);
    }
}
