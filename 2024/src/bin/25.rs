advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let schematics: Vec<u64> = input
        .split("\n\n")
        .map(|schematic| {
            assert!(schematic.len() <= 64);
            schematic
                .bytes()
                .filter(|char| char != &b'\n')
                .enumerate()
                .fold(0, |acc, (index, char)| {
                    // we use a u64 to store the schematics in reverse, i.e. bit0 is last schematic
                    // cell
                    acc | ((char == b'#') as u64) << index as u64
                })
        })
        .collect();

    let (locks, keys): (Vec<u64>, Vec<u64>) = schematics
        .iter()
        .partition(|&&schematic| (schematic >> 30) & 1 == 1);

    let nonoverlapping = locks
        .iter()
        .map(|&lock| keys.iter().filter(|&&key| (lock & key) == 0).count() as u32)
        .sum();
    Some(nonoverlapping)
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
