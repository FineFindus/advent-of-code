use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut accumulator: u64 = 0;
    let masks = input
        .trim()
        .as_bytes()
        .iter()
        .map(|char| 1 << (*char as u32 - 'a' as u32))
        .collect_vec();
    for (index, mask) in masks.iter().enumerate() {
        accumulator ^= mask;
        if index >= 4 {
            accumulator ^= masks[index - 4];
            if accumulator.count_ones() == 4 {
                return Some(index as u32 + 1);
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut accumulator: u64 = 0;
    let masks = input
        .trim()
        .as_bytes()
        .iter()
        .map(|char| 1 << (*char as u32 - 'a' as u32))
        .collect_vec();
    for (index, mask) in masks.iter().enumerate() {
        accumulator ^= mask;
        if index >= 14 {
            accumulator ^= masks[index - 14];
            if accumulator.count_ones() == 14 {
                return Some(index as u32 + 1);
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
