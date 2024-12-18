use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok())
                .collect_tuple::<(u32, u32)>()
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    Some(left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut similarity_score = HashMap::new();
    Some(
        input
            .lines()
            .filter_map(|line| {
                let (id, val) = line
                    .split_whitespace()
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect_tuple::<(u32, u32)>()?;
                let score: u32 = similarity_score.get(&val).copied().unwrap_or_default();
                similarity_score.insert(val, score + 1);
                Some(id)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|id| {
                let score = similarity_score.get(&id).unwrap_or(&0);
                id * score
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
