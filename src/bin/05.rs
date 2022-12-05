use itertools::Itertools;

pub fn part_one(input: &str) -> Option<String> {
    let (stack_input, operations_input) = input.split_once("\n\n").unwrap();

    let mut stack: Vec<Vec<char>> =
        vec![vec![]; (stack_input.lines().next().unwrap().len() + 1) / 4];
    //parse stack
    stack_input
        .lines()
        .rev()
        .skip(1)
        .map(|line| line.chars().skip(1).step_by(4).collect_vec())
        .for_each(|element| {
            element.iter().enumerate().for_each(|(index, element)| {
                if element != &' ' {
                    stack[index].push(*element)
                }
            })
        });

    //parse operations
    operations_input
        .lines()
        .filter_map(|line| {
            line.split(' ')
                .flat_map(|split| split.parse::<usize>().ok())
                .collect_tuple::<(usize, usize, usize)>()
        })
        .map(|(amount, source, target)| (amount, source - 1, target - 1))
        .for_each(|(amount, source, target)| {
            let length = &stack[source].len();
            let moving = &mut stack[source].split_off(length - amount);
            moving.reverse();
            stack[target].append(moving);
        });

    let top = stack.iter().filter_map(|vec| vec.last()).join("");
    Some(top)
}

pub fn part_two(input: &str) -> Option<String> {
    let (stack_input, operations_input) = input.split_once("\n\n").unwrap();
    let mut stack: Vec<Vec<char>> =
        vec![vec![]; (stack_input.lines().next().unwrap().len() + 1) / 4];

    //parse stack
    stack_input
        .lines()
        .rev()
        .skip(1)
        .map(|line| line.chars().skip(1).step_by(4).collect_vec())
        .for_each(|element| {
            element.iter().enumerate().for_each(|(index, element)| {
                if element != &' ' {
                    stack[index].push(*element)
                }
            })
        });

    //parse operations
    operations_input
        .lines()
        .filter_map(|line| {
            line.split(' ')
                .flat_map(|split| split.parse::<usize>().ok())
                .collect_tuple::<(usize, usize, usize)>()
        })
        .map(|(amount, source, target)| (amount, source - 1, target - 1))
        .for_each(|(amount, source, target)| {
            let length = &stack[source].len();
            let moving = &mut stack[source].split_off(length - amount);
            stack[target].append(moving);
        });

    let top = stack.iter().filter_map(|vec| vec.last()).join("");
    Some(top)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
