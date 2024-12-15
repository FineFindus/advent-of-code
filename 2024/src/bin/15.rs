use itertools::Itertools;

advent_of_code::solution!(15);

/// Returns the cell at `(y,x)`, if it exists.
fn cell<T>(grid: &[Vec<T>], y: i32, x: i32) -> Option<&T> {
    if !(0..grid.len() as i32).contains(&y) || !(0..grid[0].len() as i32).contains(&x) {
        return None;
    }
    grid[y as usize].get(x as usize)
}

fn move_entity(
    grid: &mut [Vec<char>],
    entity: char,
    position: (i32, i32),
    direction: (i32, i32),
) -> Option<(i32, i32)> {
    let target_pos = (position.0 + direction.0, position.1 + direction.1);
    match cell(grid, target_pos.0, target_pos.1) {
        Some(&'#') => None,
        Some(&'.') => {
            grid[position.0 as usize][position.1 as usize] = '.';
            grid[target_pos.0 as usize][target_pos.1 as usize] = entity;
            Some(target_pos)
        }
        Some(&'O') => {
            if move_entity(grid, 'O', target_pos, direction).is_some() {
                grid[position.0 as usize][position.1 as usize] = '.';
                grid[target_pos.0 as usize][target_pos.1 as usize] = entity;
                Some(target_pos)
            } else {
                None
            }
        }
        Some(&'[') => {
            if move_entity(grid, ']', (target_pos.0, target_pos.1 + 1), direction).is_some()
                && move_entity(grid, '[', target_pos, direction).is_some()
            {
                grid[position.0 as usize][position.1 as usize] = '.';
                if entity == '[' {
                    grid[position.0 as usize][(position.1 + 1) as usize] = '.';
                }
                grid[target_pos.0 as usize][target_pos.1 as usize] = entity;
                Some(target_pos)
            } else {
                None
            }
        }
        Some(&']') => {
            if move_entity(grid, '[', (target_pos.0, target_pos.1 - 1), direction).is_some()
                && move_entity(grid, ']', target_pos, direction).is_some()
            {
                grid[position.0 as usize][position.1 as usize] = '.';
                if entity == ']' {
                    grid[position.0 as usize][(position.1 - 1) as usize] = '.';
                }
                grid[target_pos.0 as usize][target_pos.1 as usize] = entity;
                Some(target_pos)
            } else {
                None
            }
        }
        Some(_) | None => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, moves) = input.split_once("\n\n")?;
    let mut grid = grid
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let moves = moves.lines().flat_map(|line| line.chars()).collect_vec();

    let mut robot = grid
        .iter()
        .enumerate()
        .find_map(|(idx, row)| Some((idx as i32, row.iter().position(|char| char == &'@')? as i32)))
        .expect("robot should be in grid");

    for move_type in moves {
        let direction = match move_type {
            '<' => (0, -1),
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            v => unreachable!("Unkown move instruction {v}"),
        };
        if let Some(position) = move_entity(&mut grid, '@', robot, direction) {
            robot = position;
        }
    }

    let mut gps_sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 'O' {
                continue;
            }
            gps_sum += 100 * y + x;
        }
    }
    Some(gps_sum as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    let (grid, moves) = input.split_once("\n\n")?;
    let mut grid = grid
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|char| match char {
                    '#' => ['#', '#'],
                    '.' => ['.', '.'],
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    _ => unreachable!("Unexpected {char}"),
                })
                .collect_vec()
        })
        .collect_vec();

    let moves = moves.lines().flat_map(|line| line.chars()).collect_vec();

    let mut robot = grid
        .iter()
        .enumerate()
        .find_map(|(idx, row)| Some((idx as i32, row.iter().position(|char| char == &'@')? as i32)))
        .expect("robot should be in grid");

    for move_type in moves {
        let reset_grid = grid.clone();
        let direction = match move_type {
            '<' => (0, -1),
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            v => unreachable!("Unkown move instruction {v}"),
        };
        if let Some(position) = move_entity(&mut grid, '@', robot, direction) {
            robot = position;
        } else {
            grid = reset_grid;
        }
    }

    let mut gps_sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '[' {
                continue;
            }
            gps_sum += 100 * y + x;
        }
    }
    Some(gps_sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
