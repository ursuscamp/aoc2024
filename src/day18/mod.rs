use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::{input, Dir, Vec2};

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 18");
    let data = input(18, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let memory = Memory::parse(input);
    let ticks = if memory.size == 70 { 1024 } else { 12 };
    let min_cost = memory.find_path(ticks);
    println!("P1: {}", min_cost.unwrap());
}

fn p2(input: &str) {
    let memory = Memory::parse(input);
    let tick_idx = memory
        .bytes
        .iter()
        .copied()
        .enumerate()
        .rev()
        .find_map(|(tick_idx, _node)| {
            let ticks = tick_idx + 1;
            memory.find_path(ticks).and(Some(tick_idx + 1))
        })
        .unwrap();
    let node = memory.bytes[tick_idx];
    let tick = format!("{},{}", node.x, node.y);
    println!("P2: {tick}");
}

#[derive(Debug)]
struct Memory {
    size: isize,
    bytes: Vec<Vec2>,
}

impl Memory {
    fn find_path(&self, ticks: usize) -> Option<usize> {
        let bad_bytes: HashSet<Vec2> = self.tick(ticks).iter().copied().collect();
        let mut queue = Vec::from([(0usize, Vec2 { x: 0, y: 0 }, Dir::North)]);
        let end = Vec2 {
            x: self.size,
            y: self.size,
        };
        let mut scores = HashMap::new();

        while let Some((cost, coord, dir)) = queue.pop() {
            let score = scores.entry((coord, dir)).or_insert(usize::MAX);

            *score = cost.min(*score);

            if coord == end {
                continue;
            }

            for (next_node, next_dir) in [
                (coord.right(), Dir::East),
                (coord.down(), Dir::South),
                (coord.left(), Dir::West),
                (coord.up(), Dir::North),
            ] {
                if next_node.x < 0
                    || next_node.y < 0
                    || next_node.x > self.size
                    || next_node.y > self.size
                    || bad_bytes.contains(&next_node)
                    || cost + 1
                        >= scores
                            .get(&(next_node, next_dir))
                            .copied()
                            .unwrap_or(usize::MAX)
                {
                    continue;
                }

                queue.push((cost + 1, next_node, next_dir));
            }
        }

        scores
            .iter()
            .filter_map(|((n, _), v)| if *n == end { Some(*v) } else { None })
            .min()
        // *scores.get(&end).unwrap()
    }

    fn tick(&self, ticks: usize) -> &[Vec2] {
        &self.bytes[..ticks]
    }

    fn parse(input: &str) -> Self {
        let bytes = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                Vec2 { x, y }
            })
            .collect_vec();

        let size = bytes.iter().map(|v| v.x.max(v.y)).max().unwrap();

        Memory { size, bytes }
    }

    #[allow(dead_code)]
    fn debug_print(&self, ticks: usize) {
        let c: HashSet<Vec2> = self.tick(ticks).iter().copied().collect();
        for y in 0..=self.size {
            for x in 0..=self.size {
                let v = Vec2 { x, y };
                if c.contains(&v) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
