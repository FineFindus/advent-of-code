use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self, height: usize, width: usize) -> Vec<Self> {
        let mut neighbors = Vec::with_capacity(4);
        if self.y > 0 {
            neighbors.push(Self::new(self.x, self.y - 1));
        }
        if self.y < width - 1 {
            neighbors.push(Self::new(self.x, self.y + 1));
        }
        if self.x > 0 {
            neighbors.push(Self::new(self.x - 1, self.y));
        }
        if self.x < height - 1 {
            neighbors.push(Self::new(self.x + 1, self.y));
        }
        neighbors
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    cost: u32,
    coord: Coord,
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

pub fn dijkstra<F1, F2>(
    map: &Vec<Vec<u8>>,
    source: Coord,
    target: Coord,
    check_return: F1,
    check_height: F2,
) -> Option<u32>
where
    F1: Fn(&Coord, &Coord) -> bool,
    F2: Fn(u8, u8) -> bool,
{
    let height = map.len();
    let width = map[0].len();

    let mut priority_queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    priority_queue.push(Node {
        cost: 0,
        coord: source,
    });

    visited.insert(source);

    while let Some(Node { cost, coord }) = priority_queue.pop() {
        if check_return(&coord, &target) {
            return Some(cost);
        }

        let current_height = map[coord.x][coord.y];
        let neighbors = coord
            .neighbors(height, width)
            .into_iter()
            .filter(|&local_height| {
                let local_height = map[local_height.x][local_height.y];
                check_height(local_height, current_height)
            })
            .collect_vec();

        for candidate in neighbors {
            if visited.insert(candidate) {
                priority_queue.push(Node {
                    cost: cost + 1,
                    coord: candidate,
                })
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut source = Coord::new(0, 0);
    let mut target = Coord::new(0, 0);
    let mut map = vec![vec![0; input.lines().next().unwrap().len()]; input.lines().count()];

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let height = match char {
                'S' => {
                    source.x = row;
                    source.y = col;
                    'a'
                }
                'E' => {
                    target.x = row;
                    target.y = col;
                    'z'
                }
                _ => char,
            };
            map[row][col] = height as u8 - b'a';
        }
    }
    dijkstra(
        &map,
        source,
        target,
        |&coord, &target| coord == target,
        |local_height, current_height| {
            local_height <= current_height || local_height == current_height + 1
        },
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut source = Coord::new(0, 0);
    let mut target = Coord::new(0, 0);
    let mut map = vec![vec![0; input.lines().next().unwrap().len()]; input.lines().count()];

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let height = match char {
                'S' => {
                    source.x = row;
                    source.y = col;
                    'a'
                }
                'E' => {
                    target.x = row;
                    target.y = col;
                    'z'
                }
                _ => char,
            };
            map[row][col] = height as u8 - b'a';
        }
    }
    dijkstra(
        &map,
        target,
        target,
        |&coord, _| map[coord.x][coord.y] == 0,
        |local_height, current_height| {
            local_height >= current_height || local_height == current_height - 1
        },
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
