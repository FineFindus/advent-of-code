use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut block_id = 0;
    let mut disk_map = input
        .chars()
        .filter_map(|char| char.to_digit(10))
        .enumerate()
        .flat_map(|(idx, len)| {
            block_id += (idx % 2 == 0) as u64;
            vec![
                if idx % 2 == 0 {
                    Some(block_id - 1)
                } else {
                    None
                };
                len as usize
            ]
        })
        .collect_vec();

    for index in 0..disk_map.len() {
        if disk_map[index].is_some() {
            continue;
        }

        // find last available block and move it
        let mut last_full_block = disk_map.len() - 1;
        while disk_map[last_full_block].is_none() && last_full_block > index {
            last_full_block -= 1;
        }

        if disk_map[last_full_block].is_some() {
            disk_map.swap(index, last_full_block);
        }
    }

    disk_map
        .iter()
        .enumerate()
        .map(|(idx, val)| idx as u64 * val.unwrap_or(0))
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
