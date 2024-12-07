use itertools::Itertools;

advent_of_code::solution!(7);

fn is_result_possible<const CONCAT: bool>(
    result: u64,
    intermediate_result: u64,
    values: &[u64],
) -> bool {
    if values.is_empty() || intermediate_result > result {
        return intermediate_result == result;
    }

    let v = values.first().unwrap();
    is_result_possible::<CONCAT>(result, intermediate_result + v, &values[1..])
        || is_result_possible::<CONCAT>(result, intermediate_result * v, &values[1..])
        || (CONCAT
            && is_result_possible::<true>(
                result,
                concat_nums(intermediate_result, *v),
                &values[1..],
            ))
}

// https://stackoverflow.com/questions/12700497/how-to-concatenate-two-integers-in-c/12700533#12700533
const fn concat_nums(a: u64, b: u64) -> u64 {
    let mut pow = 10;
    while b >= pow {
        pow *= 10;
    }
    a * pow + b
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
        .filter(|(result, values)| is_result_possible::<false>(*result, values[0], &values[1..]))
        .map(|(result, _values)| result)
        .sum1()
}

pub fn part_two(input: &str) -> Option<u64> {
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
        .filter(|(result, values)| is_result_possible::<true>(*result, values[0], &values[1..]))
        .map(|(result, _values)| result)
        .sum1()
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
        assert_eq!(result, Some(11387));
    }
}
