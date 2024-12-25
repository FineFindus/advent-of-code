use itertools::Itertools;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let schematics: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .map(|schematic| {
            schematic
                .lines()
                .map(|line| line.chars().collect())
                .collect()
        })
        .collect();
    let (locks, keys): (Vec<_>, Vec<_>) = schematics.iter().partition_map(|schematic| {
        let mut columns = [0; 5];
        for y in 1..schematic.len() - 1 {
            for x in 0..schematic[0].len() {
                columns[x] += (schematic[y][x] == '#') as u8;
            }
        }
        if schematic[0].iter().all(|v| v == &'#')
            && schematic.last().unwrap().iter().all(|v| v == &'.')
        {
            itertools::Either::Left(columns)
        } else {
            itertools::Either::Right(columns)
        }
    });
    let mut fits = 0;
    for lock in locks {
        for key in &keys {
            if lock.iter().zip(key).map(|(a, b)| a + b).all(|v| v <= 5) {
                fits += 1;
            }
        }
    }
    Some(fits)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
