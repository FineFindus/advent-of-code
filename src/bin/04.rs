use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                line.split(',')
                    .map(|sections| sections.split('-').collect_vec())
                    .flatten()
                    .filter_map(|section| section.parse::<u32>().ok())
                    .collect_tuple::<(u32, u32, u32, u32)>()
            })
            .filter(|sections| {
                (sections.0 <= sections.2 && sections.1 >= sections.3)
                    || (sections.0 >= sections.2 && sections.1 <= sections.3)
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                line.split(',')
                    .map(|sections| sections.split('-').collect_vec())
                    .flatten()
                    .filter_map(|section| section.parse::<u32>().ok())
                    .collect_tuple::<(u32, u32, u32, u32)>()
            })
            .filter(|sections| (sections.1 >= sections.2 && !(sections.0 > sections.3)))
            .inspect(|item| println!("{:?}", item))
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
