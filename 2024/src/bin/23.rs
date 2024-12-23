use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

fn bron_kerbosch2<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    cliques: &mut Vec<HashSet<&'a str>>,
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }
    // choose a pivot vertex u in P ⋃ X
    let pivot = p.union(x).copied().next().unwrap();
    let p_without_neighbors: HashSet<_> = p.difference(&graph[&pivot]).copied().collect();
    // for each vertex v in P \ N(u) do
    for v in p_without_neighbors {
        let neighbors = &graph[v];
        // BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
        let mut p_next = p.intersection(neighbors).copied().collect();
        let mut x_next = x.intersection(neighbors).copied().collect();
        r.insert(v);
        bron_kerbosch2(graph, cliques, r, &mut p_next, &mut x_next);
        r.remove(v);
        // P := P \ {v}
        p.remove(v);
        // X := X ⋃ {v}
        x.insert(v);
    }
}

fn parse_graph(input: &str) -> HashMap<&str, HashSet<&str>> {
    input
        .lines()
        .filter_map(|line| line.split_once("-"))
        .fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(a).or_default().insert(b);
            acc.entry(b).or_default().insert(a);
            acc
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = parse_graph(input);
    let mut triangles = Vec::new();

    for (&computer, neighbors) in graph.iter() {
        for &linked_computer in neighbors {
            if linked_computer <= computer {
                continue;
            }
            let triagnles_neighbors = neighbors
                .intersection(&graph[&linked_computer])
                .filter(|&&v| v > linked_computer) // Ensure lexicographical order
                .map(|neighbor| vec![computer, linked_computer, neighbor]);
            triangles.extend(triagnles_neighbors);
        }
    }

    let connected_computer = triangles
        .iter()
        .filter(|triangle| triangle.iter().any(|v| v.starts_with("t")))
        .count();
    Some(connected_computer as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let graph = parse_graph(input);

    let mut cliques = Vec::new();
    let mut p: HashSet<&str> = graph.keys().copied().collect();
    let mut r = HashSet::new();
    let mut x = HashSet::new();
    bron_kerbosch2(&graph, &mut cliques, &mut r, &mut p, &mut x);
    cliques
        .iter()
        .max_by_key(|v| v.len())
        .map(|max| max.iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
