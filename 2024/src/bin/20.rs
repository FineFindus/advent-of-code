use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Neg,
};

use itertools::Itertools;

advent_of_code::solution!(20);

/// Returns the cell at `(y,x)`, if it exists.
fn cell<T>(grid: &[Vec<T>], y: i32, x: i32) -> Option<&T> {
    if !(0..grid.len() as i32).contains(&y) || !(0..grid[0].len() as i32).contains(&x) {
        return None;
    }
    grid[y as usize].get(x as usize)
}

struct Cheat {
    time_saved: u32,
    start: (i32, i32),
    end: (i32, i32),
}

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

fn find_path(grid: &[Vec<char>], start: (i32, i32), goal: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    let mut queue = BinaryHeap::new();
    queue.push(Node {
        cost: 0,
        position: start,
        len: 0,
    });
    let mut visited = HashSet::from([start]);
    let mut parents = HashMap::new();

    while let Some(node) = queue.pop() {
        if node.position == goal {
            //TODO: does the goal count as part of the path?
            let mut path = vec![goal];

            let mut current = goal;
            while let Some(&parent) = parents.get(&current) {
                path.push(parent);
                current = parent;
            }
            path.reverse();

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

            //TODO: remove bound checks
            if cell(grid, position.0, position.1).is_some_and(|cell| cell == &'#')
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
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = grid.iter().enumerate().find_map(|(idx, row)| {
        Some((idx as i32, row.iter().position(|char| char == &'S')? as i32))
    })?;
    let goal = grid.iter().enumerate().find_map(|(idx, row)| {
        Some((idx as i32, row.iter().position(|char| char == &'E')? as i32))
    })?;

    let path = find_path(&grid, start, goal)?;
    path.iter()
        .enumerate()
        .tuple_combinations()
        .map(|((index_a, a), (index_b, b))| {
            let distance = (a.0 - b.0).unsigned_abs() + (a.1.saturating_sub(b.1)).unsigned_abs();
            (distance <= 2 && index_a.abs_diff(index_b) > 100) as u32
        })
        .sum1()
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
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
