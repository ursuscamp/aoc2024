use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 2");
    let data = input(2, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let lists = to_lists(input);
    let safe = lists.iter().filter(|s| State::ingest(s).is_valid()).count();
    println!("P1: {safe}");
}

fn p2(input: &str) {
    let lists = to_lists(input);
    let safe = lists.iter().filter(|s| test_fully_valid(s)).count();
    println!("P2: {safe}");
}

fn to_lists(input: &str) -> Vec<Vec<i64>> {
    let mut l = Vec::new();

    for line in input.lines() {
        l.push(
            line.split_whitespace()
                .map(|s| s.trim().parse().unwrap())
                .collect(),
        );
    }

    l
}

fn in_range(l: i64, r: i64) -> bool {
    (1..=3).contains(&l.abs_diff(r))
}

fn test_fully_valid(data: &[i64]) -> bool {
    State::ingest(data).is_valid() || test_all_skips(data)
}

fn test_all_skips(data: &[i64]) -> bool {
    for i in 0..data.len() {
        let mut data = data.to_vec();
        data.remove(i);
        if State::ingest(&data).is_valid() {
            return true;
        }
    }
    false
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Start,
    Num(i64),
    Asc(i64),
    Desc(i64),
    Invalid,
}

impl State {
    fn is_valid(&self) -> bool {
        self != &State::Invalid
    }

    fn is_invalid(&self) -> bool {
        self == &State::Invalid
    }

    fn ingest(data: &[i64]) -> Self {
        let mut state = State::Start;
        for n in data {
            state.next(*n);
        }
        state
    }

    fn next(&mut self, next: i64) {
        let next = match *self {
            State::Start => State::Num(next),
            State::Num(i) if next > i && in_range(i, next) => State::Asc(next),
            State::Num(i) if next < i && in_range(i, next) => State::Desc(next),
            State::Num(_i) => State::Invalid,
            State::Asc(i) if next > i && in_range(i, next) => State::Asc(next),
            State::Asc(_i) => State::Invalid,
            State::Desc(i) if next < i && in_range(i, next) => State::Desc(next),
            State::Desc(_i) => State::Invalid,
            State::Invalid => State::Invalid,
        };
        *self = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        assert!(State::ingest(&[7, 6, 4, 2, 1]).is_valid());
        assert!(State::ingest(&[1, 2, 7, 8, 9]).is_invalid());
        assert!(State::ingest(&[9, 7, 6, 2, 1]).is_invalid());
        assert!(State::ingest(&[1, 3, 2, 4, 5]).is_invalid());
        assert!(State::ingest(&[8, 6, 4, 4, 1]).is_invalid());
        assert!(State::ingest(&[1, 3, 6, 7, 9]).is_valid());
    }

    #[test]
    fn test_with_skips() {
        assert!(test_fully_valid(&[7, 6, 4, 2, 1]));
        assert!(!test_fully_valid(&[1, 2, 7, 8, 9]));
        assert!(!test_fully_valid(&[9, 7, 6, 2, 1]));
        assert!(test_fully_valid(&[1, 3, 2, 4, 5]));
        assert!(test_fully_valid(&[8, 6, 4, 4, 1]));
        assert!(test_fully_valid(&[1, 3, 6, 7, 9]));
    }
}
