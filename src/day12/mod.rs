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
    let cost = calculate_bulk_cost(&board);
    println!("P2: {cost:?}");
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

fn find_regions(board: &HashMap<Coord, char>) -> Vec<HashSet<Coord>> {
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
    regions
}

fn calculate_cost(board: &HashMap<Coord, char>) -> usize {
    find_regions(board).iter().map(region_cost).sum()
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

fn calculate_bulk_cost(board: &HashMap<Coord, char>) -> usize {
    find_regions(board).iter().map(region_bulk_cost).sum()
}

fn region_bulk_cost(region: &HashSet<Coord>) -> usize {
    count_region_sides(region) * region.len()
}

fn count_region_sides(region: &HashSet<Coord>) -> usize {
    let minx = region.iter().map(Coord::x).min().unwrap();
    let maxx = region.iter().map(Coord::x).max().unwrap();
    let miny = region.iter().map(Coord::y).min().unwrap();
    let maxy = region.iter().map(Coord::y).max().unwrap();

    let mut counter = 0;
    let mut visited_top = HashSet::new();
    let mut visited_bottom = HashSet::new();
    let mut visited_left = HashSet::new();
    let mut visited_right = HashSet::new();

    for y in miny..=maxy {
        for x in minx..=maxx {
            let cur_coord = Coord { x, y };

            let mut coord = cur_coord;
            if coord.is_top_border(region) && !visited_top.contains(&coord) {
                counter += 1;

                while coord.is_top_border(region) {
                    visited_top.insert(coord);
                    coord = coord.right();
                }
            }

            coord = cur_coord;
            if coord.is_bottom_border(region) && !visited_bottom.contains(&coord) {
                counter += 1;

                while coord.is_bottom_border(region) {
                    visited_bottom.insert(coord);
                    coord = coord.right();
                }
            }

            coord = cur_coord;
            if coord.is_left_border(region) && !visited_left.contains(&coord) {
                counter += 1;

                while coord.is_left_border(region) {
                    visited_left.insert(coord);
                    coord = coord.down();
                }
            }

            coord = cur_coord;
            if coord.is_right_border(region) && !visited_right.contains(&coord) {
                counter += 1;

                while coord.is_right_border(region) {
                    visited_right.insert(coord);
                    coord = coord.down();
                }
            }
        }
    }

    counter
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

    fn is_top_border(&self, region: &HashSet<Coord>) -> bool {
        region.contains(self) && !region.contains(&self.up())
    }

    fn is_bottom_border(&self, region: &HashSet<Coord>) -> bool {
        region.contains(self) && !region.contains(&self.down())
    }

    fn is_left_border(&self, region: &HashSet<Coord>) -> bool {
        region.contains(self) && !region.contains(&self.left())
    }

    fn is_right_border(&self, region: &HashSet<Coord>) -> bool {
        region.contains(self) && !region.contains(&self.right())
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

    fn x(&self) -> isize {
        self.x
    }

    fn y(&self) -> isize {
        self.y
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

    #[test]
    fn test_regions() {
        #[rustfmt::skip]
        let regions = vec![
            vec![
                vec![1,1,1],
                vec![1,1,1],
                vec![1,1,1],
            ],
            vec![
                vec![1,1,1,1],
                vec![1,1,0,0],
                vec![1,1,0,0],
            ]
        ];

        let regions = regions
            .into_iter()
            .map(|regvec| {
                regvec
                    .into_iter()
                    .enumerate()
                    .flat_map(move |(y, line)| {
                        line.into_iter().enumerate().filter_map(move |(x, n)| {
                            if n == 1 {
                                Some(Coord {
                                    x: x as isize,
                                    y: y as isize,
                                })
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<HashSet<_>>()
            })
            .collect_vec();

        assert!(Coord { x: 3, y: 0 }.is_right_border(&regions[1]));
        assert!(Coord { x: 3, y: 0 }.is_top_border(&regions[1]));
        assert!(Coord { x: 3, y: 0 }.is_bottom_border(&regions[1]));
        assert!(!Coord { x: 3, y: 0 }.is_left_border(&regions[1]));
        assert_eq!(count_region_sides(&regions[0]), 4);
        assert_eq!(count_region_sides(&regions[1]), 6);
    }
}
