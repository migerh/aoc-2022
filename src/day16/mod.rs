use std::str::FromStr;

use anyhow::{Context, Error, Result};
use pathfinding::prelude::dijkstra;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Valve {
    name: String,
    flow_rate: usize,
    valves: Vec<String>,
}

type Node = (Valve, usize, usize, isize, Vec<String>);

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Valve (?P<valve>.*)? has flow rate=(?P<flow>-?\d+)?; tunnels? leads? to valves? (?P<valves>.*)?$").unwrap();
        }

        let (name, flow_rate, valves) = RE
            .captures(s)
            .and_then(|cap| {
                let name = cap.name("valve").map(|v| v.as_str())?.to_owned();
                let flow_rate = cap
                    .name("flow")
                    .map(|v| v.as_str().parse::<usize>())?
                    .ok()?;
                let valves = cap.name("valves").map(|v| v.as_str())?;

                Some((name, flow_rate, valves))
            })
            .context("Error during parse")?;

        let valves = if valves.contains(',') {
            valves.split(", ").map(|v| v.to_owned()).collect::<Vec<_>>()
        } else {
            vec![valves.to_owned()]
        };

        Ok(Valve {
            name,
            flow_rate,
            valves,
        })
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Vec<Valve>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Valve::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

fn get_neighbors(valves: &[Valve], pos: &Node) -> Vec<(Node, isize)> {
    if pos.1 >= 30 {
        return vec![];
    }

    let all_neighbor_valves = valves
        .iter()
        .filter(|v| pos.0.valves.contains(&v.name))
        .collect::<Vec<_>>();
    
    let unopened = all_neighbor_valves
        .iter()
        .map(|&v| {
            ((v.clone(), pos.1 + 1, pos.2, pos.3, pos.4.clone()), 0)
        })
        .collect::<Vec<_>>();

    let unopened_neighbor_valves = valves
        .iter()
        .filter(|v| pos.0.valves.contains(&v.name) && !pos.4.contains(&v.name))
        .collect::<Vec<_>>();

    let opened = unopened_neighbor_valves
        .iter()
        .map(|&v| {
            let mut opened = pos.4.clone();
            opened.push(v.clone().name);

            (
                (
                    v.clone(),
                    pos.1 + 2,
                    pos.2 + v.flow_rate,
                    pos.3 - (v.flow_rate * (30 - pos.1 - 2)) as isize,
                    opened,
                ),
                -((v.flow_rate * (30 - pos.1 - 2)) as isize),
            )
        })
        .collect::<Vec<_>>();

    vec![opened, unopened]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[Valve]) -> Result<isize> {
    let start = input
        .iter()
        .find(|v| v.name == "AA")
        .context("Cannot find start")?;

    let result = dijkstra(
        &(start.clone(), 0, 0, 0, vec![]),
        |v| get_neighbors(input, v),
        |v| v.1 == 30,
    ).context("Dijkstra failed")?;

    for r in result.0 {
        println!("{:?}", r);
    }
    let total_flow = -result.1;
    Ok(total_flow)
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[Valve]) -> Result<usize> {
    Ok(input.len())
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Valve>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part2(&data)?))
    }
}
