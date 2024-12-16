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
    for dir in directions {
        if move_robot(&mut board, dir, robot) {
            robot = robot + dir;
        }
    }
    let gps: usize = calc_gps(&board).into_iter().sum();
    println!("P1: {gps}");
}

fn p2(input: &str) {}

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

fn move_robot(board: &mut Board, dir: Vec2, pos: Vec2) -> bool {
    let next_pos = pos + dir;
    let next_tile = *board
        .get(next_pos.y as usize)
        .and_then(|line| line.get(next_pos.x as usize))
        .unwrap();
    match next_tile {
        Tile::Floor => {
            board[next_pos.y as usize][next_pos.x as usize] = board[pos.y as usize][pos.x as usize];
            board[pos.y as usize][pos.x as usize] = Tile::Floor;
            true
        }
        Tile::Box if move_robot(board, dir, next_pos) => {
            board[next_pos.y as usize][next_pos.x as usize] = board[pos.y as usize][pos.x as usize];
            board[pos.y as usize][pos.x as usize] = Tile::Floor;
            true
        }
        _ => false,
    }
}

fn calc_gps(board: &Board) -> Vec<usize> {
    let mut gps = Vec::with_capacity(100);
    for (y, line) in board.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if tile == &Tile::Box {
                gps.push(100 * y + x);
            }
        }
    }
    gps
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Floor,
    Wall,
    Box,
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

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
        };
        write!(f, "{ch}")
    }
}
