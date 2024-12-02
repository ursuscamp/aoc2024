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
    let safe = lists.iter().filter(|s| is_safe(s)).count();
    println!("Safe lists: {safe}");
}

fn p2(input: &str) {}

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

fn is_safe(list: &[i64]) -> bool {
    if !is_ascending(list) && !is_descending(list) {
        return false;
    }
    is_tolerance(list)
}

fn is_descending(list: &[i64]) -> bool {
    list.windows(2).all(|n| n[0] >= n[1])
}

fn is_ascending(list: &[i64]) -> bool {
    list.windows(2).all(|n| n[1] >= n[0])
}

fn is_tolerance(list: &[i64]) -> bool {
    list.windows(2).all(|n| {
        let tolerance = n[0].abs_diff(n[1]);
        (1..=3).contains(&tolerance)
    })
}
