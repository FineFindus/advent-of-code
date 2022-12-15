use std::fmt::{Display, Formatter, Write};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Area {
    Rock,
    Sand,
    Air,
}

struct Grid {
    items: Vec<Vec<Area>>,
}

impl Display for Grid {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in &self.items {
            for area in row {
                match area {
                    Area::Air => fmt.write_char('.'),
                    Area::Rock => fmt.write_char('#'),
                    Area::Sand => fmt.write_char('o'),
                }?;
            }
            fmt.write_char('\n')?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            items: vec![vec![Area::Air; width]; height],
        }
    }

    pub fn set_rock_path(&mut self, start: (usize, usize), end: (usize, usize)) {
        let diff_x = end.0 as isize - start.0 as isize;
        let diff_y = end.1 as isize - start.1 as isize;

        for i in 0..=diff_x.abs().max(diff_y.abs()) {
            let x_coord = start.0 as isize + (diff_x.signum() * i);
            let y_coord = start.1 as isize + (diff_y.signum() * i);
            self.items[y_coord as usize][x_coord as usize] = Area::Rock;
        }
    }

    pub fn spawn_sand(&mut self, source: usize) -> bool {
        let mut x = source;
        let mut y = 0;

        loop {
            if x == 0 || x + 1 == self.items[0].len() || y + 1 == self.items.len() {
                return false;
            }

            //move down
            if self.items[y + 1][x] == Area::Air {
                y += 1;
                continue;
            } else if self.items[y + 1][x.saturating_sub(1)] == Area::Air {
                x -= 1;
                continue;
            } else if self.items[y + 1][x + 1] == Area::Air {
                x += 1;
                continue;
            } else if self.items[y][x] == Area::Air {
                //rest
                self.items[y][x] = Area::Sand;
                break;
            } else {
                return false;
            }
        }
        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rocks = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .filter_map(|split| split.split_once(','))
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect_vec()
        })
        .collect_vec();
    let min_x = rocks.iter().flatten().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = rocks.iter().flatten().max_by_key(|(x, _)| x).unwrap().0 + 1;
    let max_y = rocks.iter().flatten().max_by_key(|(_, y)| y).unwrap().1 + 1;

    let mut cave = Grid::new(max_y, max_x - min_x);

    for path in rocks {
        let mut path_iter = path.iter().peekable();
        while let Some(start) = path_iter.next() {
            if let Some(end) = path_iter.peek() {
                cave.set_rock_path((start.0 - min_x, start.1), (end.0 - min_x, end.1))
            }
        }
    }

    //let the sand fall
    let mut sand_counter = 0;
    let sand_source = 500 - min_x;
    loop {
        if !cave.spawn_sand(sand_source) {
            break;
        }
        sand_counter += 1;
    }
    Some(sand_counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rocks = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .filter_map(|split| split.split_once(','))
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect_vec()
        })
        .collect_vec();
    //stupid hack
    //simply increasing the width, instead of solving it a smarter way
    let min_x = rocks.iter().flatten().min_by_key(|(x, _)| x).unwrap().0 - 200;
    let max_x = rocks.iter().flatten().max_by_key(|(x, _)| x).unwrap().0 + 200;
    let max_y = rocks.iter().flatten().max_by_key(|(_, y)| y).unwrap().1 + 1;

    let mut cave = Grid::new(max_y + 2, max_x - min_x);

    for path in rocks {
        let mut path_iter = path.iter().peekable();
        while let Some(start) = path_iter.next() {
            if let Some(end) = path_iter.peek() {
                cave.set_rock_path((start.0 - min_x, start.1), (end.0 - min_x, end.1))
            }
        }
    }
    cave.set_rock_path((0, max_y + 1), (cave.items[0].len() - 1, max_y + 1));

    //let the sand fall
    let mut sand_counter = 0;
    let sand_source = 500 - min_x;
    loop {
        if !cave.spawn_sand(sand_source) {
            break;
        }
        sand_counter += 1;
    }
    Some(sand_counter)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
