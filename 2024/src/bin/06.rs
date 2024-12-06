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

struct Guard<'a> {
    grid: &'a mut [Vec<char>],
    position: (i32, i32),
    direction: (i32, i32),
}

impl<'a> Guard<'a> {
    pub fn new(grid: &'a mut [Vec<char>]) -> Self {
        let position = grid
            .iter()
            .enumerate()
            .find_map(|(idx, row)| {
                Some((idx as i32, row.iter().position(|char| char == &'^')? as i32))
            })
            .expect("guard should be in grid");

        Self {
            grid,
            position,
            direction: (-1, 0),
        }
    }

    pub fn reset(&mut self, positon: (i32, i32), direction: (i32, i32)) {
        self.position = positon;
        self.direction = direction;
    }

    pub fn next_step(&self) -> Option<(i32, i32)> {
        let next = (
            self.position.0 + self.direction.0,
            self.position.1 + self.direction.1,
        );
        if !(0..self.grid.len() as i32).contains(&next.0)
            || !(0..self.grid[0].len() as i32).contains(&next.1)
        {
            return None;
        }
        Some(next)
    }

    pub fn step(&mut self) -> Option<(i32, i32)> {
        let next_pos = self.next_step()?;
        // mark path
        self.grid[self.position.0 as usize][self.position.1 as usize] = 'X';
        // step to next cell
        match self.grid[next_pos.0 as usize][next_pos.1 as usize] {
            //rotate 90 deg
            '#' => {
                self.direction = (self.direction.1, self.direction.0.neg());
                return self.step();
            }
            // walk
            _ => {
                self.position = next_pos;
            }
        };
        Some(self.position)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut mapped_area = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut guard = Guard::new(&mut mapped_area);

    let mut obstruction_counter = 0;

    for y in 0..guard.grid.len() {
        for x in 0..guard.grid[0].len() {
            let guard_pos = guard.position;
            let direction = guard.direction;
            if guard.grid[y][x] == '#' {
                continue;
            }
            guard.grid[y][x] = '#';
            let mut steps = 0;
            while guard.step().is_some() {
                if steps >= 25_000 {
                    obstruction_counter += 1;
                    break;
                }
                steps += 1;
            }
            guard.reset(guard_pos, direction);
            guard.grid[y][x] = '.';
        }
    }

    Some(obstruction_counter)
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
        assert_eq!(result, Some(6));
    }
}
