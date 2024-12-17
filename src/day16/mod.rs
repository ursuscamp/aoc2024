use std::{collections::BinaryHeap, sync::atomic::AtomicUsize};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::utils::{input, Vec2};

static PLAYER_ID: AtomicUsize = AtomicUsize::new(0);

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 16");
    let data = input(16, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let board = parse(input);
    let (score, count) = board.new_traverse();
    println!("P1: {score}");
    println!("P2: {count}");
}

fn p2(_input: &str) {}

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
        player: Player::new(pos),
    }
}

#[derive(Debug, Clone)]
struct Board {
    map: Vec<Vec<Tile>>,
    end: Vec2,
    player: Player,
}

impl Board {
    fn new_traverse(&self) -> (usize, usize) {
        let mut scores = FxHashMap::default();
        scores.insert((self.player.pos, self.player.dir), 0usize);

        let mut queue = BinaryHeap::from([self.player.clone()]);
        queue.extend(self.player.cross());

        let mut visited: FxHashMap<usize, FxHashSet<Vec2>> = FxHashMap::default();
        visited
            .entry(self.player.id)
            .or_default()
            .insert(self.player.pos);
        let mut happy_player: Vec<Player> = Vec::new();

        while let Some(mut player) = queue.pop() {
            loop {
                // Move the player forward
                let new_players = player.advance();

                visited.entry(player.id).or_default().insert(player.pos);

                let score = scores.entry((player.pos, player.dir)).or_insert(usize::MAX);

                if self.is_wall(player.pos) {
                    visited.remove(&player.id);
                    break;
                }

                if player.score > *score {
                    visited.remove(&player.id);
                    break;
                }

                *score = player.score.min(*score);

                if self.is_end(player.pos) {
                    happy_player.push(player);
                    break;
                }

                // Keep evaluating next players
                let nv = visited.get(&player.id).unwrap().clone();
                visited
                    .entry(new_players[0].id)
                    .or_insert_with(|| nv.clone());
                visited.entry(new_players[1].id).or_insert_with(|| nv);
                queue.extend(new_players);
            }
        }

        let lowest_score = scores
            .into_iter()
            .filter_map(|((p, _d), v)| if p == self.end { Some(v) } else { None })
            .min()
            .unwrap();
        let mut success_set = happy_player
            .into_iter()
            .filter_map(|player| {
                if player.score <= lowest_score {
                    Some(player.id)
                } else {
                    None
                }
            })
            .flat_map(|player_id| &visited[&player_id])
            .copied()
            .collect::<FxHashSet<_>>();
        success_set.insert(self.player.pos);
        (lowest_score, success_set.len())
    }

    #[allow(dead_code)]
    fn print_debug(&self, markers: Option<FxHashSet<Vec2>>) {
        let markers = markers.unwrap_or_default();
        for (y, line) in self.map.iter().enumerate() {
            print!("{y:>3}: ");
            for (x, tile) in line.iter().enumerate() {
                let pos: Vec2 = (x as isize, y as isize).into();
                if markers.contains(&pos) {
                    print!("O");
                } else if pos == self.player.pos {
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

    fn is_wall(&self, pos: Vec2) -> bool {
        self.get(pos) == Some(Tile::Wall)
    }

    fn is_end(&self, pos: Vec2) -> bool {
        pos == self.end
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Floor,
    Wall,
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

#[derive(Debug, PartialEq, Eq)]
struct Player {
    id: usize,
    pos: Vec2,
    dir: Dir,
    score: usize,
}

impl Player {
    fn new(pos: Vec2) -> Player {
        Player {
            id: PLAYER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            pos,
            dir: Dir::East,
            score: 0,
        }
    }

    fn advance(&mut self) -> [Player; 2] {
        self.go(self.dir);

        self.cross()
    }

    fn east_west_cross(&self) -> [Player; 2] {
        let east = {
            let mut east = self.clone();
            east.turn(Dir::East);
            east
        };
        let west = {
            let mut west = self.clone();
            west.turn(Dir::West);
            west
        };

        [east, west]
    }

    fn north_south_cross(&self) -> [Player; 2] {
        let north = {
            let mut north = self.clone();
            north.turn(Dir::North);
            north
        };
        let south = {
            let mut south = self.clone();
            south.turn(Dir::South);
            south
        };

        [north, south]
    }

    fn turn(&mut self, dir: Dir) {
        self.score += self.dir.to(dir);
        self.dir = dir;
    }

    fn go(&mut self, dir: Dir) {
        self.turn(dir);
        self.pos = match dir {
            Dir::North => self.pos.up(),
            Dir::South => self.pos.down(),
            Dir::East => self.pos.right(),
            Dir::West => self.pos.left(),
        };
        self.score += 1;
    }

    fn cross(&self) -> [Player; 2] {
        match self.dir {
            Dir::North | Dir::South => self.east_west_cross(),
            Dir::East | Dir::West => self.north_south_cross(),
        }
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Self {
            id: PLAYER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            pos: self.pos,
            dir: self.dir,
            score: self.score,
        }
    }
}
