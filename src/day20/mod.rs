use core::panic;
use std::collections::{HashMap, HashSet};

use crate::utils::{input, Vec2};

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 20");
    let data = input(20, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let track = Track::parse(input);

    let cheats = track
        .count_cheats(2)
        .into_iter()
        .filter_map(|(saving, count)| if saving >= 100 { Some(count) } else { None })
        .sum::<usize>();
    println!("P1: {cheats}");
}

fn p2(input: &str) {
    let track = Track::parse(input);

    let cheats = track
        .count_cheats(20)
        .into_iter()
        .filter_map(|(saving, count)| if saving >= 100 { Some(count) } else { None })
        .sum::<usize>();
    println!("P2: {cheats}");
}

#[derive(Debug)]
struct Track {
    #[allow(dead_code)]
    size: Vec2,
    start: Vec2,
    #[allow(dead_code)]
    end: Vec2,
    track: HashMap<Vec2, usize>,
    #[allow(dead_code)]
    walls: HashSet<Vec2>,
}

impl Track {
    fn count_cheats(&self, cheat_range: usize) -> HashMap<usize, usize> {
        let mut cheats = HashMap::new();

        for (tile, tile_cost) in &self.track {
            let savings = self
                .track
                .iter()
                .filter_map(|(other_tile, other_cost)| {
                    let md = tile.manhattan_distance(other_tile);
                    if md <= cheat_range {
                        Some((other_tile, other_cost, md))
                    } else {
                        None
                    }
                })
                .filter_map(|(_other_tile, other_cost, md)| {
                    let savings = other_cost.saturating_sub(*tile_cost).saturating_sub(md);
                    if savings > 0 {
                        Some(savings)
                    } else {
                        None
                    }
                });

            for saving in savings {
                *cheats.entry(saving).or_default() += 1;
            }
        }

        cheats
    }

    fn assign_costs(&mut self) {
        let mut visited = HashSet::from([self.start]);
        let mut queue = Vec::from([(0usize, self.start)]);

        while let Some((cost, coord)) = queue.pop() {
            for coord in [coord.up(), coord.down(), coord.left(), coord.right()] {
                if !self.track.contains_key(&coord) {
                    continue;
                }

                if !visited.insert(coord) {
                    continue;
                }
                let cost = cost + 1;
                self.track.insert(coord, cost);
                queue.push((cost, coord));
            }
        }
    }

    fn parse(input: &str) -> Track {
        let mut size_x = 0isize;
        let mut size_y = 0isize;
        let mut track = HashMap::new();
        let mut walls = HashSet::new();
        let mut start = Vec2::default();
        let mut end = Vec2::default();
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                size_x = size_x.max(x as isize + 1);
                let v = Vec2 {
                    x: x as isize,
                    y: y as isize,
                };
                match ch {
                    '.' => {
                        track.insert(v, 0);
                    }
                    'S' => {
                        track.insert(v, 0);
                        start = v;
                    }
                    'E' => {
                        track.insert(v, 0);
                        end = v;
                    }
                    '#' => {
                        walls.insert(v);
                    }
                    _ => panic!("Unknown character {ch}"),
                }
            }
            size_y = size_y.max(y as isize + 1);
        }
        let mut track = Track {
            size: Vec2 {
                x: size_x,
                y: size_y,
            },
            start,
            end,
            track,
            walls,
        };

        track.assign_costs();

        track
    }

    #[allow(dead_code)]
    fn print_debug(&self) {
        for y in 0..self.size.y {
            print!("{y:<5}");
            for x in 0..self.size.x {
                let v = Vec2 { x, y };
                let tile = if self.start == v {
                    'S'
                } else if self.end == v {
                    'E'
                } else if self.track.contains_key(&v) {
                    '.'
                } else {
                    '#'
                };
                print!("{tile}");
            }
            println!();
        }
    }
}
