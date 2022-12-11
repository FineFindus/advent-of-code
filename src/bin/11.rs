use core::str::FromStr;
use std::{collections::VecDeque, num::ParseIntError};

use itertools::Itertools;
#[derive(Debug)]
enum Operation {
    Square,
    Mulitply(u64),
    Add(u64),
}
#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    total_inspections: u64,
    operation: Operation,
    divisor: u64,
    true_monkey: u64,
    false_monkey: u64,
}

impl Monkey {
    pub fn new(
        items: VecDeque<u64>,
        operation: Operation,
        test: u64,
        true_monkey: u64,
        false_monkey: u64,
    ) -> Self {
        Self {
            items,
            total_inspections: 0,
            operation,
            divisor: test,
            true_monkey,
            false_monkey,
        }
    }
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .skip(1)
            .map(|line| line.trim())
            .map(|line| line.split_once(':').unwrap().1.trim())
            .collect_vec();
        let items = lines[0]
            .trim()
            .split(',')
            .filter_map(|item| item.trim().parse::<u64>().ok())
            .collect_vec();
        let test = lines[2]
            .strip_prefix("divisible by ")
            .unwrap()
            .parse::<u64>()?;
        let true_monkey = lines[3]
            .strip_prefix("throw to monkey ")
            .unwrap()
            .parse::<u64>()?;
        let false_monkey = lines[4]
            .strip_prefix("throw to monkey ")
            .unwrap()
            .parse::<u64>()?;

        let operation = match lines[1].strip_prefix("new = old ").unwrap() {
            "* old" => Operation::Square,
            other => {
                let (op, val) = other.split_once(' ').unwrap();
                let value = val.parse::<u64>()?;
                match op {
                    "*" => Operation::Mulitply(value),
                    _ => Operation::Add(value),
                }
            }
        };

        Ok(Monkey::new(
            VecDeque::from(items),
            operation,
            test,
            true_monkey,
            false_monkey,
        ))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = input
        .split("\n\n")
        .filter_map(|line| line.parse::<Monkey>().ok())
        .collect_vec();
    for _ in 0..20 {
        for index in 0..monkeys.len() {
            while let Some(item) = monkeys[index].items.pop_front() {
                //inspect item
                let monkey = &monkeys[index];
                let item = match monkey.operation {
                    Operation::Mulitply(factor) => item * factor,
                    Operation::Square => item * item,
                    Operation::Add(term) => item + term,
                } / 3;
                let target = if item % monkey.divisor == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };
                monkeys[index].total_inspections += 1;
                monkeys[target as usize].items.push_back(item);
            }
        }
    }
    Some(
        monkeys
            .into_iter()
            .map(|monkey| monkey.total_inspections as u32)
            .sorted()
            .rev()
            .take(2)
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = input
        .split("\n\n")
        .filter_map(|line| line.parse::<Monkey>().ok())
        .collect_vec();

    let modulo = monkeys.iter().fold(1, |acc, monkey| {
        if acc % monkey.divisor == 0 {
            acc
        } else {
            acc * monkey.divisor
        }
    });
    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            while let Some(item) = monkeys[index].items.pop_front() {
                //inspect item
                let monkey = &monkeys[index];
                let item = match monkey.operation {
                    Operation::Mulitply(factor) => item * factor,
                    Operation::Square => item * item,
                    Operation::Add(term) => item + term,
                } % modulo;

                let target = if item % monkey.divisor == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };
                monkeys[index].total_inspections += 1;
                monkeys[target as usize].items.push_back(item);
            }
        }
    }
    Some(
        monkeys
            .into_iter()
            .map(|monkey| monkey.total_inspections as u64)
            .sorted()
            .rev()
            .take(2)
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
