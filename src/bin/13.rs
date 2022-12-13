use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Item(usize),
    List(Vec<Packet>),
}

fn parse(s: &str) -> Packet {
    //copied from  https://topaz.github.io/paste/#XQAAAQBeCAAAAAAAAAA6nMjJFC0FFjnljfK8JmZ0VKsVNYQl92XsRT44yzp5r7p7YUv4/DfhI1Q1IVnKdrcnPC+Pitxun7R5F+iAHaEQvxItKdGWUtMe3KU0cET7hXmkaTSpiTZfEJuSSY4YC6ufH4sR6qrepq9FUinK9wqyUCoN/rgfclSy9SjxOibvJ+xwvNuVqSjIoe1B1MVySBvr2ruL/35TJC7LWfwlIkJwaVGsM56Utsn091QbZjn0oac6gXI5DHhVKFrrRp8GasMHFLflGGtqss41kEBI99ihClZjmLaUVixLu9HJR012W+crEELVx71NwM66787Hr8FPVykhARUzTxFcdrRAEDWRh2dsKENhdfzLoao+KG9nuQ3u8plXs75vhdbowFFTuVJ7+VqUMnR+sjMRkspEXeEG80iUoNDgL3ZYy/pwx0JEnKmcqhZs8AGnEyB/VFQImBqGZDal3uIgHgOnYNYREk7BdVbOnlOZ4vvOdMxxFwZdDV980bOsT1OiDl+tWAT7KaMPIg2dua+WYIVgQdpjiColEK3b1UNrKuponOpBPxy1JeLTUsCrcVMzQzNBq7YBcsIHBUvJcyuPO1mwUjizQncJklKHV6ZsDaO4h2x/IpktTn0E3zBt8sBk63XP6JwTROLyDjDi06ggOh3XXzBm/eDq58gLFOhO47RsEO9gSQXM+gOpdE5CI6jjHZ3o8VDraI0hooNDrdDGmL+nKyp9/Ge0E6gkVHmgxrCPOKhsZjLxCsI2Au6YJHvWqlFFeHP6FmvTaDKKV/KzNY2fY7Kj1ExtENn8LX+mZO6ArvdJ6C5CjlQRpdzpKzRBSFau79vRApwc83UML7s7UCcmVbZs8GLayEdiFpR3/ucwgh5RZVMDCZ4ULCvQg31tpxu3sby9RDzEx/xd6KM99n/rHHYIYANF6Hy0fcDD2foeoG2B/YF0w09FNWTFNwGuREZsB3GoozIlsyYAv1EcZ3rKHtoLK7lkroO5wQi/3949no+ZktISu30JNT/emD9jMwl1YEs04V3iT+llbCSK08lpluDrJ4VXR/17iwsRpGPbAL1vsMdEKqzq/35wZSyFj2A4gGj3FIq8eHCjx/ZWB5XLNcIjwWGmVxpmCPn9Oi+k1xToCygRGYTnD+Fcx5e5qPKGwvvVEhs=
    //because I gave up after multiple hours of frustration
    if &s[0..1] == "[" {
        let mut stack: i32 = 0;
        Packet::List(
            s[1..s.len() - 1]
                .split(|c| {
                    if c == '[' {
                        stack += 1
                    } else if c == ']' {
                        stack -= 1
                    }
                    c == ',' && stack == 0
                })
                .filter_map(|s| (!s.is_empty()).then(|| parse(s)))
                .collect(),
        )
    } else {
        Packet::Item(s.parse().unwrap())
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(l), Self::List(r)) => l.cmp(r),
            (Self::List(l), &Self::Item(r)) => l.cmp(&vec![Self::Item(r)]),
            (Self::Item(l), Self::Item(r)) => l.cmp(r),
            (&Self::Item(l), Self::List(r)) => vec![Self::Item(l)].cmp(r),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .filter_map(|l| l.split_once('\n'))
        .map(|(left, right)| (parse(left.trim()), parse(right.trim())))
        .enumerate()
        .filter_map(|(i, (first, second))| (first <= second).then_some(i + 1))
        .map(|index| index as u32)
        .sum1::<u32>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map( parse)
        .collect_vec();

    let divider_1 = parse("[[2]]");
    let divider_2 = parse("[[6]]");

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    
    packets.sort_unstable();

    Some(
        packets
            .into_iter()
            .positions(|packet| packet == divider_1 || packet == divider_2)
            .map(|idx| idx + 1)
            .product::<usize>() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
