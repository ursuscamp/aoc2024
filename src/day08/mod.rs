use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 8");
    let data = input(8, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let board = parse(input);
    let antinodes = find_antinodes(&board);
    println!("P1: {}", antinodes.len());
}

fn p2(input: &str) {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn find_nodes(board: &[Vec<char>]) -> HashMap<char, HashSet<Coord>> {
    let mut nodes: HashMap<char, HashSet<Coord>> = HashMap::new();
    for (y, line) in board.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == '.' {
                continue;
            }
            let coord = Coord { x, y };
            nodes.entry(*ch).or_default().insert(coord);
        }
    }
    nodes
}

fn find_antinodes(board: &[Vec<char>]) -> HashSet<Coord> {
    let nodes = find_nodes(board);
    let mut antinodes = HashSet::new();
    for (_node, coords) in nodes {
        for perms in coords.into_iter().permutations(2) {
            let lan = line_antinodes(board, perms[0], perms[1]);
            antinodes.extend(lan);
        }
    }
    antinodes
}

fn line_antinodes(board: &[Vec<char>], a: Coord, b: Coord) -> Vec<Coord> {
    let mut antinodes = Vec::new();
    let (rise, run) = slope(a, b);
    {
        let x = a.x as isize + run;
        let y = a.y as isize + rise;
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            if y < board.len() && x < board[0].len() {
                antinodes.push(Coord { x, y });
            }
        }
    }

    #[allow(clippy::comparison_chain)]
    let rise = if rise < 0 {
        rise.abs()
    } else if rise > 0 {
        0 - rise
    } else {
        rise
    };

    #[allow(clippy::comparison_chain)]
    let run = if run < 0 {
        run.abs()
    } else if run > 0 {
        0 - run
    } else {
        run
    };

    {
        let x = b.x as isize + run;
        let y = b.y as isize + rise;
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            if board.get(y).and_then(|l| l.get(x)).is_some() {
                antinodes.push(Coord { x, y });
            }
        }
    }
    antinodes
}

fn slope(a: Coord, b: Coord) -> (isize, isize) {
    (a.y as isize - b.y as isize, a.x as isize - b.x as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope() {
        let a = Coord { x: 4, y: 3 };
        let b = Coord { x: 5, y: 5 };
        assert_eq!(slope(a, b), (-2, -1));
    }

    #[test]
    fn test_line_antinodes() {
        let input = "..........
                           ...#......
                           #.........
                           ....a.....
                           ........a.
                           .....a....
                           ..#.......
                           ......#...
                           ..........
                           ..........";
        let board = parse(input);
        let a = Coord { x: 4, y: 3 };
        let b = Coord { x: 5, y: 5 };
        assert_eq!(
            line_antinodes(&board, a, b),
            vec![Coord { x: 3, y: 1 }, Coord { x: 6, y: 7 }]
        );

        let a = Coord { x: 5, y: 5 };
        let b = Coord { x: 8, y: 4 };
        assert_eq!(line_antinodes(&board, a, b), vec![Coord { x: 2, y: 6 }]);
    }
}
