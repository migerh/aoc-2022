use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Error, Result};
use regex::Regex;

#[derive(PartialEq, Eq, Clone)]
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

    for sensor in input {
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

fn check(pos: &Coords, sensors: &[Sensor], limit: usize) -> bool {
    let mut fits = pos.0 >= 0 && pos.0 <= limit as i128 &&
        pos.1 >= 0 && pos.1 <= limit as i128;

    if !fits {
        return false;
    }

    for sensor in sensors {
        let distance = manhattan(&sensor.pos, &sensor.beacon);
        fits = fits && manhattan(&sensor.pos, pos) > distance;

        if !fits {
            return false;
        }
    }

    fits
}

fn walk_border(sensors: &[Sensor], index: usize, limit: usize) -> Option<Coords> {
    if index >= sensors.len() {
        return None;
    }

    let sensor = sensors.get(index)?;
    let distance = manhattan(&sensor.pos, &sensor.beacon);

    let mut pos = (sensor.pos.0 - distance - 1, sensor.pos.1);

    while pos.0 != sensor.pos.0 {
        pos.0 += 1;
        pos.1 += 1;

        if check(&pos, sensors, limit) {
            return Some(pos);
        }
    }

    while pos.1 != sensor.pos.1 {
        pos.0 += 1;
        pos.1 -= 1;

        if check(&pos, sensors, limit) {
            return Some(pos);
        }
    }

    while pos.0 != sensor.pos.0 {
        pos.0 -= 1;
        pos.1 -= 1;

        if check(&pos, sensors, limit) {
            return Some(pos);
        }
    }

    while pos.1 != sensor.pos.1 {
        pos.0 -= 1;
        pos.1 += 1;

        if check(&pos, sensors, limit) {
            return Some(pos);
        }
    }

    None
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[Sensor]) -> Result<i128> {
    let limit = 4_000_000;
    let mut result = None;
    for (i, _) in input.iter().enumerate() {
        if let Some(p) = walk_border(input, i, limit) {
            result = Some(p.0 * (limit as i128) + p.1);
            break;
        }
    }

    result.context("No solution found")
}
