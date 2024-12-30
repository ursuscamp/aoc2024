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

fn p2(input: &str) {
    let graph = parse(input);
    let neighbors = find_maximum_clique(&graph);
    let password = neighbors.into_iter().sorted().join(",");
    println!("P2: {:?}", password);
}

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

fn find_maximum_clique(graph: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut max_clique: HashSet<String> = HashSet::new();
    let mut current_clique: HashSet<String> = HashSet::new();

    // Convert all nodes to a vector for easier iteration
    let nodes: Vec<String> = graph.keys().cloned().collect();

    // Helper function to check if a node can be added to current clique
    fn is_connected_to_all(
        node: &str,
        clique: &HashSet<String>,
        graph: &HashMap<String, HashSet<String>>,
    ) -> bool {
        if let Some(neighbors) = graph.get(node) {
            clique
                .iter()
                .all(|clique_node| neighbors.contains(clique_node))
        } else {
            false
        }
    }

    // Recursive function to find maximum clique
    fn find_clique_recursive(
        candidates: &[String],
        current_clique: &mut HashSet<String>,
        max_clique: &mut HashSet<String>,
        graph: &HashMap<String, HashSet<String>>,
    ) {
        if candidates.is_empty() {
            if current_clique.len() > max_clique.len() {
                max_clique.clear();
                max_clique.extend(current_clique.iter().cloned());
            }
            return;
        }

        for (i, candidate) in candidates.iter().enumerate() {
            if current_clique.len() + (candidates.len() - i) <= max_clique.len() {
                // Early pruning: impossible to beat max_clique
                return;
            }

            if is_connected_to_all(candidate, current_clique, graph) {
                // Try including this node
                current_clique.insert(candidate.clone());

                // Create new candidates list excluding current node and non-neighbors
                let new_candidates: Vec<String> = candidates[(i + 1)..]
                    .iter()
                    .filter(|&node| is_connected_to_all(node, current_clique, graph))
                    .cloned()
                    .collect();

                find_clique_recursive(&new_candidates, current_clique, max_clique, graph);
                current_clique.remove(candidate);
            }
        }
    }

    find_clique_recursive(&nodes, &mut current_clique, &mut max_clique, graph);
    max_clique
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
    let results = graph
        .keys()
        .combinations(num)
        .filter(|r| {
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
        })
        .collect_vec();

    results
}
