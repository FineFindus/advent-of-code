use std::{collections::HashSet, ops::Neg};

use itertools::Itertools;

advent_of_code::solution!(10);

/// Returns the cell at `(y,x)`, if it exists.
fn cell(grid: &[Vec<u32>], y: i32, x: i32) -> Option<&u32> {
    if !(0..grid.len() as i32).contains(&y) || !(0..grid[0].len() as i32).contains(&x) {
        return None;
    }
    grid[y as usize].get(x as usize)
}

fn calculate_trail(
    grid: &[Vec<u32>],
    scores: &mut HashSet<(i32, i32)>,
    y: i32,
    x: i32,
) -> Option<u32> {
    // get current cell
    let trail_head = cell(grid, y, x)?;
    if trail_head == &9 {
        scores.insert((y, x));
        return Some(1);
    }

    // check all four directions for higher points
    let mut directions = (-1, 0);
    (0..4)
        .map(|_| {
            directions = (directions.1, directions.0.neg());
            directions
        })
        .map(|directions| (y + directions.0, x + directions.1))
        .filter(|&(y, x)| {
            cell(grid, y, x).is_some_and(|cell| {
                cell.abs_diff(*trail_head) == 1 && cell > trail_head && cell < &10
            })
        })
        .filter_map(|(y, x)| calculate_trail(grid, scores, y, x))
        .sum1()
}

fn rate_trail<F>(input: &str, f: F) -> Option<u32>
where
    F: Fn(&[Vec<u32>], usize, usize) -> u32,
{
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap_or(10))
                .collect_vec()
        })
        .collect_vec();

    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_x, &trail)| trail == 0)
                .map(move |(x, _trail)| (x, y))
        })
        .map(|(x, y)| f(&grid, x, y))
        .sum1()
}

pub fn part_one(input: &str) -> Option<u32> {
    rate_trail(input, |grid, x, y| {
        let mut scores = HashSet::new();
        calculate_trail(grid, &mut scores, y as i32, x as i32);
        scores.len() as u32
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    rate_trail(input, |grid, x, y| {
        let mut scores = HashSet::new();
        calculate_trail(grid, &mut scores, y as i32, x as i32).unwrap_or(0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
