use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Neg,
};

use itertools::Itertools;

advent_of_code::solution!(16);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    cost: u32,
    position: (i32, i32),
    direction: (i32, i32),
    path: HashSet<(i32, i32)>,
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

fn find_best_path(
    grid: &[Vec<char>],
    reindeer_position: (i32, i32),
    goal: (i32, i32),
) -> Option<u32> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Node {
        cost: 0,
        position: reindeer_position,
        direction: (0, 1),
        path: HashSet::new(),
    });
    visited.insert(reindeer_position);
    while let Some(node) = queue.pop() {
        if node.position == goal {
            return Some(node.cost);
        }

        let mut direction = node.direction;
        for i in 0..4 {
            let candidate = (node.position.0 + direction.0, node.position.1 + direction.1);
            // rotate next direction by 90 degree
            if grid[candidate.0 as usize][candidate.1 as usize] != '#' && visited.insert(candidate)
            {
                queue.push(Node {
                    cost: node.cost + 1 + 1000 * i.min(1),
                    position: candidate,
                    direction,
                    path: node.path.clone(),
                });
            }
            direction = (direction.1, direction.0.neg());
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let reindeer_position = grid
        .iter()
        .enumerate()
        .find_map(|(idx, row)| Some((idx as i32, row.iter().position(|char| char == &'S')? as i32)))
        .expect("`reindeer` should be in grid");

    let goal = grid
        .iter()
        .enumerate()
        .find_map(|(idx, row)| Some((idx as i32, row.iter().position(|char| char == &'E')? as i32)))
        .expect("`reindeer` should be in grid");
    find_best_path(&grid, reindeer_position, goal)
}

fn find_best_seats(
    grid: &[Vec<char>],
    position: (i32, i32),
    goal: (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut queue = BinaryHeap::from([Node {
        cost: 0,
        position,
        direction: (0, 1),
        path: HashSet::from([position]),
    }]);

    let mut best_cost = None;
    let mut best_seats: HashSet<(i32, i32)> = HashSet::new();
    let mut min_costs: HashMap<((i32, i32), (i32, i32)), u32> = HashMap::new();
    while let Some(node) = queue.pop() {
        match best_cost {
            Some(old_score) if old_score < node.cost => {
                continue;
            }
            _ => {
                if node.position == goal {
                    best_cost = Some(node.cost);
                    best_seats.extend(node.path.into_iter());
                    continue;
                }
            }
        };

        let min_cost = min_costs
            .entry((node.position, node.direction))
            .or_insert(node.cost);
        if node.cost > *min_cost {
            continue;
        }

        let mut direction = node.direction;
        for i in 0..4 {
            let candidate = (node.position.0 + direction.0, node.position.1 + direction.1);
            // rotate next direction by 90 degree
            if grid[candidate.0 as usize][candidate.1 as usize] != '#'
                && !node.path.contains(&candidate)
            {
                let mut candidate_node = Node {
                    cost: node.cost + 1 + 1000 * i.min(1),
                    position: candidate,
                    direction,
                    path: node.path.clone(),
                };
                candidate_node.path.insert(candidate_node.position);
                queue.push(candidate_node);
            }
            direction = (direction.1, direction.0.neg());
        }
    }
    best_seats
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let reindeer_position = grid
        .iter()
        .enumerate()
        .find_map(|(idx, row)| Some((idx as i32, row.iter().position(|char| char == &'S')? as i32)))
        .expect("`reindeer` should be in grid");

    let goal = grid
        .iter()
        .enumerate()
        .find_map(|(idx, row)| Some((idx as i32, row.iter().position(|char| char == &'E')? as i32)))
        .expect("`reindeer` should be in grid");
    let paths = find_best_seats(&grid, reindeer_position, goal);

    Some(paths.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
