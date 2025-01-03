use std::ops::Neg;

use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != 'X' {
                continue;
            }

            for direction in (-1..=1).cartesian_product(-1..=1) {
                let (mut new_y, mut new_x) = (y as i32, x as i32);
                for expected_letter in ['M', 'A', 'S'] {
                    new_y += direction.0;
                    new_x += direction.1;
                    if new_y < 0
                        || new_y as usize >= grid.len()
                        || new_x < 0
                        || new_x as usize >= grid[y].len()
                        || grid[new_y as usize][new_x as usize] != expected_letter
                    {
                        break;
                    }
                    sum += (expected_letter == 'S') as u32;
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    (1..grid.len() - 1)
        .cartesian_product(1..grid[0].len() - 1)
        .filter(|&(y, x)| grid[y][x] == 'A')
        .map(|(y, x)| {
            let mut directions = (-1, -1);
            (0..4)
                .map(|_| {
                    // rotate directions by 90 degree
                    directions = (directions.1, directions.0.neg());
                    directions
                })
                .map(|directions| {
                    (
                        (y as i32 + directions.0, x as i32 + directions.1),
                        (y as i32 + directions.0.neg(), x as i32 + directions.1.neg()),
                    )
                })
                .map(|((y_1, x_1), (y_2, x_2))| {
                    (
                        grid[y_1 as usize][x_1 as usize],
                        grid[y_2 as usize][x_2 as usize],
                    )
                })
                .filter(|(a, b)| a == &'M' && b == &'S')
                .count()
        })
        .map(|count| (count == 2) as u32)
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
