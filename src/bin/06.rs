use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let bytes = input.as_bytes();

    let mut items: Vec<u8> = Vec::with_capacity(5);
    for (index, byte) in bytes.iter().enumerate() {
        items.push(*byte);

        if index.checked_sub(4).is_some() {
            items.remove(0);
        }

        if items.len() == 4 &&
            items.iter().unique().collect_vec().len() == 4 {
                return Some(index as u32 + 1);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let bytes = input.as_bytes();

    let mut items: Vec<u8> = Vec::with_capacity(5);
    for (index, byte) in bytes.iter().enumerate() {
        items.push(*byte);

        if index.checked_sub(14).is_some() {
            items.remove(0);
        }

        if items.len() == 14 &&
            items.iter().unique().collect_vec().len() == 14 {
                return Some(index as u32 + 1);
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
