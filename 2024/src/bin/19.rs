use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(19);

fn is_design_possible<'a>(
    design: &'a str,
    towels: &[&str],
    cache: &mut HashMap<&'a str, u64>,
) -> Option<u64> {
    if design.is_empty() {
        return Some(1);
    }
    if let Some(combinations) = cache.get(design) {
        return Some(*combinations);
    }

    towels
        .iter()
        .filter_map(|towel| design.strip_prefix(towel))
        .filter_map(|design| {
            let combinations = is_design_possible(design, towels, cache)?;
            cache.insert(design, combinations);
            Some(combinations)
        })
        .sum1()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, designs) = input.split_once("\n\n")?;
    let towels: Vec<&str> = towels.split(", ").collect();

    let mut cache = HashMap::new();
    let possible_designs = designs
        .lines()
        .filter(|design| is_design_possible(design, &towels, &mut cache).is_some())
        .count();
    Some(possible_designs as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels, designs) = input.split_once("\n\n")?;
    let towels: Vec<&str> = towels.split(", ").collect();

    let mut cache = HashMap::new();
    designs
        .lines()
        .filter_map(|design| is_design_possible(design, &towels, &mut cache))
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
