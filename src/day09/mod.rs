use itertools::Itertools;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 9");
    let data = input(9, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let mut board = parse(input);
    sort_board(&mut board);
    let cs = checksum(&board);
    println!("P1: {cs}");
}

fn p2(input: &str) {
    let board = parse(input);
    let board = sort_board_improved(&board);
    let result = checksum(&board);
    println!("P2: {result}");
}

fn parse(input: &str) -> Vec<Option<usize>> {
    let mut v = Vec::new();
    let chars = input.trim().chars().collect_vec();
    for (id, ch) in chars.chunks(2).enumerate() {
        let file_blocks = ch[0].to_digit(10).unwrap();
        (0..file_blocks).for_each(|_| v.push(Some(id)));

        if let Some(space_blocks) = ch.get(1) {
            let space_blocks = space_blocks.to_digit(10).unwrap();
            (0..space_blocks).for_each(|_| v.push(None));
        }
    }
    v
}

#[allow(dead_code)]
fn board_to_string(board: &[Option<usize>]) -> String {
    board
        .iter()
        .map(|n| match n {
            Some(i) => i.to_string(),
            None => String::from("."),
        })
        .collect()
}

fn swap<T: Default>(board: &mut [T], first: usize, second: usize) {
    let f = std::mem::take(&mut board[first]);
    let s = std::mem::take(&mut board[second]);
    board[first] = s;
    board[second] = f;
}

fn sort_board(board: &mut [Option<usize>]) {
    loop {
        let first_space = board
            .iter()
            .enumerate()
            .find_map(|(idx, opt)| if opt.is_none() { Some(idx) } else { None })
            .unwrap();
        let last_block = board
            .iter()
            .enumerate()
            .rev()
            .find_map(|(idx, opt)| if opt.is_some() { Some(idx) } else { None })
            .unwrap();
        // Sorted!
        if first_space == last_block + 1 {
            return;
        }
        swap(board, first_space, last_block);
    }
}

fn sort_board_improved(board: &[Option<usize>]) -> Vec<Option<usize>> {
    // Split the board into continguous blocks of either `None` or `Some(x)`
    let mut blocks = board
        .chunk_by(|l, r| *l == *r)
        .map(ToOwned::to_owned)
        .collect_vec();

    // A queue representing the various blocks to pull from
    let mut blocks_queue = blocks.iter().cloned().collect_vec();

    // For each block in the queue
    while let Some(next_file) = blocks_queue.pop() {
        // Sanity check
        if next_file.is_empty() {
            continue;
        }

        // If the block is none
        if next_file[0].is_none() {
            continue;
        }

        // Look for a space block that is sufficient length
        let space_position = blocks
            .iter()
            .find_position(|b| b.len() >= next_file.len() && b[0].is_none());

        if let Some((space_position, _)) = space_position {
            // Find where the current position of the file block is (since the canonical ordering
            // has now been resorted many times)
            let file_position = blocks.iter().find_position(|b| b == &&next_file).unwrap().0;

            // Don't move the block to the right of itself
            if space_position > file_position {
                continue;
            }

            // Move the file block and re-group them
            let (filled_space, emptied_file) =
                move_file(&blocks[space_position], &blocks[file_position]);
            blocks[space_position] = filled_space;
            blocks[file_position] = emptied_file;
            blocks = regroup(blocks);
        }
    }
    blocks.into_iter().flatten().collect_vec()
}

fn move_file(
    space: &[Option<usize>],
    file: &[Option<usize>],
) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let mut space = space.to_owned();
    let mut file = file.to_owned();
    space
        .iter_mut()
        .zip(file.iter_mut())
        .for_each(|(s, f)| *s = std::mem::take(f));
    (space, file)
}

fn regroup(blocks: Vec<Vec<Option<usize>>>) -> Vec<Vec<Option<usize>>> {
    blocks
        .into_iter()
        .flatten()
        .collect_vec()
        .chunk_by(|l, r| *l == *r)
        .map(ToOwned::to_owned)
        .collect_vec()
}

fn checksum(board: &[Option<usize>]) -> usize {
    board
        .iter()
        .enumerate()
        .map(|(idx, opt)| idx * opt.unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let board = parse("2333133121414131402");
        assert_eq!(
            board_to_string(&board),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_swap() {
        let mut board = vec![Some(0), None, Some(1)];
        swap(&mut board, 0, 2);
        assert_eq!(board, &[Some(1), None, Some(0)]);
    }

    #[test]
    fn test_move_file() {
        let result = move_file(&[None, None, None, None], &[Some(1), Some(2)]);
        assert_eq!(
            result,
            (vec![Some(1), Some(2), None, None], vec![None, None])
        );
    }
}
