use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = input.split_once("\n\n")?;
    let rules = rules
        .lines()
        .filter_map(|line| line.split_once("|"))
        .filter_map(|(first, second)| {
            Some((first.parse::<u32>().ok()?, second.parse::<u32>().ok()?))
        })
        .collect::<HashSet<_>>();

    updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect_vec()
        })
        .filter(|update| update.is_sorted_by(|&a, &b| rules.contains(&(a, b))))
        .inspect(|update| println!("{:?}", update))
        .map(|update| update[update.len() / 2])
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
