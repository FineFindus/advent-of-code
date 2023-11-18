use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn move_by(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }

    pub fn distance_to(&self, other: Position) -> i32 {
        ((self.x as f32 - other.x as f32).powi(2) + (self.y as f32 - other.y as f32).powi(2)).sqrt()
            as i32
    }
}

pub fn move_tail(head: Position, tail: &mut Position) {
    let xdiff = head.x - tail.x;
    let ydiff = head.y - tail.y;
    if xdiff.abs() <= 1 && ydiff.abs() <= 1 {
        return;
    }
    tail.x += xdiff.signum();
    tail.y += ydiff.signum();
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut head = Position::default();
    let mut tail = Position::default();

    // add start pos
    visited.insert(tail);

    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(direction, distance)| (direction.as_bytes()[0], distance.parse::<isize>().unwrap()))
        .for_each(|(direction, distance)| {
            for _ in 0..distance {
                match direction {
                    b'U' => head.move_by(0, 1),
                    b'D' => head.move_by(0, -1),
                    b'L' => head.move_by(-1, 0),
                    b'R' => head.move_by(1, 0),
                    _ => {}
                }
                move_tail(head, &mut tail);
                visited.insert(tail);
            }
        });

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut head = Position::default();
    let mut rope = vec![Position::default(); 9];

    // add start pos
    visited.insert(rope[0]);

    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(direction, distance)| (direction.as_bytes()[0], distance.parse::<isize>().unwrap()))
        .for_each(|(direction, distance)| {
            for _ in 0..distance {
                match direction {
                    b'U' => head.move_by(0, 1),
                    b'D' => head.move_by(0, -1),
                    b'L' => head.move_by(-1, 0),
                    b'R' => head.move_by(1, 0),
                    _ => {}
                }

                move_tail(head, &mut rope[0]);
                for index in 1..rope.len() {
                    let prev = rope[index - 1];
                    let standing_line = rope.get_mut(index).unwrap();
                    move_tail(prev, standing_line);
                }
                visited.insert(*rope.last().unwrap());
            }
        });

    Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
