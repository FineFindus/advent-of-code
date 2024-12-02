use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|v| v.parse::<u32>().ok())
                    .collect_vec()
            })
            .filter(|reports| reports.is_sorted() || reports.is_sorted_by(|a, b| b <= a))
            .filter(|reports| {
                reports.iter().tuple_windows().all(|(a, b)| {
                    let diff = a.abs_diff(*b);
                    (1..=3).contains(&diff)
                })
            })
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
