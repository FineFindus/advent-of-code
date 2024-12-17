use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let (registers, program) = input.split_once("\n\n")?;
    let (mut reg_a, mut reg_b, mut reg_c) = registers
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(_, val)| val.parse::<u32>().ok())
        .collect_tuple()?;
    let instructions = program
        .strip_prefix("Program: ")?
        .strip_suffix("\n")?
        .split(',')
        .map(|v| v.parse::<u32>().unwrap())
        .collect_vec();

    let mut output = Vec::new();
    let mut instruction_ptr = 0;
    while instruction_ptr < instructions.len() {
        let opcode = instructions[instruction_ptr];
        let operand = instructions[instruction_ptr + 1];
        let combo_operand_mapping = [0, 1, 2, 3, reg_a, reg_b, reg_c];
        let combo_operand = combo_operand_mapping[operand as usize];
        let mut increase_instruction_ptr = true;
        match opcode {
            //adv
            0 => reg_a /= 2u32.pow(combo_operand),
            //bxl
            1 => reg_b ^= operand,
            // bst
            2 => reg_b = combo_operand % 8,
            // jnz
            3 => {
                if reg_a != 0 {
                    instruction_ptr = operand as usize;
                    increase_instruction_ptr = false;
                }
            }
            // bxc
            4 => reg_b ^= reg_c,
            // out
            5 => {
                output.push(combo_operand % 8);
            }
            // bdv
            6 => reg_b = reg_a / 2u32.pow(combo_operand),
            // cdv
            7 => reg_c = reg_a / 2u32.pow(combo_operand),
            v => unreachable!("Found expected opcode {v}"),
        }
        if increase_instruction_ptr {
            instruction_ptr += 2;
        }
    }

    Some(output.into_iter().join(","))
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
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
