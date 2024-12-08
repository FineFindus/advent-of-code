use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.find("\n")?;
    let height = input.split("\n").count();

    let mut antenna: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    input.lines().enumerate().for_each(|(line_index, line)| {
        line.chars()
            .enumerate()
            .filter(|&(_, char)| char != '.')
            .for_each(|(index, char)| {
                let location = (line_index as isize, index as isize);
                antenna
                    .entry(char)
                    .and_modify(|locs| locs.push(location))
                    .or_insert(vec![location]);
            })
    });
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_type, locations) in antenna.iter_mut() {
        while let Some((y, x)) = locations.pop() {
            for &(y_2, x_2) in locations.iter() {
                let dist = (y_2 - y, x_2 - x);
                let antinode_before = (y_2 + dist.0, x_2 + dist.1);
                let antinode_after = (y - dist.0, x - dist.1);
                antinodes.insert(antinode_before);
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
    None
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
        assert_eq!(result, None);
    }
}
