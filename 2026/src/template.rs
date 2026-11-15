pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    const INPUT: &str = include_str!("../data/inputs/DAY.txt");

    advent_of_code::solve(part_one, INPUT, 1);
    advent_of_code::solve(part_two, INPUT, 2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../data/examples/DAY.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE), None);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE), None);
    }
}
