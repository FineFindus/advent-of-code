use itertools::Itertools;

advent_of_code::solution!(13);

fn calculate_price_combination(a: (u32, u32), b: (u32, u32), price: (u32, u32)) -> Option<u32> {
    (0..100)
        .cartesian_product(0..100)
        .map(|(pressed_a, pressed_b)| {
            (
                pressed_a,
                pressed_b,
                (
                    pressed_a * a.0 + pressed_b * b.0,
                    pressed_a * a.1 + pressed_b * b.1,
                ),
            )
        })
        .filter(|(_, _, location)| location == &price)
        .map(|(pressed_a, pressed_b, _)| pressed_a * 3 + pressed_b)
        .min()
}

pub fn part_one(input: &str) -> Option<u32> {
    //TODO: maybe not use regex to extract the numbers
    regex::Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .expect("`regex` should be valid")
    .captures_iter(input)
    .map(|c| c.extract())
    .map(|(_, values)| values.map(|v| v.parse::<u32>().unwrap()))
    .filter_map(|[a_x, a_y, b_x, b_y, p_x, p_y]| {
        calculate_price_combination((a_x, a_y), (b_x, b_y), (p_x, p_y))
    })
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
