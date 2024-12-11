use itertools::Itertools;

advent_of_code::solution!(11);

fn advance_stone(stone: u128, blinks: usize) -> u128 {
    if blinks == 0 {
        return 1;
    }

    let mut sum = 0;
    let blinked_engraving = match stone {
        0 => 1,
        n if digits(n) % 2 == 0 => {
            let divisor = 10u128.pow(digits(n) / 2);
            sum += advance_stone(n / divisor, blinks - 1);
            n % divisor
        }
        n => n * 2024,
    };

    advance_stone(blinked_engraving, blinks - 1) + sum
}

fn digits(n: u128) -> u32 {
    n.ilog10() + 1
}

pub fn part_one(input: &str) -> Option<u128> {
    input
        .split(" ")
        .map(|num| num.parse::<u128>().unwrap())
        .map(|stone| advance_stone(stone, 25))
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
