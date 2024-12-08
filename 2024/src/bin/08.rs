use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn parse_antenna_locations(input: &str) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antenna: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    input.lines().enumerate().for_each(|(line_index, line)| {
        line.chars()
            .enumerate()
            .filter(|&(_, char)| char != '.')
            .for_each(|(index, char)| {
                let location = (index as isize, line_index as isize);
                antenna
                    .entry(char)
                    .and_modify(|locs| locs.push(location))
                    .or_insert(vec![location]);
            })
    });
    antenna
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.find("\n")?;
    let height = input.split("\n").count();

    let mut antenna = parse_antenna_locations(input);
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_type, locations) in antenna.iter_mut() {
        while let Some((x, y)) = locations.pop() {
            for &(x_2, y_2) in locations.iter() {
                let dist = (y_2 - y, x_2 - x);
                let antinode_before = (y_2 + dist.0, x_2 + dist.1);
                antinodes.insert(antinode_before);
                let antinode_after = (y - dist.0, x - dist.1);
                antinodes.insert(antinode_after);
            }
        }
    }

    Some(
        antinodes
            .iter()
            .filter(|(y, x)| (0..width as isize).contains(y) && (0..height as isize).contains(x))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.find("\n")?;
    let height = input.split("\n").count();

    let antenna = parse_antenna_locations(input);
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    let is_in_bounds = |x, y| (0..width as isize).contains(&y) && (0..height as isize).contains(&x);

    for (_type, locations) in antenna {
        for &(x, y) in locations.iter() {
            for &(x_2, y_2) in locations.iter() {
                if (x, y) == (x_2, y_2) {
                    continue;
                }
                let dist = (x_2 - x, y_2 - y);
                let mut antinode = (x, y);
                while is_in_bounds(antinode.0, antinode.1) {
                    antinodes.insert(antinode);
                    antinode = (antinode.0 + dist.0, antinode.1 + dist.1);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
