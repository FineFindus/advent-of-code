enum Instruction {
    NoOp,
    Add(isize),
}

pub fn incrase_clock_cycle(clock: &mut usize, reg: isize, result_sum: &mut isize) {
    if *clock % 40 == 20 {
        *result_sum += *clock as isize * reg;
    }
    *clock += 1;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut register = 1;
    let mut clock = 1;
    let mut result_sum = 0;

    input
        .lines()
        .filter_map(|line| match line.as_bytes()[0] {
            b'n' => Some(Instruction::NoOp),
            b'a' => Some(Instruction::Add(
                line.split_once(' ').unwrap().1.parse::<isize>().unwrap(),
            )),
            _ => None,
        })
        .for_each(|instruction| match instruction {
            Instruction::NoOp => incrase_clock_cycle(&mut clock, register, &mut result_sum),
            Instruction::Add(value) => {
                incrase_clock_cycle(&mut clock, register, &mut result_sum);
                incrase_clock_cycle(&mut clock, register, &mut result_sum);
                register += value;
            }
        });

    Some(result_sum as u32)
}

pub fn incrase_clock_cycle_screen(clock: &mut usize, reg: isize, screen: &mut [u8]) {
    //check if pixel should be drawn
    let screen_pos = *clock as isize - 1;
    let line_pos = (*clock as isize - 1) % 40;
    if reg == line_pos || reg - 1 == line_pos || reg + 1 == line_pos {
        screen[screen_pos as usize] = b'#';
    }
    *clock += 1;
}

pub fn part_two(input: &str) -> Option<String> {
    let mut register = 1;
    let mut clock = 1;

    let mut screen = vec![b'.'; 40 * 6];

    input
        .lines()
        .filter_map(|line| match line.as_bytes()[0] {
            b'n' => Some(Instruction::NoOp),
            b'a' => Some(Instruction::Add(
                line.split_once(' ').unwrap().1.parse::<isize>().unwrap(),
            )),
            _ => None,
        })
        .for_each(|instruction| match instruction {
            Instruction::NoOp => incrase_clock_cycle_screen(&mut clock, register, &mut screen),
            Instruction::Add(value) => {
                incrase_clock_cycle_screen(&mut clock, register, &mut screen);
                incrase_clock_cycle_screen(&mut clock, register, &mut screen);
                register += value;
            }
        });

    for i in 1..6 {
        screen.insert(40 * i + i - 1, b'\n');
    }
    String::from_utf8(screen).ok()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(String::from("##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....")));
    }
}
