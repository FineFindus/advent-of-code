use std::collections::BTreeMap;

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

pub fn part_two(input: &str) -> Option<u64> {
    let mut block_id = 0;
    let (files, free_space) = input
        .chars()
        .filter_map(|char| char.to_digit(10))
        .map(|size| size as u64)
        .enumerate()
        .fold(
            (vec![], vec![]),
            |(mut files, mut free_space), (idx, size)| {
                if size > 0 {
                    if idx % 2 == 0 {
                        files.push((block_id, size));
                    } else {
                        free_space.push((block_id, size));
                    }
                    block_id += size;
                }

                (files, free_space)
            },
        );

    let mut free_space_ids: BTreeMap<u64, u64> = BTreeMap::from_iter(free_space);
    let mut checksum = 0;

    for (block_id, &(file_block_id, file_size)) in files.iter().enumerate().rev() {
        // check if there is any free space before our file
        let free_space = free_space_ids
            .iter()
            .take_while(|&(&id, _size)| id < file_block_id)
            .find(|&(_id, &block_size)| block_size >= file_size);

        let new_file_block_id = if let Some((&free_space_id, &free_space_size)) = free_space {
            free_space_ids.remove(&free_space_id);
            // re-add left-over space
            if free_space_size > file_size {
                free_space_ids.insert(free_space_id + file_size, free_space_size - file_size);
            }
            free_space_id
        } else {
            file_block_id
        };

        // arithmetic sum, see https://pastebin.com/HrJPUAdq
        checksum += (block_id as u64) * (new_file_block_id * 2 + file_size - 1) * file_size / 2;
    }

    checksum.into()
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
        assert_eq!(result, Some(2858));
    }
}
