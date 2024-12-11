use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

fn advance_stone(stone: u64, cache: &mut HashMap<(u64, usize), u64>, blinks: usize) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(blinked_stones) = cache.get(&(stone, blinks)) {
        return *blinked_stones;
    }

    let mut sum = 0;
    let blinked_engraving = match stone {
        0 => 1,
        n if digits(n) % 2 == 0 => {
            let divisor = 10u64.pow(digits(n) / 2);
            sum += advance_stone(n / divisor, cache, blinks - 1);
            n % divisor
        }
        n => n * 2024,
    };

    sum += advance_stone(blinked_engraving, cache, blinks - 1);
    cache.insert((stone, blinks), sum);
    sum
}

fn digits(n: u64) -> u32 {
    n.ilog10() + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut cache = HashMap::new();
    input
        .split(" ")
        .map(|num| num.parse::<u64>().unwrap())
        .map(|stone| advance_stone(stone, &mut cache, 25))
        .sum1()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cache = HashMap::new();
    input
        .split(" ")
        .map(|num| num.parse::<u64>().unwrap())
        .map(|stone| advance_stone(stone, &mut cache, 75))
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
