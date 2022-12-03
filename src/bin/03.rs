use itertools::Itertools;

fn alphabet_index_of(char: u8) -> u8 {
    if char.is_ascii_uppercase() {
        char - 38
    } else {
        char - 96
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .filter_map(|(first, second)| {
            let mut alphabet: u64 = 0;
            for char in first.bytes() {
                alphabet |= 1 << alphabet_index_of(char);
            }
            let mut item = None;
            for char in second.bytes() {
                if (alphabet >> alphabet_index_of(char)) & 1 == 1 {
                    item = Some(char);
                    break;
                }
            }
            item
        })
        .map(|char| alphabet_index_of(char) as u32)
        .sum1::<u32>()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .filter_map(|mut lines| {
            let mut alphabet: u64 = 0;
            for char in lines.next().unwrap().bytes() {
                alphabet |= 1 << alphabet_index_of(char);
            }
            let mut second: u64 = 0;
            for char in lines.next().unwrap().bytes() {
                second |= 1 << alphabet_index_of(char);
            }
            alphabet &= second;
            let mut item = None;
            for char in lines.next().unwrap().bytes() {
                if (alphabet >> alphabet_index_of(char)) & 1 == 1 {
                    item = Some(char);
                    break;
                }
            }
            item
        })
        .map(|char| alphabet_index_of(char) as u32)
        .sum1::<u32>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
