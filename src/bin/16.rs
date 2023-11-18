use std::{str::FromStr, string::ParseError, thread};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Valve {
    name: String,
    flow_rate: u32,
    tunnel: Vec<String>,
    opened: bool,
}

impl Valve {
    pub fn new(name: String, flow_rate: u32, tunnel: Vec<String>) -> Self {
        Self {
            name,
            flow_rate,
            tunnel,
            opened: false,
        }
    }
}

impl FromStr for Valve {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (valve, tunnel) = s.split_once(';').unwrap();
        let valve_name = valve.strip_prefix("Valve ").unwrap().split_at(2).0;
        let flow_rate = valve.split_at(23).1.parse::<u32>().unwrap();

        let tunnel_prefix = if tunnel.contains("tunnels") {
            " tunnels lead to valves "
        } else {
            " tunnel leads to valve "
        };
        let tunnel = tunnel
            .strip_prefix(tunnel_prefix)
            .unwrap()
            .split(',')
            .map(|tunnel| tunnel.trim())
            .map(|tunnel| tunnel.to_owned())
            .collect_vec();

        Ok(Valve::new(valve_name.to_string(), flow_rate, tunnel))
    }
}

pub struct Volcano {
    valves: Vec<Valve>,
    time: u8,
    total_flow: u32,
}

pub fn find_maxium_flow(
    valves: &mut Vec<Valve>,
    mut minute: u8,
    flow: u32,
    current_valve: &str,
) -> u32 {
    if minute == 0 {
        //time's up
        return flow;
    } else {
        minute = minute - 1;
    }
    let valve_pos = valves
        .iter()
        .position(|valve| valve.name == current_valve)
        .unwrap();
    let valve = valves.get_mut(valve_pos).unwrap();
    dbg!(&valve, flow, minute);
    let mut curr_flow = flow;
    //try open valve
    if !valve.opened && valve.flow_rate != 0 {
        valve.opened = true;
        curr_flow += valve.flow_rate * minute.saturating_sub(1) as u32;
        curr_flow = find_maxium_flow(valves, minute.saturating_sub(1), curr_flow, current_valve);
        return curr_flow;
    } else {
        for valve in valve.tunnel.clone() {
            let new_flow = find_maxium_flow(valves, minute, curr_flow, &valve);
            if valves
                .iter()
                .all(|valve| valve.opened || valve.flow_rate == 0)
            {
                curr_flow = new_flow;
            }
        }
    }
    curr_flow = flow;
    dbg!(curr_flow)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut valves = input
        .lines()
        .filter_map(|line| line.parse::<Valve>().ok())
        .inspect(|item| println!("{:?}", item))
        .collect_vec();
    let max_flow = find_maxium_flow(&mut valves, 30, 0, "AA");
    Some(max_flow)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
