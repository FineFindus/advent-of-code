use std::collections::HashMap;

advent_of_code::solution!(21);

// inspired by https://gist.github.com/KateMorley/5c3987ecd5a214b6ebc0311bb4e0c339

enum Keypad {
    Numeric,
    Directional,
}

impl Keypad {
    pub fn is_gap(&self, x: i32, y: i32) -> bool {
        match self {
            Self::Numeric => x == 0 && y == 3,
            Self::Directional => x == 0 && y == 0,
        }
    }

    pub fn position(&self, key: char) -> (i32, i32) {
        if key == 'A' {
            return match self {
                Keypad::Numeric => (2, 3),
                Keypad::Directional => (2, 0),
            };
        }

        match key {
            // numeric keys
            '7' => (0, 0),
            '8' => (1, 0),
            '9' => (2, 0),
            '4' => (0, 1),
            '5' => (1, 1),
            '6' => (2, 1),
            '1' => (0, 2),
            '2' => (1, 2),
            '3' => (2, 2),
            '0' => (1, 3),
            // directional keys
            '^' => (1, 0),
            '<' => (0, 1),
            'v' => (1, 1),
            '>' => (2, 1),
            _ => unreachable!("Found unexpected key: {key}"),
        }
    }
}

struct Robot {
    keypad: Keypad,
    cache: HashMap<(char, char), u64>,
    next: Option<Box<Robot>>,
}

impl Robot {
    fn new(keypad: Keypad, next: Option<Box<Robot>>) -> Self {
        Self {
            keypad,
            cache: HashMap::new(),
            next,
        }
    }

    fn encode_length(&mut self, current_key: char, next_key: char) -> u64 {
        if let Some(pushes) = self.cache.get(&(current_key, next_key)) {
            return *pushes;
        }

        let (current_x, current_y) = self.keypad.position(current_key);
        let (next_x, next_y) = self.keypad.position(next_key);

        let dx = next_x - current_x;
        let dy = next_y - current_y;

        let key_horizontal = if dx > 0 { '>' } else { '<' };
        let key_vertical = if dy > 0 { 'v' } else { '^' };

        let mut pushes = u64::MAX;

        if !self.keypad.is_gap(current_x, next_y) {
            pushes = pushes.min(self.encode_next(key_vertical, dy.abs(), key_horizontal, dx.abs()))
        };

        if !self.keypad.is_gap(next_x, current_y) {
            pushes = pushes.min(self.encode_next(key_horizontal, dx.abs(), key_vertical, dy.abs()))
        };

        self.cache.insert((current_key, next_key), pushes);

        pushes
    }

    fn encode_next(&mut self, key_1: char, times_1: i32, key_2: char, times_2: i32) -> u64 {
        if let Some(next) = self.next.as_mut() {
            let mut pushes = 0;
            let mut last_key = 'A';

            for (key, times) in [(key_1, times_1), (key_2, times_2)] {
                for _ in 0..times {
                    pushes += next.encode_length(last_key, key);
                    last_key = key
                }
            }

            pushes + next.encode_length(last_key, 'A')
        } else {
            (times_1 + times_2 + 1) as u64
        }
    }
}

fn compute_complexity(input: &str, robots: usize) -> u64 {
    let mut next = None;
    for _ in 0..robots {
        next = Some(Box::new(Robot::new(Keypad::Directional, next)));
    }

    let mut robot = Robot::new(Keypad::Numeric, next);

    let mut complexity = 0;
    for keys in input.lines() {
        let numeric_value = &keys[..keys.len() - 1].parse::<u64>().unwrap();

        let mut current_key = 'A';
        let mut length = 0;
        for next_key in keys.chars() {
            length += robot.encode_length(current_key, next_key);
            current_key = next_key;
        }

        complexity += numeric_value * length;
    }
    complexity
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(compute_complexity(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(compute_complexity(input, 25))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
