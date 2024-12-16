use std::collections::{HashMap, VecDeque};

use crate::utils::{input, Vec2};

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 16");
    let data = input(16, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let board = parse(input);
    let score = board.new_traverse();
    println!("P1: {score}");
}

fn p2(input: &str) {}

fn parse(input: &str) -> Board {
    let mut map = Vec::new();
    let mut pos = Vec2::default();
    let mut end = Vec2::default();
    for (y, line) in input.lines().enumerate() {
        map.push(Vec::new());
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                '.' => Tile::Floor,
                '#' => Tile::Wall,
                'S' => {
                    pos = (x as isize, y as isize).into();
                    Tile::Floor
                }
                'E' => {
                    end = (x as isize, y as isize).into();
                    Tile::Floor
                }
                _ => unreachable!(),
            };
            map[y].push(tile);
        }
    }
    Board {
        map,
        end,
        player: Player {
            finished: false,
            pos,
            dir: Dir::East,
            score: 0,
        },
    }
}

#[derive(Debug, Clone)]
struct Board {
    map: Vec<Vec<Tile>>,
    end: Vec2,
    player: Player,
}

impl Board {
    fn new_traverse(&self) -> usize {
        let mut scores = HashMap::new();
        scores.insert(self.player.pos, 0usize);
        let mut queue = VecDeque::from([self.player]);
        let (a, b) = self.player.cross(self);
        queue.extend([a, b].into_iter().flatten());

        while let Some(mut player) = queue.pop_front() {
            while !player.finished {
                // Move the player forward
                let new_players = player.advance(self);

                // Get the score for the current cell
                let s = scores.entry(player.pos).or_insert(usize::MAX);

                // If we already have a lower score for this cell then we can skip it
                if player.score > *s {
                    break;
                }

                // Store the score for the current cell
                *s = player.score.min(*s);

                // If we have reached the end cell, then stop
                if player.pos == self.end {
                    break;
                }

                // Keep evaluating next players
                queue.extend(new_players.into_iter().flatten());
            }
        }

        *scores.get(&self.end).unwrap()
    }

    #[allow(dead_code)]
    fn print_debug(&self) {
        for (y, line) in self.map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let pos: Vec2 = (x as isize, y as isize).into();
                if pos == self.player.pos {
                    print!("S");
                } else if pos == self.end {
                    print!("E");
                } else {
                    print!("{tile}");
                }
            }
            println!()
        }
    }

    fn get(&self, pos: Vec2) -> Option<Tile> {
        self.map
            .get(pos.y as usize)
            .and_then(|line| line.get(pos.x as usize))
            .copied()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Floor,
    Wall,
}

impl Tile {
    fn is_traversable(&self) -> bool {
        match self {
            Tile::Floor => true,
            Tile::Wall => false,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
        };
        write!(f, "{ch}")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn to(&self, other: Dir) -> usize {
        match self {
            Dir::North => match other {
                Dir::North => 0,
                Dir::South => 2000,
                Dir::East => 1000,
                Dir::West => 1000,
            },
            Dir::South => match other {
                Dir::North => 2000,
                Dir::South => 0,
                Dir::East => 1000,
                Dir::West => 1000,
            },
            Dir::East => match other {
                Dir::North => 1000,
                Dir::South => 1000,
                Dir::East => 0,
                Dir::West => 2000,
            },
            Dir::West => match other {
                Dir::North => 1000,
                Dir::South => 1000,
                Dir::East => 2000,
                Dir::West => 0,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Player {
    finished: bool,
    pos: Vec2,
    dir: Dir,
    score: usize,
}

impl Player {
    fn advance(&mut self, board: &Board) -> [Option<Player>; 2] {
        match self.dir {
            Dir::North => self.north(),
            Dir::South => self.south(),
            Dir::East => self.east(),
            Dir::West => self.west(),
        }

        if self.pos == board.end {
            self.finished = true;
            return [None, None];
        }

        let tile = board.get(self.pos).unwrap();
        if tile.is_traversable() {
            let (a, b) = self.cross(board);
            [a, b]
        } else {
            self.finished = true;
            [None, None]
        }
    }

    fn east_west_cross(&self, board: &Board) -> (Option<Player>, Option<Player>) {
        let east = board
            .get(self.pos.right())
            .filter(Tile::is_traversable)
            .map(|_| {
                let mut east = *self;
                east.east();
                east
            });

        let west = board
            .get(self.pos.left())
            .filter(Tile::is_traversable)
            .map(|_| {
                let mut west = *self;
                west.west();
                west
            });

        (east, west)
    }

    fn north_south_cross(&self, board: &Board) -> (Option<Player>, Option<Player>) {
        let north = board
            .get(self.pos.up())
            .filter(Tile::is_traversable)
            .map(|_| {
                let mut north = *self;
                north.north();
                north
            });
        let south = board
            .get(self.pos.down())
            .filter(Tile::is_traversable)
            .map(|_| {
                let mut south = *self;
                south.south();
                south
            });

        (north, south)
    }

    fn north(&mut self) {
        self.pos = self.pos.up();
        self.score += self.dir.to(Dir::North) + 1;
        self.dir = Dir::North;
    }

    fn south(&mut self) {
        self.pos = self.pos.down();
        self.score += self.dir.to(Dir::South) + 1;
        self.dir = Dir::South;
    }

    fn west(&mut self) {
        self.pos = self.pos.left();
        self.score += self.dir.to(Dir::West) + 1;
        self.dir = Dir::West;
    }

    fn east(&mut self) {
        self.pos = self.pos.right();
        self.score += self.dir.to(Dir::East) + 1;
        self.dir = Dir::East;
    }

    fn cross(&self, board: &Board) -> (Option<Player>, Option<Player>) {
        match self.dir {
            Dir::North | Dir::South => self.east_west_cross(board),
            Dir::East | Dir::West => self.north_south_cross(board),
        }
    }
}
