use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    ops::Neg,
};

use itertools::Itertools;

advent_of_code::solution!(12);

/// Returns the cell at `(y,x)`, if it exists.
fn cell<T>(grid: &[Vec<T>], y: i32, x: i32) -> Option<&T> {
    if !(0..grid.len() as i32).contains(&y) || !(0..grid[0].len() as i32).contains(&x) {
        return None;
    }
    grid[y as usize].get(x as usize)
}

fn calculate_plot_size(garden: &[Vec<char>], y: i32, x: i32) -> (HashSet<(i32, i32)>, usize) {
    let label = cell(garden, y, x).unwrap();
    let mut edges = HashSet::new();
    let mut area = HashSet::new();
    area.insert((y, x));

    let mut cells_to_check: VecDeque<(i32, i32)> = VecDeque::from([(y, x)]);
    while let Some(plot) = cells_to_check.pop_front() {
        let mut directions = (-1, 0);
        let neighbors = (0..4)
            .map(|_| {
                directions = (directions.1, directions.0.neg());
                directions
            })
            .map(|directions| (plot.0 + directions.0, plot.1 + directions.1))
            .map(|(y, x)| (y, x, cell(garden, y, x)))
            .filter(|&(y, x, _neighbor)| {
                !edges.contains(&((y, x), plot)) && !area.contains(&(y, x))
            })
            .collect_vec();

        for (ny, nx, neighbor) in neighbors {
            dbg!((ny, nx, neighbor));
            match neighbor {
                Some(char) if char == label => {
                    area.insert((ny, nx));
                    cells_to_check.push_back((ny, nx));
                }
                Some(_) | None => {
                    edges.insert(((ny, nx), plot));
                }
            }
        }
    }

    (area, edges.len())
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut checked_plots: HashSet<(i32, i32)> = HashSet::new();
    let garden = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut price = 0;
    for y in 0..garden.len() as i32 {
        for x in 0..garden[0].len() as i32 {
            if checked_plots.contains(&(y, x)) {
                continue;
            }
            let (plots, perimenter) = calculate_plot_size(&garden, y, x);
            checked_plots.extend(plots.iter());
            price += plots.len() * perimenter;
        }
    }
    Some(price as u32)
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
