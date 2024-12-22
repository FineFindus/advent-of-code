use itertools::Itertools;

advent_of_code::solution!(22);

fn generate_secrets(mut secret: u64, n: usize) -> u64 {
    for _ in 0..n {
        // 1. multiply
        //TODO: this is a 2^n number...
        secret ^= secret * 64;
        secret %= 16777216;
        // 2. divide
        secret ^= secret / 32;
        secret %= 16777216;

        // 3. multiply, mix, prune
        secret ^= secret * 2048;
        secret %= 16777216;
    }
    secret
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|secret| generate_secrets(secret, 2000))
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
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
