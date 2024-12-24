// Shamelssly copied from https://github.com/michel-kraemer/adventofcode-rust/blob/main/2024/day21/src/main.rs
// because I just couldn't wrap my head around this

use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::utils::input;

pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const NUMERIC: [[u8; 3]; 4] = [
    [b'7', b'8', b'9'],
    [b'4', b'5', b'6'],
    [b'1', b'2', b'3'],
    [b' ', b'0', b'A'],
];

const DIRECTIONAL: [[u8; 3]; 2] = [[b' ', b'^', b'A'], [b'<', b'v', b'>']];

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 21");
    let data = input(21, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let mut cache = HashMap::new();
    let mut path_cache = [const { None }; 128 * 128];
    let result = input
        .lines()
        .map(|code| {
            (
                code,
                find_shortest_sequence(code.as_bytes(), 25, true, &mut cache, &mut path_cache),
            )
        })
        .map(|(code, seq)| code[..3].parse::<usize>().unwrap() * seq)
        .sum::<usize>();
    println!("P1: {result}");
}

fn p2(input: &str) {}

fn find_shortest_paths(
    keypad: &[[u8; 3]],
    from: u8,
    to: u8,
    cache: &mut [Option<Rc<Vec<Vec<u8>>>>; 128 * 128],
) -> Rc<Vec<Vec<u8>>> {
    if let Some(cached) = &cache[from as usize * 128 + to as usize] {
        return cached.clone();
    }

    if from == to {
        let result = Rc::new(vec![vec![b'A']]);
        cache[from as usize * 128 + to as usize] = Some(result.clone());
        return result;
    }

    // find 'from' and 'to' on keypad
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, row) in keypad.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == from {
                start = (x, y);
            }
            if c == to {
                end = (x, y);
            }
        }
    }

    // flood fill keypad to find the shortest distances
    let mut dists = vec![[usize::MAX; 3]; keypad.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        dists[y][x] = steps;
        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < 3
                && ny < keypad.len() as i32
                && keypad[ny as usize][nx as usize] != b' '
                && dists[ny as usize][nx as usize] == usize::MAX
            {
                queue.push_back((nx as usize, ny as usize, steps + 1));
            }
        }
    }

    // backtrack from 'end' back to 'start' and collect all paths
    let mut paths = Vec::new();
    let mut stack = Vec::new();
    stack.push((end.0, end.1, vec![b'A']));
    while let Some((x, y, path)) = stack.pop() {
        if x == start.0 && y == start.1 {
            paths.push(path);
            continue;
        }
        for (i, (dx, dy)) in DIRS.iter().enumerate() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < 3
                && ny < keypad.len() as i32
                && dists[ny as usize][nx as usize] < dists[y][x]
            {
                // do everything in reverse
                let c = match i {
                    0 => b'<',
                    1 => b'^',
                    2 => b'>',
                    3 => b'v',
                    _ => panic!(),
                };
                let mut new_path = vec![c];
                new_path.extend(&path);
                stack.push((nx as usize, ny as usize, new_path));
            }
        }
    }

    let result = Rc::new(paths);
    cache[from as usize * 128 + to as usize] = Some(result.clone());
    result
}

fn find_shortest_sequence(
    s: &[u8],
    depth: usize,
    highest: bool,
    cache: &mut HashMap<(Vec<u8>, usize), usize>,
    path_cache: &mut [Option<Rc<Vec<Vec<u8>>>>; 128 * 128],
) -> usize {
    let cache_key = (s.to_vec(), depth);
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    let mut cursor = b'A';
    let mut result = 0;
    for &c in s {
        let paths = find_shortest_paths(
            if highest { &NUMERIC } else { &DIRECTIONAL },
            cursor,
            c,
            path_cache,
        );
        if depth == 0 {
            // all paths have the same length
            result += paths[0].len();
        } else {
            result += paths
                .iter()
                .map(|p| find_shortest_sequence(p, depth - 1, false, cache, path_cache))
                .min()
                .unwrap();
        }
        cursor = c;
    }

    cache.insert(cache_key, result);

    result
}
