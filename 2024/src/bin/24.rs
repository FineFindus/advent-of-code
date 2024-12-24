use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateType {
    And,
    Or,
    Xor,
}
impl FromStr for GateType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(GateType::And),
            "XOR" => Ok(GateType::Xor),
            "OR " => Ok(GateType::Or),
            _ => Err("Unkonw gate type".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Equation<'a> {
    Value(bool),
    Gate {
        ty: GateType,
        a: &'a str,
        b: &'a str,
    },
}

impl<'a> Equation<'a> {
    pub fn solve(&self, name: &'a str, equations: &mut HashMap<&'a str, Self>) -> bool {
        match self {
            Equation::Value(v) => *v,
            Equation::Gate { ty, a, b } => {
                let a = equations.get(a).copied().unwrap().solve(a, equations);
                let b = equations.get(b).copied().unwrap().solve(b, equations);
                let result = match ty {
                    GateType::And => a && b,
                    GateType::Or => a || b,
                    GateType::Xor => a ^ b,
                };
                equations.insert(name, Self::Value(result));
                result
            }
        }
    }
}

impl<'a> From<&'a str> for Equation<'a> {
    fn from(s: &'a str) -> Self {
        Equation::Gate {
            ty: GateType::from_str(&s[4..=6]).expect("Failed to find gate type"),
            a: &s[..3],
            b: s[7..].trim(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (variables, gates) = input.split_once("\n\n")?;
    let mut gates: HashMap<&str, Equation> = gates
        .lines()
        .filter_map(|line| line.split_once(" -> "))
        .map(|(equation, result)| (result, Equation::from(equation)))
        .collect();
    gates.extend(
        variables
            .lines()
            .filter_map(|line| line.split_once(": "))
            .map(|(name, value)| (name, Equation::Value(value == "1"))),
    );

    let binding = gates.clone();
    let bits = binding
        .iter()
        .filter(|(key, _v)| key.starts_with("z"))
        .map(|(key, equation)| (key, equation.solve(key, &mut gates)))
        .map(|(key, val)| (key[1..].parse::<usize>().unwrap(), val))
        .fold(0, |acc, (k, b)| acc | (b as u64) << k);
    Some(bits)
}

pub fn part_two(input: &str) -> Option<String> {
    let (variables, gates) = input.split_once("\n\n")?;
    let mut gates: HashMap<&str, Equation> = gates
        .lines()
        .filter_map(|line| line.split_once(" -> "))
        .map(|(equation, result)| (result, Equation::from(equation)))
        .collect();
    gates.extend(
        variables
            .lines()
            .filter_map(|line| line.split_once(": "))
            .map(|(name, value)| (name, Equation::Value(value == "1"))),
    );

    let bits = gates
        .keys()
        .filter(|key| key.starts_with("z"))
        .map(|key| key[1..].parse::<usize>().unwrap())
        .max()
        .unwrap();
    let z_max = format!("z{}", bits);

    let is_input_wire = |a: &str, b: &str| {
        const INPUTS: [&str; 2] = ["x", "y"];
        INPUTS.contains(&&a[..=0]) && INPUTS.contains(&&b[..=0])
    };

    let is_output_wire = |output: &str| output.starts_with("z");

    // https://en.wikipedia.org/wiki/File:Full-adder_logic_diagram.svg?useskin=vector#/media/File:Full-adder_logic_diagram.svg
    let wrong_gates = gates
        .iter()
        .filter(|(output, eq)| {
            let gate @ Equation::Gate { ty, a, b } = eq else {
                return false;
            };
            if is_output_wire(output) && output != &&z_max {
                // if a gate computes an output, it must be computed from XOR, except for last
                // carry bit
                return ty != &GateType::Xor;
            }
            if !is_output_wire(output) && !is_input_wire(a, b) {
                // intermediates gates cannot be Xor gates
                return ty == &GateType::Xor;
            }
            if is_input_wire(a, b) && !(a.ends_with("00") && b.ends_with("00")) {
                // XOR inputs should go to XOR and AND inputs to OR,
                // except for the fist input bits
                let expected_operation = if ty == &GateType::Xor {
                    GateType::Xor
                } else {
                    GateType::Or
                };
                return !gates.iter().any(|(key, gate)| {
                    &key != output
                        && match gate {
                            Equation::Value(_) => false,
                            Equation::Gate { ty, a, b } => {
                                (output == &a || output == &b) && ty == &expected_operation
                            }
                        }
                });
            }
            false
        })
        .map(|(output, eq)| output)
        .sorted()
        .join(",");

    Some(wrong_gates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // The provided test case uses completely different assumptions
        // assert_eq!(result, Some("z00,z01,z02,z05".to_string()));
        assert_eq!(result, Some("z00,z02".to_string()));
    }
}
