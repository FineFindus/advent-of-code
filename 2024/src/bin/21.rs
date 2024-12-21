use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(21);

fn find_numeric_path(grid: &[Vec<char>], start: (i32, i32), goal: (i32, i32)) -> Vec<char> {
    let dy = goal.0 - start.0;
    let dx = goal.1 - start.1;

    // let covers_empty_spot = (goal.0 == 0 && start.1 == -2) || (start.0 == 0 && goal.1 == -2);
    let mut directions = Vec::new();
    let horiz = Vec::from_iter(
        std::iter::repeat(if dx >= 0 { '>' } else { '<' }).take(dx.unsigned_abs() as usize),
    );

    let vert = Vec::from_iter(
        std::iter::repeat(if dy >= 0 { '^' } else { 'v' }).take(dy.unsigned_abs() as usize),
    );

    if start.0 == 0 && goal.1 == 0 {
        directions.extend(vert);
        directions.extend(horiz);
    } else if start.1 == 0 && goal.0 == 0 {
        directions.extend(horiz);
        directions.extend(vert);
    } else if dx < 0 {
        directions.extend(horiz);
        directions.extend(vert);
    } else {
        directions.extend(vert);
        directions.extend(horiz);
    }
    directions
}
fn find_dir_path(grid: &[Vec<char>], start: (i32, i32), goal: (i32, i32)) -> Vec<char> {
    let dy = goal.0 - start.0;
    let dx = goal.1 - start.1;

    // let covers_empty_spot = (goal.0 == 0 && start.1 == -2) || (start.0 == 0 && goal.1 == -2);
    let mut directions = Vec::new();
    let horiz = Vec::from_iter(
        std::iter::repeat(if dx >= 0 { '>' } else { '<' }).take(dx.unsigned_abs() as usize),
    );

    let vert = Vec::from_iter(
        std::iter::repeat(if dy >= 0 { '^' } else { 'v' }).take(dy.unsigned_abs() as usize),
    );

    if start.1 == 0 && goal.0 == 1 {
        directions.extend(horiz);
        directions.extend(vert);
    } else if start.0 == 1 && goal.0 == 0 {
        directions.extend(vert);
        directions.extend(horiz);
    } else if dx < 0 {
        directions.extend(horiz);
        directions.extend(vert);
    } else {
        directions.extend(vert);
        directions.extend(horiz);
    }
    directions
}

fn encode_numeric_keypad(keypad: &[Vec<char>], start: (i32, i32), keys: &[char]) -> Vec<char> {
    // Directional map with (i32, i32) coordinates
    let mut numerical_map: HashMap<char, (i32, i32)> = HashMap::new();
    numerical_map.insert('A', (0, 2));
    numerical_map.insert('0', (0, 1));
    numerical_map.insert('1', (1, 0));
    numerical_map.insert('2', (1, 1));
    numerical_map.insert('3', (1, 2));
    numerical_map.insert('4', (2, 0));
    numerical_map.insert('5', (2, 1));
    numerical_map.insert('6', (2, 2));
    numerical_map.insert('7', (3, 0));
    numerical_map.insert('8', (3, 1));
    numerical_map.insert('9', (3, 2));

    let mut position = numerical_map[&'A'];
    let mut directions = Vec::new();
    for key in keys {
        let goal = numerical_map[key];
        if position != goal {
            let path = find_numeric_path(keypad, position, goal);
            directions.extend(path);
        }

        directions.push('A');
        position = goal;
    }
    directions
}

fn encode_keypad(keypad: &[Vec<char>], start: (i32, i32), keys: &[char]) -> Vec<char> {
    let mut directional_map: HashMap<char, (i32, i32)> = HashMap::new();
    directional_map.insert('A', (1, 2));
    directional_map.insert('^', (1, 1));
    directional_map.insert('<', (0, 0));
    directional_map.insert('v', (0, 1));
    directional_map.insert('>', (0, 2));
    let mut position = directional_map[&'A'];
    let mut directions = Vec::new();
    for key in keys {
        let goal = directional_map[key];
        if position != goal {
            let path = find_dir_path(keypad, position, goal);
            directions.extend(path);
        }
        directions.push('A');
        position = goal;
    }
    directions
}

fn numeric_value(chars: &[char]) -> u32 {
    let mut result = 0;
    for (i, c) in chars.iter().enumerate() {
        // Check if the character is a digit
        if let Some(digit) = c.to_digit(10) {
            result += digit * 10u32.pow((2 - i) as u32);
        } else {
            return 0;
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let keycodes = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let numeric_keypad = [
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];
    const NUMERIC_KEYPAD_A: (i32, i32) = (3, 2);

    let directional_keypad = [vec![' ', '^', 'A'], vec!['<', 'v', '>']];
    const DIRECTIONAL_KEYPAD_A: (i32, i32) = (0, 2);
    let mut complexity = 0;
    for keys in keycodes {
        let numeric_input = encode_numeric_keypad(&numeric_keypad, NUMERIC_KEYPAD_A, &keys);
        dbg!(numeric_input.iter().join(""));
        let robot_directional_input =
            encode_keypad(&directional_keypad, DIRECTIONAL_KEYPAD_A, &numeric_input);
        dbg!(robot_directional_input.iter().join(""));
        let directional_input = encode_keypad(
            &directional_keypad,
            DIRECTIONAL_KEYPAD_A,
            &robot_directional_input,
        );
        dbg!(directional_input.iter().join(""));
        let length = directional_input.len() as u32;
        dbg!(length);
        complexity += numeric_value(&keys[..3]) * length;
    }
    Some(complexity)
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
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
