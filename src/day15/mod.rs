use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Error, Result};
use regex::Regex;

#[derive(PartialEq, Eq)]
enum State {
    Beacon,
    Sensor,
    Nope,
}

type Coords = (i128, i128);
type Map = HashMap<Coords, State>;

#[derive(Debug)]
pub struct Sensor {
    pos: Coords,
    beacon: Coords,
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Sensor at x=(?P<sx>-?\d+)?, y=(?P<sy>-?\d+)?: closest beacon is at x=(?P<bx>-?\d+)?, y=(?P<by>-?\d+)?$").unwrap();
        }

        let (sx, sy, bx, by) = RE
            .captures(s)
            .and_then(|cap| {
                let sx = cap.name("sx").map(|v| v.as_str().parse::<i128>())?.ok()?;

                let sy = cap.name("sy").map(|v| v.as_str().parse::<i128>())?.ok()?;

                let bx = cap.name("bx").map(|v| v.as_str().parse::<i128>())?.ok()?;

                let by = cap.name("by").map(|v| v.as_str().parse::<i128>())?.ok()?;

                Some((sx, sy, bx, by))
            })
            .context("Error during parse")?;

        Ok(Sensor {
            pos: (sx, sy),
            beacon: (bx, by),
        })
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Result<Vec<Sensor>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Sensor::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

fn manhattan(a: &Coords, b: &Coords) -> i128 {
    (b.1 - a.1).abs() + (b.0 - a.0).abs()
}

fn mark_sensor(sensor: &Sensor, map: &mut Map, line: i128) {
    let distance = manhattan(&sensor.pos, &sensor.beacon);
    println!("distance {distance}");

    let pos = sensor.pos;
    map.entry(pos)
        .and_modify(|v| *v = State::Sensor)
        .or_insert(State::Sensor);
    for y in -distance..=distance {
        let rest = distance - y.abs();
        if pos.1 + y != line {
            continue;
        }

        for x in -rest..=rest{
            let new_pos = (pos.0 + x, pos.1 + y);
            if manhattan(&pos, &new_pos) <= distance  && new_pos.1 == line {
                map.entry(new_pos)
                    .and_modify(|v| *v = State::Nope)
                    .or_insert(State::Nope);
            }
        }
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[Sensor]) -> Result<usize> {
    let mut map = Map::new();
    let line = 2_000_000;
    // let line = 10;

    for sensor in input {
        println!("Sensor at ({}, {})", sensor.pos.0, sensor.pos.1);
        mark_sensor(sensor, &mut map, line);
    }

    for sensor in input {
        map.entry(sensor.pos)
            .and_modify(|v| *v = State::Sensor)
            .or_insert(State::Sensor);
        map.entry(sensor.beacon)
            .and_modify(|v| *v = State::Beacon)
            .or_insert(State::Beacon);
    }

    let result = map
        .into_iter()
        .filter(|((_, y), _)| *y == line)
        .filter(|(_, value)| *value == State::Nope)
        .count();

    Ok(result)
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[Sensor]) -> Result<usize> {
    Ok(input.len())
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Sensor>> {
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
