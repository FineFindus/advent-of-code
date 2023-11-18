use std::{collections::HashMap, path::PathBuf};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut path = PathBuf::new();
    let mut files: HashMap<PathBuf, u32> = HashMap::new();

    input
        .split('$')
        .skip(1)
        .for_each(|lines| match lines.trim().lines().next() {
            Some("ls") => {
                let size = lines
                    .lines()
                    .skip(1)
                    .filter_map(|line| line.split_once(' '))
                    .filter_map(|(identifier, _value)| match identifier {
                        "dir" => None,
                        value => value.parse::<u32>().ok(),
                    })
                    .sum::<u32>();
                files.insert(path.clone(), size);
                for ancestors in path.ancestors().skip(1) {
                    files
                        .entry(ancestors.to_path_buf())
                        .and_modify(|parent_size| *parent_size += size);
                }
            }
            Some("cd ..") => {
                path.pop();
            }
            Some(value) => {
                let dir = value.split_once(' ').unwrap().1;
                path.push(dir);
            }
            None => {}
        });
    files.values().filter(|dir| dir <= &&100000).sum1()
}
pub fn part_two(input: &str) -> Option<u32> {
    const MAX_SIZE: u32 = 70000000;
    const MIN_UNUSED: u32 = 30000000;
    let mut path = PathBuf::new();
    let mut files: HashMap<PathBuf, u32> = HashMap::new();

    input
        .split('$')
        .skip(1)
        .for_each(|lines| match lines.trim().lines().next() {
            Some("ls") => {
                let size = lines
                    .lines()
                    .skip(1)
                    .filter_map(|line| line.split_once(' '))
                    .filter_map(|(identifier, _value)| match identifier {
                        "dir" => None,
                        value => value.parse::<u32>().ok(),
                    })
                    .sum::<u32>();
                files.insert(path.clone(), size);
                for ancestors in path.ancestors().skip(1) {
                    files
                        .entry(ancestors.to_path_buf())
                        .and_modify(|parent_size| *parent_size += size);
                }
            }
            Some("cd ..") => {
                path.pop();
            }
            Some(value) => {
                let dir = value.split_once(' ').unwrap().1;
                path.push(dir);
            }
            None => {}
        });
    let currently_unused = MAX_SIZE - files.get(&PathBuf::from("/")).unwrap();
    let min_delete = MIN_UNUSED - currently_unused;
    files.into_values().filter(|dir| dir >= &min_delete).min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
