use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Neg,
};

use itertools::Itertools;

advent_of_code::solution!(18);

#[cfg(test)]
const SIZE: i32 = 6;
#[cfg(not(test))]
const SIZE: i32 = 70;
#[cfg(test)]
const FALLEN_BYTES: usize = 12;
#[cfg(not(test))]
const FALLEN_BYTES: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    cost: u32,
    position: (i32, i32),
    len: u32,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(
    falling_bytes: &HashSet<(i32, i32)>,
    start: (i32, i32),
    goal: (i32, i32),
) -> Option<HashSet<(i32, i32)>> {
    let mut queue = BinaryHeap::new();
    queue.push(Node {
        cost: 0,
        position: start,
        len: 0,
    });
    let mut visited = HashSet::from([start]);
    let mut parents = HashMap::new();
    let bounds = 0..=SIZE;

    while let Some(node) = queue.pop() {
        if node.position == goal {
            let mut path = HashSet::from([goal]);

            let mut current = goal;
            while let Some(&parent) = parents.get(&current) {
                path.insert(parent);
                current = parent;
            }

            return Some(path);
        }

        let mut direction = (0, 1);
        for _ in 0..4 {
            direction = (direction.1, direction.0.neg());
            let position = (node.position.0 + direction.0, node.position.1 + direction.1);
            let candidate = Node {
                cost: (position.0 - goal.0).unsigned_abs()
                    + (position.1.saturating_sub(goal.1)).unsigned_abs(),
                position,
                len: node.len + 1,
            };

            if falling_bytes.contains(&position)
                || !bounds.contains(&position.0)
                || !bounds.contains(&position.1)
                || !visited.insert(position)
            {
                continue;
            }

            queue.push(candidate);
            parents.insert(position, node.position);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let falling_bytes: HashSet<(i32, i32)> = input
        .lines()
        .filter_map(|line| line.split_once(','))
        .filter_map(|(x, y)| Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?)))
        .take(FALLEN_BYTES)
        .collect();
    find_path(&falling_bytes, (0, 0), (SIZE, SIZE)).map(|path| path.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<String> {
    let falling_bytes = input
        .lines()
        .filter_map(|line| line.split_once(','))
        .filter_map(|(x, y)| Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?)))
        .collect_vec();
    let mut fallen_bytes: HashSet<(i32, i32)> =
        HashSet::from_iter(falling_bytes.iter().take(FALLEN_BYTES).copied());

    let mut path = find_path(&fallen_bytes, (0, 0), (SIZE, SIZE));
    for falling_byte in falling_bytes.into_iter().skip(FALLEN_BYTES) {
        fallen_bytes.insert(falling_byte);
        if !path
            .as_ref()
            .is_some_and(|path| path.contains(&falling_byte))
        {
            continue;
        }

        // a byte has fallen on our path, calculate a new path
        path = find_path(&fallen_bytes, (0, 0), (SIZE, SIZE));
        if path.is_none() {
            return Some(format!("{},{}", falling_byte.0, falling_byte.1));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
