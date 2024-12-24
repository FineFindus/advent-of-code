use itertools::Itertools;

advent_of_code::solution!(22);

fn generate_secrets(mut secret: u64, n: usize) -> Vec<u64> {
    let mut secrets = Vec::with_capacity(n);
    for _ in 0..n {
        secrets.push(secret);
        // 1. multiply
        secret ^= secret * 64;
        secret %= 16777216;
        // 2. divide
        secret ^= secret / 32;
        secret %= 16777216;
        // 3. multiply
        secret ^= secret * 2048;
        secret %= 16777216;
    }
    secrets
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter_map(|secret| generate_secrets(secret, 2000).last().copied())
        .sum1()
}

#[derive(Debug, Clone)]
struct Vendor {
    prices: Vec<u64>,
    prices_changes: Vec<i8>,
}

impl Vendor {
    pub fn new(secret: u64) -> Self {
        let secrets = generate_secrets(secret, 2001);
        let prices = secrets.iter().map(|secret| secret % 10).collect_vec();
        let price_diffs = prices
            .iter()
            .tuple_windows()
            .map(|(&a, &b)| b as i8 - a as i8)
            .collect_vec();
        Self {
            prices,
            prices_changes: price_diffs,
        }
    }

    pub fn possible_sequences(&self) -> Vec<[i8; 4]> {
        self.prices_changes
            .windows(4)
            .map(|window| window.try_into().unwrap())
            .collect_vec()
    }

    pub fn price_for_sequence(&self, sequence: &[i8; 4]) -> Option<u64> {
        let changes = self
            .prices_changes
            .windows(sequence.len())
            .position(|window| window == sequence)?;
        Some(self.prices[changes + sequence.len()])
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let vendors = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(Vendor::new)
        .collect_vec();
    let vendor_sequences = vendors
        .iter()
        .flat_map(|vendor| vendor.possible_sequences())
        .unique();

    vendor_sequences
        .map(|sequence| {
            let total_bananas: u64 = vendors
                .iter()
                .filter_map(|vendor| vendor.price_for_sequence(&sequence))
                .sum();
            total_bananas
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // this is just to ensure that the test still passes, as part two uses different data
        // assert_eq!(result, Some(37327623));
        assert_eq!(result, Some(36492164));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
