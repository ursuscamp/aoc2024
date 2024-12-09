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
    let antinodes = find_antinodes(&board, false);
    println!("P1: {}", antinodes.len());
}

fn p2(input: &str) {
    let board = parse(input);
    let antinodes = find_antinodes(&board, true);
    println!("P1: {}", antinodes.len());
}

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

fn find_antinodes(board: &[Vec<char>], resonance: bool) -> HashSet<Coord> {
    let nodes = find_nodes(board);
    let mut antinodes = HashSet::new();
    for (_node, coords) in nodes {
        for perms in coords.into_iter().permutations(2) {
            let lan = if resonance {
                resonant_line_antinodes(board, perms[0], perms[1])
            } else {
                line_antinodes(board, perms[0], perms[1])
            };
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
        if is_on_board(board, x, y) {
            antinodes.push(Coord {
                x: x as usize,
                y: y as usize,
            });
        }
    }

    #[allow(clippy::comparison_chain)]
    let rise = flip_sign(rise);

    #[allow(clippy::comparison_chain)]
    let run = flip_sign(run);

    {
        let x = b.x as isize + run;
        let y = b.y as isize + rise;
        if is_on_board(board, x, y) {
            antinodes.push(Coord {
                x: x as usize,
                y: y as usize,
            });
        }
    }
    antinodes
}

fn resonant_line_antinodes(board: &[Vec<char>], a: Coord, b: Coord) -> Vec<Coord> {
    let mut antinodes = Vec::new();

    // All points in the line are now antinodes becuase of resonance, which includes the antenna
    // points.
    antinodes.extend([a, b]);

    let (rise, run) = slope(a, b);

    let mut a = a;
    loop {
        let x = a.x as isize + run;
        let y = a.y as isize + rise;
        if is_on_board(board, x, y) {
            let coord = Coord {
                x: x as usize,
                y: y as usize,
            };
            antinodes.push(coord);
            a = coord;
        } else {
            break;
        }
    }

    #[allow(clippy::comparison_chain)]
    let rise = flip_sign(rise);

    #[allow(clippy::comparison_chain)]
    let run = flip_sign(run);

    let mut b = b;
    loop {
        let x = b.x as isize + run;
        let y = b.y as isize + rise;
        if is_on_board(board, x, y) {
            let coord = Coord {
                x: x as usize,
                y: y as usize,
            };
            antinodes.push(coord);
            b = coord;
        } else {
            break;
        }
    }
    antinodes
}

fn slope(a: Coord, b: Coord) -> (isize, isize) {
    (a.y as isize - b.y as isize, a.x as isize - b.x as isize)
}

fn is_on_board(board: &[Vec<char>], x: isize, y: isize) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    let x = x as usize;
    let y = y as usize;

    y < board.len() && x < board[0].len()
}

fn flip_sign(n: isize) -> isize {
    if n < 0 {
        n.abs()
    } else if n > 0 {
        0 - n
    } else {
        n
    }
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
