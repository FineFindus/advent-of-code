use itertools::Itertools;

advent_of_code::solution!(2);

fn is_safe(reports: &[u32]) -> bool {
    (reports.is_sorted() || reports.is_sorted_by(|a, b| b <= a))
        && reports.iter().tuple_windows().all(|(a, b)| {
            let diff = a.abs_diff(*b);
            (1..=3).contains(&diff)
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|v| v.parse::<u32>().ok())
                .collect_vec()
        })
        .filter(|reports| is_safe(reports))
        .count() as u32;
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|v| v.parse::<u32>().ok())
                .collect_vec()
        })
        .filter(|reports| {
            let mut reports = reports.clone();
            for index in 0..reports.len() {
                let problem_dampener = reports.remove(index);
                if is_safe(&reports) {
                    return true;
                }
                reports.insert(index, problem_dampener)
            }
            false
        })
        .count() as u32;
    Some(safe_reports)
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
        assert_eq!(result, Some(4));
    }
}
