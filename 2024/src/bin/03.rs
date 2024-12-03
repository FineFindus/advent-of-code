use itertools::Itertools;

advent_of_code::solution!(3);

fn execute_mul_instructions(instructions: &str) -> Option<u32> {
    regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .expect("`regex` should be valid")
        .captures_iter(instructions)
        .map(|c| c.extract())
        .map(|(_, [x, y])| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .map(|(x, y)| x * y)
        .sum1()
}

pub fn part_one(input: &str) -> Option<u32> {
    execute_mul_instructions(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .split("do()")
        .filter_map(|instructions| instructions.split("don't()").next())
        .filter_map(execute_mul_instructions)
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
