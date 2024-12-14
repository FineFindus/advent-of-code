use itertools::Itertools;

advent_of_code::solution!(14);

#[cfg(not(test))]
const WIDTH: i32 = 101;
#[cfg(test)]
const WIDTH: i32 = 11;
#[cfg(not(test))]
const HEIGHT: i32 = 103;
#[cfg(test)]
const HEIGHT: i32 = 7;

fn predict_robot_position(start_position: (i32, i32), velocity: (i32, i32)) -> (i32, i32) {
    let mut position = start_position;
    for _ in 0..100 {
        position = (
            (position.0 + velocity.0).rem_euclid(WIDTH),
            (position.1 + velocity.1).rem_euclid(HEIGHT),
        );
    }
    position
}

pub fn part_one(input: &str) -> Option<u32> {
    let positions = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")
        .expect("`regex` should be a valid regex")
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, digits)| digits.map(|digit| digit.parse::<i32>().unwrap()))
        .map(|[px, py, vx, vy]| predict_robot_position((px, py), (vx, vy)))
        .collect_vec();
    let mut quadrants = [0; 4];
    for position in positions {
        if position.1 < HEIGHT / 2 {
            if position.0 < WIDTH / 2 {
                quadrants[0] += 1;
            } else if position.0 > (WIDTH / 2) {
                quadrants[1] += 1;
            }
        } else if position.1 > HEIGHT / 2 {
            if position.0 < WIDTH / 2 {
                quadrants[2] += 1;
            } else if position.0 > (WIDTH / 2) {
                quadrants[3] += 1;
            }
        }
    }
    quadrants.iter().product1()
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
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
