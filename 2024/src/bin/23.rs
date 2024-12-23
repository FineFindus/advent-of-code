use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);
fn find_triangles<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut triangles = Vec::new();

    for (&computer, neighbors) in graph.iter() {
        for &linked_computer in neighbors {
            if linked_computer > computer {
                let common_neighbors: Vec<&&str> = neighbors
                    .intersection(&graph[&linked_computer])
                    .filter(|&&v| v > linked_computer) // Ensure lexicographical order
                    .collect();

                for neighbor in common_neighbors {
                    triangles.push(vec![computer, linked_computer, neighbor]);
                }
            }
        }
    }

    triangles
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = input.lines().filter_map(|line| line.split_once("-")).fold(
        HashMap::new(),
        |mut acc, (a, b)| {
            acc.entry(a)
                .and_modify(|connections: &mut HashSet<&str>| {
                    connections.insert(b);
                })
                .or_insert(HashSet::from([b]));
            acc.entry(b)
                .and_modify(|connections: &mut HashSet<&str>| {
                    connections.insert(a);
                })
                .or_insert(HashSet::from([a]));
            acc
        },
    );
    // find_cliques(&graph);
    let connected_computer = find_triangles(&graph)
        .iter()
        .filter(|triangle| triangle.iter().any(|v| v.starts_with("t")))
        .count();
    Some(connected_computer as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
