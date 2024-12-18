use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
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

fn find_path(falling_bytes: &[(i32, i32)], start: (i32, i32), goal: (i32, i32)) -> Option<u32> {
    let mut queue = BinaryHeap::new();
    queue.push(Node {
        cost: 0,
        position: start,
        len: 0,
    });
    let mut visited = HashSet::from([start]);
    let bounds = 0..=SIZE;

    while let Some(node) = queue.pop() {
        if node.position == goal {
            return Some(node.len);
        }

        let mut direction = (0, 1);
        for _ in 0..4 {
            direction = (direction.1, direction.0.neg());
            let position = (node.position.0 + direction.0, node.position.1 + direction.1);
            let candidate = Node {
                cost: node.cost + 1,
                position,
                len: node.len + 1,
            };
            if falling_bytes.contains(&position) {
                continue;
            }
            if bounds.contains(&position.0)
                && bounds.contains(&position.1)
                && visited.insert(position)
            {
                queue.push(candidate);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let falling_bytes = input
        .lines()
        .filter_map(|line| line.split_once(','))
        .filter_map(|(x, y)| Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?)))
        .take(FALLEN_BYTES)
        .collect_vec();
    find_path(&falling_bytes, (0, 0), (SIZE, SIZE))
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
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
