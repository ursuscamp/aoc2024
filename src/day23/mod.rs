use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 23");
    let data = input(23, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let graph = parse(input);
    let connections = interconnected_computers(3, &graph);
    println!("P1: {}", connections.len());
}

fn p2(input: &str) {}

fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();
        let l = l.to_string();
        let r = r.to_string();

        map.entry(l.clone()).or_default().insert(r.clone());
        map.entry(r.clone()).or_default().insert(l.clone());
    }

    map
}

fn interconnected_computers(
    num: usize,
    graph: &HashMap<String, HashSet<String>>,
) -> Vec<Vec<&String>> {
    unique(find_connections(num, graph))
        .into_iter()
        .filter(|cl| cl.iter().any(|s| s.starts_with('t')))
        .collect_vec()
}

fn unique(items: Vec<Vec<&String>>) -> Vec<Vec<&String>> {
    items
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .unique()
        .collect_vec()
}

fn find_connections(num: usize, graph: &HashMap<String, HashSet<String>>) -> Vec<Vec<&String>> {
    let mut results = graph.keys().combinations(num).collect_vec();

    results.retain(|r| {
        for i in 0..r.len() {
            for j in 0..r.len() {
                if i == j {
                    continue;
                }
                if !graph[r[i]].contains(r[j]) {
                    return false;
                }
            }
        }
        true
    });

    results
}
