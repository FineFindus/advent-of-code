use itertools::Itertools;

advent_of_code::solution!(7);

fn is_result_possible(result: u64, intermediate_result: u64, values: &[u64]) -> bool {
    let Some(v) = values.first() else {
        return intermediate_result == result;
    };
    is_result_possible(result, intermediate_result + v, &values[1..])
        || is_result_possible(result, intermediate_result * v, &values[1..])
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .filter_map(|line| line.split_once(":"))
        .filter_map(|(result, values)| {
            Some((
                result.parse::<u64>().ok()?,
                values
                    .split_whitespace()
                    .map(|val| val.parse::<u64>().unwrap())
                    .collect_vec(),
            ))
        })
        .filter(|(result, values)| is_result_possible(*result, values[0], &values[1..]))
        .map(|(result, _values)| result)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
