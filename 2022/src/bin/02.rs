use itertools::Itertools;

const WIN_TABLE: [[u8; 3]; 3] = [
    //  X  Y  Z
    [1, 2, 0], // A (rock)
    [0, 1, 2], // B (paper)
    [2, 0, 1], // C (scissors)
];

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|round| round.as_bytes())
        .map(|bytes| (bytes[0] - b'A', bytes[2] - b'X'))
        .map(|(elf, player)| WIN_TABLE[elf as usize][player as usize] * 3 + player + 1)
        .map(|points| points as u32)
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|round| round.as_bytes())
        .map(|bytes| (bytes[0] - b'A', bytes[2] - b'X'))
        .map(|(elf, player)| {
            (
                elf,
                WIN_TABLE[elf as usize]
                    .into_iter()
                    .position(|item| item == player)
                    .unwrap() as u8,
            )
        })
        .map(|(elf, position)| WIN_TABLE[elf as usize][position as usize] * 3 + position + 1)
        .map(|points| points as u32)
        .sum1()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
