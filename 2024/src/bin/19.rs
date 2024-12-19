advent_of_code::solution!(19);

fn is_design_possible(design: &str, towels: &mut Vec<&str>) -> bool {
    if design.is_empty() {
        return true;
    }

    for index in 0..towels.len() {
        let towel = towels[index];
        if let Some(design) = design.strip_prefix(towel) {
            if is_design_possible(design, towels) {
                return true;
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, designs) = input.split_once("\n\n")?;
    let mut towels: Vec<&str> = towels.split(", ").collect();

    let possible_designs = designs
        .lines()
        .filter(|design| is_design_possible(design, &mut towels))
        .count();
    Some(possible_designs as u32)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
