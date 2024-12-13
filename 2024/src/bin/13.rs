use itertools::Itertools;

advent_of_code::solution!(13);

fn calculate_price_combination(
    (a_x, a_y): (i64, i64),
    (b_x, b_y): (i64, i64),
    (p_x, p_y): (i64, i64),
) -> Option<i64> {
    // solve using Cramer's rule
    // https://en.wikipedia.org/wiki/Cramer%27s_rule?useskin=vector
    let det = a_x * b_y - a_y * b_x;
    let button_a = (p_x * b_y - p_y * b_x) / det;
    let button_b = (a_x * p_y - a_y * p_x) / det;

    // check if result is works
    if (
        a_x * button_a + b_x * button_b,
        a_y * button_a + b_y * button_b,
    ) == (p_x, p_y)
    {
        Some(button_a * 3 + button_b)
    } else {
        None
    }
}


pub fn part_one(input: &str) -> Option<i64> {
    //TODO: maybe not use regex to extract the numbers
    regex::Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .expect("`regex` should be valid")
    .captures_iter(input)
    .map(|c| c.extract())
    .map(|(_, values)| values.map(|v| v.parse::<i64>().unwrap()))
    .filter_map(|[a_x, a_y, b_x, b_y, p_x, p_y]| {
        calculate_price_combination((a_x, a_y), (b_x, b_y), (p_x, p_y))
    })
    .sum1()
}

pub fn part_two(input: &str) -> Option<i64> {
    const ERROR: i64 = 10_000_000_000_000;
    regex::Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .expect("`regex` should be valid")
    .captures_iter(input)
    .map(|c| c.extract())
    .map(|(_, values)| values.map(|v| v.parse::<i64>().unwrap()))
    .filter_map(|[a_x, a_y, b_x, b_y, p_x, p_y]| {
        calculate_price_combination((a_x, a_y), (b_x, b_y), (ERROR + p_x, ERROR + p_y))
    })
    .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
