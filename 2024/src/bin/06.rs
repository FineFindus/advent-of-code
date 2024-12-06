use std::ops::Neg;

use itertools::Itertools;

advent_of_code::solution!(6);

fn cell(mapped_area: &[Vec<char>], y: i32, x: i32) -> Option<&char> {
    if !(0..mapped_area.len() as i32).contains(&y) || !(0..mapped_area[0].len() as i32).contains(&x)
    {
        return None;
    }
    mapped_area[y as usize].get(x as usize)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut mapped_area = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut direction = (-1, 0);
    let mut guard_pos = mapped_area.iter().enumerate().find_map(|(idx, row)| {
        Some((idx as i32, row.iter().position(|char| char == &'^')? as i32))
    })?;

    loop {
        let next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        mapped_area[guard_pos.0 as usize][guard_pos.1 as usize] = 'X';
        match cell(&mapped_area, next_pos.0, next_pos.1) {
            //rotate 90 deg
            Some('#') => direction = (direction.1, direction.0.neg()),
            // walk
            Some(_) => {
                guard_pos = next_pos;
            }
            // out-of-bounds
            None => break,
        };
    }

    mapped_area
        .iter()
        .map(|row| row.iter().filter(|char| char == &&'X').count() as u32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
