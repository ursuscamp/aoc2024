use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

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
    let min_cost = memory.find_path_nonrec(ticks);
    // let min_cost = memory.find_path(ticks);
    println!("P1: {}", min_cost.unwrap());
}

fn p2(input: &str) {}

#[derive(Debug)]
struct Memory {
    size: isize,
    bytes: Vec<Vec2>,
}

impl Memory {
    fn find_path(&self, ticks: usize) -> usize {
        let bad_bytes: HashSet<Vec2> = self.tick(ticks).iter().copied().collect();
        let mut visited = HashSet::new();
        self._find_path(&bad_bytes, usize::MAX, Vec2 { x: 0, y: 0 }, &mut visited)
            .unwrap();
        visited.len() - 1
    }

    fn _find_path(
        &self,
        bad_bytes: &HashSet<Vec2>,
        cost: usize,
        node: Vec2,
        visited: &mut HashSet<Vec2>,
    ) -> Option<usize> {
        visited.insert(node);

        if node == self.end() {
            println!("End found: {cost}");
            return Some(cost);
        }

        let next = [node.right(), node.down(), node.up(), node.left()]
            .into_iter()
            .filter_map(|nn| {
                if nn.x < 0
                    || nn.y < 0
                    || nn.x > self.size
                    || nn.y > self.size
                    || bad_bytes.contains(&nn)
                    || visited.contains(&nn)
                {
                    return None;
                }
                self._find_path(bad_bytes, cost + 1, nn, visited)
            })
            .min();

        if next.is_none() {
            visited.remove(&node);
        }

        next
    }

    fn end(&self) -> Vec2 {
        Vec2 {
            x: self.size,
            y: self.size,
        }
    }

    fn find_path_nonrec(&self, ticks: usize) -> Option<usize> {
        let bad_bytes: HashSet<Vec2> = self.tick(ticks).iter().copied().collect();
        // let mut queue = BinaryHeap::from([(0usize, Vec2 { x: 0, y: 0 })]);
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
