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

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != 'A' {
                continue;
            }

            let elemns = (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|(a, b)| *a != 0 && *b != 0)
                .map(|(a, b)| (y as i32 + a, x as i32 + b))
                .filter(|(new_x, new_y)| {
                    (0..grid.len() as i32).contains(new_y)
                        && (0..grid[y].len() as i32).contains(new_x)
                })
                .map(|(y, x)| grid[y as usize][x as usize])
                .counts();

            if elemns.get(&'M') == Some(&2)
                && elemns.get(&'S') == Some(&2)
            // extremly hacky solution
                && grid[y - 1][x - 1] != grid[y + 1][x + 1]
            {
                sum += 1;
            }
        }
    }
    Some(sum)
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
