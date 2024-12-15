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

fn predict_robot_position(
    start_position: (i32, i32),
    velocity: (i32, i32),
    steps: i32,
) -> (i32, i32) {
    (
        (start_position.0 + velocity.0 * steps).rem_euclid(WIDTH),
        (start_position.1 + velocity.1 * steps).rem_euclid(HEIGHT),
    )
}

fn calculate_safety_score(positions: &[(i32, i32)]) -> Option<u32> {
    let mut quadrants = [0; 4];
    for position in positions {
        let upper = position.1 > HEIGHT / 2;
        let right = position.0 > WIDTH / 2;
        let index = upper as usize * 2 + right as usize;
        // only increase the count if the position actually falls into one of the quadrants
        quadrants[index] += (position.0 != WIDTH / 2 && position.1 != HEIGHT / 2) as u32;
    }
    quadrants.iter().product1()
}

fn parse_robots(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")
        .expect("`regex` should be a valid regex")
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, digits)| digits.map(|digit| digit.parse::<i32>().unwrap()))
        .map(|[px, py, vx, vy]| ((px, py), (vx, vy)))
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let positions = parse_robots(input)
        .into_iter()
        .map(|(position, velocity)| predict_robot_position(position, velocity, 100))
        .collect_vec();
    calculate_safety_score(&positions)
}

pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse_robots(input);
    let mut positions = robots.iter().map(|(pos, _vel)| *pos).collect_vec();
    let time = (1..WIDTH * HEIGHT)
        .filter_map(|time| {
            positions = positions
                .iter()
                .enumerate()
                .map(|(index, &position)| predict_robot_position(position, robots[index].1, 1))
                .collect_vec();
            Some((time, calculate_safety_score(&positions)?))
        })
        .min_by_key(|(_time, score)| *score)?
        .0 as u32;
    Some(time)
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
        // this is just to ensure that the test still passes, there is not christmas tree in the
        // test data
        assert_eq!(result, Some(5));
    }
}
