use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 4");
    let data = input(4, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let input = parse_input(input);
    let count = find_xmas_count(&input);
    println!("P1: {count:?}");
}

fn p2(input: &str) {
    let input = parse_input(input);
    let count = find_x_mas_count(&input);
    println!("P2: {count:?}");
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }

    fn in_input(&self, input: &Vec<Vec<char>>) -> Option<char> {
        input.get(self.y)?.get(self.x).copied()
    }

    fn offset_horz(&self, offset: usize) -> Coord {
        Coord::new(self.x + offset, self.y)
    }

    fn offset_vert(&self, offset: usize) -> Coord {
        Coord::new(self.x, self.y + offset)
    }

    fn offset_diag_down(&self, offset: usize) -> Coord {
        Coord::new(self.x + offset, self.y + offset)
    }

    fn offset_diag_up(&self, offset: usize) -> Option<Coord> {
        Some(Coord::new(self.x + offset, self.y.checked_sub(offset)?))
    }

    fn search_set(&self) -> Vec<CoordSet> {
        let mut set = Vec::new();
        set.push(CoordSet(
            *self,
            self.offset_horz(1),
            self.offset_horz(2),
            self.offset_horz(3),
        ));
        set.push(CoordSet(
            *self,
            self.offset_vert(1),
            self.offset_vert(2),
            self.offset_vert(3),
        ));
        set.push(CoordSet(
            *self,
            self.offset_diag_down(1),
            self.offset_diag_down(2),
            self.offset_diag_down(3),
        ));

        // Diagonal up may not exist
        let mas = self
            .offset_diag_up(1)
            .and_then(|m| Some((m, self.offset_diag_up(2)?)))
            .and_then(|(m, a)| Some((m, a, self.offset_diag_up(3)?)));
        if let Some((m, a, s)) = mas {
            set.push(CoordSet(*self, m, a, s));
        }
        set
    }

    fn x_search_set(&self) -> Option<Vec<XCoordSet>> {
        let a = XCoordSet(*self, self.offset_diag_down(1), self.offset_diag_down(2));
        let b = Some(self.offset_vert(2))
            .and_then(|m| Some((m, m.offset_diag_up(1)?)))
            .and_then(|(m, a)| Some((m, a, a.offset_diag_up(1)?)))
            .map(|s| XCoordSet(s.0, s.1, s.2))?;
        Some(vec![a, b])
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct CoordSet(Coord, Coord, Coord, Coord);

impl CoordSet {
    fn in_input(&self, input: &Vec<Vec<char>>) -> Option<(char, char, char, char)> {
        let x = self.0.in_input(input)?;
        let m = self.1.in_input(input)?;
        let a = self.2.in_input(input)?;
        let s = self.3.in_input(input)?;
        Some((x, m, a, s))
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct XCoordSet(Coord, Coord, Coord);

impl XCoordSet {
    fn in_input(&self, input: &Vec<Vec<char>>) -> Option<(char, char, char)> {
        let m = self.0.in_input(input)?;
        let a = self.1.in_input(input)?;
        let s = self.2.in_input(input)?;
        Some((m, a, s))
    }
}

fn find_xmas_count(input: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let mut elim_board = vec![vec!['.'; input[0].len()]; input.len()];

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let coord = Coord::new(x, y);
            let sets = coord.search_set();
            for set in sets {
                let word = set.in_input(input);
                match word {
                    Some(('X', 'M', 'A', 'S')) | Some(('S', 'A', 'M', 'X')) => {
                        elim_board[set.0.y][set.0.x] = set.0.in_input(input).unwrap();
                        elim_board[set.1.y][set.1.x] = set.1.in_input(input).unwrap();
                        elim_board[set.2.y][set.2.x] = set.2.in_input(input).unwrap();
                        elim_board[set.3.y][set.3.x] = set.3.in_input(input).unwrap();
                        count += 1
                    }
                    _ => {}
                };
            }
        }
    }
    print_board(&elim_board);

    count
}

fn find_x_mas_count(input: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let coord = Coord::new(x, y);
            let sets = match coord.x_search_set() {
                Some(sets) => sets,
                None => continue,
            };

            let all = sets.into_iter().all(|set| {
                let word = set.in_input(input);
                matches!(word, Some(('M', 'A', 'S') | ('S', 'A', 'M')))
            });

            if all {
                count += 1;
            }
        }
    }
    count
}

fn print_board(input: &Vec<Vec<char>>) {
    for line in input {
        let s = line.iter().collect::<String>();
        println!("{s}");
    }
}
