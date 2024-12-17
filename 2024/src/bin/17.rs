use itertools::Itertools;

advent_of_code::solution!(17);

fn parse_program(input: &str) -> Option<(u64, u64, u64, Vec<u64>)> {
    let (registers, program) = input.split_once("\n\n")?;
    let (reg_a, reg_b, reg_c) = registers
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(_, val)| val.parse::<u64>().ok())
        .collect_tuple()?;
    let instructions = program
        .strip_prefix("Program: ")?
        .strip_suffix("\n")?
        .split(',')
        .map(|v| v.parse::<u64>().unwrap())
        .collect_vec();
    Some((reg_a, reg_b, reg_c, instructions))
}

fn execute(mut reg_a: u64, mut reg_b: u64, mut reg_c: u64, instructions: &[u64]) -> Vec<u64> {
    let mut output = Vec::new();

    let mut instruction_ptr = 0;
    while instruction_ptr < instructions.len() {
        let opcode = instructions[instruction_ptr];
        let operand = instructions[instruction_ptr + 1];
        let combo_operand_mapping = [0, 1, 2, 3, reg_a, reg_b, reg_c];
        let combo_operand = combo_operand_mapping[operand as usize];

        match opcode {
            //adv
            0 => reg_a >>= combo_operand,
            //bxl
            1 => reg_b ^= operand,
            // bst
            2 => reg_b = combo_operand % 8,
            // jnz
            3 => {
                if reg_a != 0 {
                    instruction_ptr = operand as usize;
                    continue;
                }
            }
            // bxc
            4 => reg_b ^= reg_c,
            // out
            5 => {
                output.push(combo_operand % 8);
            }
            // bdv
            6 => reg_b = reg_a >> combo_operand,
            // cdv
            7 => reg_c = reg_a >> combo_operand,
            v => unreachable!("Found expected opcode {v}"),
        }
        instruction_ptr += 2;
    }
    output
}

pub fn part_one(input: &str) -> Option<String> {
    let (reg_a, reg_b, reg_c, instructions) = parse_program(input)?;

    Some(
        execute(reg_a, reg_b, reg_c, &instructions)
            .into_iter()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, reg_b, reg_c, instructions) = parse_program(input)?;
    let mut reg_a = 0;
    let mut prev_correct = 0;

    loop {
        let output = execute(reg_a, reg_b, reg_c, &instructions);
        // find number of correct digits from the end
        let updated_correct = instructions
            .iter()
            .rev()
            .zip(output.iter().rev())
            .take_while(|(ins, out)| ins == out)
            .count();

        if updated_correct == instructions.len() {
            return Some(reg_a);
        }

        if prev_correct < updated_correct {
            prev_correct = updated_correct;
            reg_a <<= 3;
        } else {
            reg_a += 1;
        }
    }
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
        assert_eq!(result, Some(117440));
    }
}
