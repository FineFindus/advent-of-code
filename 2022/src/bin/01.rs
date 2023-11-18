use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|elf| {
            elf.split('\n')
                .filter_map(|calories| calories.parse::<u32>().ok())
                .sum1::<u32>()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|elf| {
            elf.split('\n')
                .filter_map(|calories| calories.parse::<u32>().ok())
                .sum1::<u32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum1()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
