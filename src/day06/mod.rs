use std::collections::HashSet;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 6");
    let data = input(6, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let (board, mut player) = parse(input);
    let mut cs = HashSet::new();
    cs.insert(player.coord);
    while let Some(next_player) = player.next(&board) {
        cs.insert(next_player.coord);
        player = next_player;
    }
    println!("P1: {}", cs.len());
}

fn p2(input: &str) {}

#[derive(Debug, Clone, PartialEq, Hash, Default, Copy, Eq)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn next(&self, dir: &Direction) -> Option<Coord> {
        Some(match dir {
            Direction::Up => Coord {
                x: self.x,
                y: self.y.checked_sub(1)?,
            },
            Direction::Down => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Coord {
                x: self.x.checked_sub(1)?,
                y: self.y,
            },
            Direction::Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
        })
    }

    fn from_board(&self, board: &Vec<Vec<char>>) -> Option<char> {
        board.get(self.y).and_then(|l| l.get(self.x)).copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub struct Player {
    coord: Coord,
    direction: Direction,
}

impl Player {
    fn next(&self, board: &Vec<Vec<char>>) -> Option<Player> {
        let nc = self.coord.next(&self.direction)?;
        match nc.from_board(board) {
            Some('.') => Some(Player {
                coord: nc,
                direction: self.direction,
            }),
            Some('#') => self.turn().next(board),
            Some(_) => unimplemented!(),
            None => None,
        }
    }

    fn turn(&self) -> Player {
        Player {
            coord: self.coord,
            direction: self.direction.next(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Default, Copy, Eq)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, Player) {
    let mut board = Vec::new();
    let mut player = Player::default();
    for (y, line) in input.lines().enumerate() {
        let mut lv = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            lv.push(match ch {
                '.' => '.',
                '#' => '#',
                '^' => {
                    player.coord = Coord { x, y };
                    '.'
                }
                _ => unimplemented!(),
            });
        }
        board.push(lv);
    }
    (board, player)
}

fn print_board(board: &Vec<Vec<char>>) {
    for line in board {
        for ch in line {
            print!("{ch}");
        }
        println!("");
    }
}
