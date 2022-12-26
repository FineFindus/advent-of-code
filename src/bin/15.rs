use itertools::Itertools;

pub fn parse(input: &str) -> Vec<((isize, isize), (isize, isize))> {
    input
        .lines()
        .filter_map(|line| line.split_once(':'))
        .map(|(sensor, beacon)| {
            (
                sensor
                    .strip_prefix("Sensor at ")
                    .unwrap()
                    .split_once(", ")
                    .unwrap(),
                beacon
                    .strip_prefix(" closest beacon is at ")
                    .unwrap()
                    .split_once(", ")
                    .unwrap(),
            )
        })
        .map(|(sensor, beacon)| {
            (
                (
                    sensor.0.strip_prefix("x=").unwrap(),
                    sensor.1.strip_prefix("y=").unwrap(),
                ),
                (
                    beacon.0.strip_prefix("x=").unwrap(),
                    beacon.1.strip_prefix("y=").unwrap(),
                ),
            )
        })
        .map(|(sensor, beacon)| {
            (
                (
                    sensor.0.parse::<isize>().unwrap(),
                    sensor.1.parse::<isize>().unwrap(),
                ),
                (
                    beacon.0.parse::<isize>().unwrap(),
                    beacon.1.parse::<isize>().unwrap(),
                ),
            )
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let positions = parse(input);
    let search_line = 10; // change this to 2000000 for solving
    let sensors = positions
        .iter()
        .filter(|(sensor, beacon)| {
            let distance = (beacon.0 - sensor.0).abs() + (beacon.1 - sensor.1).abs();
            let sensor_range = (sensor.1 - distance)..(sensor.1 + distance);
            sensor_range.contains(&search_line)
        })
        .flat_map(|((sx, sy), (bx, by))| {
            let distance = (bx - sx).abs() + (by - sy).abs();
            let line_offset = sy - search_line;
            let dist_on_line = distance - line_offset.abs();
            (sx - dist_on_line)..=sx + dist_on_line
        })
        .unique()
        .filter(|&x| {
            positions
                .clone()
                .into_iter()
                .find(|item| item.1 .0 == x && item.1 .1 == search_line)
                .is_none()
        })
        .count();

    Some(sensors as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let positions = parse(input);
    let max_search_space = 4000000; // change to 4000000 for solving
    //let max_search_space = 20; // change to 4000000 for solving

    let points = positions
        .iter()
        .flat_map(|(sensor, beacon)| {
            let radius = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs() + 1;
            //convert to points outside radius
            let mut points = Vec::new();
            for x in 0..=radius {
                let dy = radius - x;
                points.push((sensor.0 + x, sensor.1 + dy));
                points.push((sensor.0 + x, sensor.1 - dy));
                points.push((sensor.0 - x, sensor.1 + dy));
                points.push((sensor.0 - x, sensor.1 - dy));
            }
            points
        })
    .sorted().dedup_with_count()
        .sorted_by_key(|item| item.0)
        .rev()
        .map(|(_, item)| item)
    .collect::<Vec<(isize, isize)>>();


    let distress_beacon = points.iter().filter(|(x, y)| x >= &0 && x < &max_search_space && y >= &0 && y < &max_search_space).filter(|(x,y)| {

let blocked = positions
            .iter()
            .filter(|(sensor, beacon)| {
                let distance = (beacon.0 - sensor.0).abs() + (beacon.1 - sensor.1).abs();
                let sensor_range =
                    ((sensor.1 - distance).max(0))..((sensor.1 + distance).min(max_search_space));
                sensor_range.contains(&y)
            })
            .flat_map(|((sx, sy), (bx, by))| {
                let distance = (bx - sx).abs() + (by - sy).abs();
                let line_offset = sy - y;
                let dist_on_line = distance - line_offset.abs();
                ((sx - dist_on_line).max(0))..=(sx + dist_on_line).min(max_search_space)
            })
            .unique().collect_vec();

        !blocked.contains(x)
    }).inspect(|item| println!("{:?}", item)).next().unwrap();


    Some((distress_beacon.0 * 4000000 + distress_beacon.1 ) as u64)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
