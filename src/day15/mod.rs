use itertools::Itertools;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 15");
    let data = input(15, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let (mut board, directions, mut robot) = parse(input);
    #[allow(clippy::never_loop)]
    for dir in directions {
        if let Some(nb) = push(&board, dir, robot + dir) {
            board = nb;
            robot = robot + dir;
        }
    }
    let gps: usize = calc_gps(&board).into_iter().sum();
    println!("P1: {gps}");
}

fn p2(input: &str) {
    let (board, directions, robot) = parse(input);
    let mut board = extend_board(&board);
    let mut robot = extend_robot(robot);
    for dir in directions {
        if let Some(nb) = push(&board, dir, robot + dir) {
            board = nb;
            robot = robot + dir;
        }
    }
    let gps: usize = calc_gps(&board).into_iter().sum();
    println!("P2: {gps}");
}

fn push(board: &Board, dir: Vec2, pos: Vec2) -> Option<Board> {
    let tile = board[pos.y as usize][pos.x as usize];
    let np = pos + dir;
    match tile {
        Tile::Floor => Some(board.clone()),
        Tile::Wall => None,
        Tile::Box => {
            if let Some(mut nb) = push(board, dir, np) {
                nb[np.y as usize][np.x as usize] = Tile::Box;
                nb[pos.y as usize][pos.x as usize] = Tile::Floor;
                Some(nb)
            } else {
                None
            }
        }
        Tile::LBox if dir.x == 0 => {
            if let Some(nb) = push(board, dir, pos + dir) {
                if let Some(mut nb) = push(&nb, dir, pos.right() + dir) {
                    let rbox = pos.right();
                    let nrbox = rbox + dir;
                    nb[np.y as usize][np.x as usize] = Tile::LBox;
                    nb[nrbox.y as usize][nrbox.x as usize] = Tile::RBox;
                    nb[pos.y as usize][pos.x as usize] = Tile::Floor;
                    nb[rbox.y as usize][rbox.x as usize] = Tile::Floor;
                    Some(nb)
                } else {
                    None
                }
            } else {
                None
            }
        }

        Tile::LBox if dir.x == -1 => {
            let left = pos.left();
            let lbox = pos;
            let rbox = pos.right();
            if let Some(mut nb) = push(board, dir, left) {
                nb[left.y as usize][left.x as usize] = Tile::LBox;
                nb[lbox.y as usize][lbox.x as usize] = Tile::RBox;
                nb[rbox.y as usize][rbox.x as usize] = Tile::Floor;
                Some(nb)
            } else {
                None
            }
        }

        Tile::LBox if dir.x == 1 => {
            let right = pos.right().right();
            let rbox = pos.right();
            let lbox = pos;
            if let Some(mut nb) = push(board, dir, right) {
                nb[right.y as usize][right.x as usize] = Tile::RBox;
                nb[rbox.y as usize][rbox.x as usize] = Tile::LBox;
                nb[lbox.y as usize][lbox.x as usize] = Tile::Floor;
                Some(nb)
            } else {
                None
            }
        }

        Tile::RBox => push(board, dir, pos.left()),
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> (Board, Vec<Vec2>, Vec2) {
    let mut board = Vec::new();
    let mut dirs = Vec::new();
    let mut parse_board = true;
    let mut robot = Vec2::default();

    for (y, line) in input.lines().enumerate() {
        if parse_board {
            if line.is_empty() {
                parse_board = false;
                continue;
            }
            board.push(
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| match ch {
                        '@' => {
                            robot = (x as isize, y as isize).into();
                            Tile::Floor
                        }
                        _ => Tile::from(ch),
                    })
                    .collect(),
            );
        } else {
            dirs.extend(line.chars().map(|ch| match ch {
                '>' => Vec2::from((1isize, 0isize)),
                '<' => (-1, 0).into(),
                '^' => (0, -1).into(),
                'v' => (0, 1).into(),
                _ => unreachable!(),
            }));
        }
    }

    (board, dirs, robot)
}

fn extend_board(board: &Board) -> Board {
    board
        .clone()
        .into_iter()
        .map(|line| {
            line.into_iter()
                .flat_map(|tile| match tile {
                    Tile::Box => [Tile::LBox, Tile::RBox],
                    _ => [tile, tile],
                })
                .collect_vec()
        })
        .collect_vec()
}

fn extend_robot(robot: Vec2) -> Vec2 {
    Vec2 {
        x: robot.x * 2,
        y: robot.y,
    }
}

#[allow(dead_code)]
fn debug_board(board: &Board, robot: Vec2) {
    for (y, line) in board.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if robot == (x as isize, y as isize).into() {
                print!("@");
            } else {
                print!("{tile}");
            }
        }
        println!()
    }
}

fn calc_gps(board: &Board) -> Vec<usize> {
    let mut gps = Vec::with_capacity(100);
    for (y, line) in board.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match *tile {
                Tile::LBox | Tile::Box => gps.push(100 * y + x),
                _ => {}
            };
        }
    }
    gps
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Floor,
    Wall,
    Box,
    LBox,
    RBox,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            'O' => Tile::Box,
            _ => unreachable!(),
        }
    }
}

type Board = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn right(&self) -> Vec2 {
        Vec2 {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn left(&self) -> Vec2 {
        Vec2 {
            x: self.x - 1,
            y: self.y,
        }
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from(value: (isize, isize)) -> Self {
        Vec2 {
            x: value.0,
            y: value.1,
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::LBox => '[',
            Tile::RBox => ']',
        };
        write!(f, "{ch}")
    }
}
