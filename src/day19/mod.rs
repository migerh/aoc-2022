use std::str::FromStr;

use anyhow::{Context, Error, Result};
use regex::Regex;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Cost {
    ore: isize,
    clay: isize,
    obsidian: isize,
}

impl Cost {
    fn new_ore(ore: isize) -> Self {
        Cost {
            ore,
            clay: 0,
            obsidian: 0,
        }
    }

    fn new_clay(ore: isize) -> Self {
        Cost {
            ore,
            clay: 0,
            obsidian: 0,
        }
    }

    fn new_obsidian(ore: isize, clay: isize) -> Self {
        Cost {
            ore,
            clay,
            obsidian: 0,
        }
    }

    fn new_geode(ore: isize, obsidian: isize) -> Self {
        Cost {
            ore,
            clay: 0,
            obsidian,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blueprint {
    number: isize,
    cost_ore: Cost,
    cost_clay: Cost,
    cost_obsidian: Cost,
    cost_geode: Cost,
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Blueprint (?P<number>\d+)?: Each ore robot costs (?P<ore_ore>\d+)? ore. Each clay robot costs (?P<clay_ore>\d+)? ore. Each obsidian robot costs (?P<obs_ore>\d+)? ore and (?P<obs_clay>\d+)? clay. Each geode robot costs (?P<geode_ore>\d+)? ore and (?P<geode_obs>\d+)? obsidian.$").unwrap();
        }

        let (number, ore_ore, clay_ore, obs_ore, obs_clay, geode_ore, geode_obs) = RE
            .captures(s)
            .and_then(|cap| {
                let number = cap
                    .name("number")
                    .map(|v| v.as_str().parse::<isize>())?
                    .ok()?;

                let ore_ore = cap
                    .name("ore_ore")
                    .map(|v| v.as_str().parse::<isize>())?
                    .ok()?;

                let clay_ore = cap
                    .name("clay_ore")
                    .map(|v| v.as_str().parse::<isize>())?
                    .ok()?;

                let obs_ore = cap
                    .name("obs_ore")
                    .map(|v| v.as_str().parse::<isize>())?
                    .ok()?;

                let obs_clay = cap
                    .name("obs_clay")
                    .map(|v| v.as_str().parse::<isize>())?
                    .ok()?;

                let geode_ore = cap
                    .name("geode_ore")
                    .map(|v| v.as_str().parse::<isize>())?
                    .ok()?;

                let geode_obs = cap
                    .name("geode_obs")
                    .map(|v| v.as_str().parse::<isize>())?
                    .ok()?;

                Some((
                    number, ore_ore, clay_ore, obs_ore, obs_clay, geode_ore, geode_obs,
                ))
            })
            .context("Error during parse")?;

        Ok(Blueprint {
            number,
            cost_ore: Cost::new_ore(ore_ore),
            cost_clay: Cost::new_clay(clay_ore),
            cost_obsidian: Cost::new_obsidian(obs_ore, obs_clay),
            cost_geode: Cost::new_geode(geode_ore, geode_obs),
        })
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Result<Vec<Blueprint>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Blueprint::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Resources {
    ore: isize,
    clay: isize,
    obsidian: isize,
    geode: isize,
}

impl Resources {
    fn new_robots() -> Self {
        Resources {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn new_resources() -> Self {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn add(&mut self, other: &Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }

    fn beyond_limits(&self, limits: &Self) -> bool {
        self.ore >= limits.ore && self.clay >= limits.clay && self.obsidian >= limits.obsidian
    }
}

pub struct State {
    res: Resources,
    robos: Resources,
}

fn find_new_states(res: &Resources, robos: &Resources, bp: &Blueprint, limits: &Resources) -> Vec<State> {
    let mut new_states = vec![];

    if res.ore >= bp.cost_geode.ore && res.obsidian >= bp.cost_geode.obsidian {
        let mut new_res = *res;
        new_res.ore -= bp.cost_geode.ore;
        new_res.obsidian -= bp.cost_geode.obsidian;
        new_res.add(robos);

        let mut new_robos = *robos;
        new_robos.geode += 1;

        new_states.push(State {
            res: new_res,
            robos: new_robos,
        });
    }

    if res.ore >= bp.cost_obsidian.ore && res.clay >= bp.cost_obsidian.clay && robos.obsidian < limits.obsidian {
        let mut new_res = *res;
        new_res.ore -= bp.cost_obsidian.ore;
        new_res.clay -= bp.cost_obsidian.clay;
        new_res.add(robos);

        let mut new_robos = *robos;
        new_robos.obsidian += 1;

        new_states.push(State {
            res: new_res,
            robos: new_robos,
        });
    }

    if res.ore >= bp.cost_clay.ore && robos.clay < limits.clay {
        let mut new_res = *res;
        new_res.ore -= bp.cost_clay.ore;
        new_res.add(robos);

        let mut new_robos = *robos;
        new_robos.clay += 1;

        new_states.push(State {
            res: new_res,
            robos: new_robos,
        });
    }

    if res.ore >= bp.cost_ore.ore && robos.ore < limits.ore {
        let mut new_res = *res;
        new_res.ore -= bp.cost_ore.ore;
        new_res.add(robos);

        let mut new_robos = *robos;
        new_robos.ore += 1;

        new_states.push(State {
            res: new_res,
            robos: new_robos,
        });
    }

    let mut new_res = *res;
    new_res.add(robos);
    new_states.push(State {
        res: new_res,
        robos: *robos,
    });

    new_states
}

fn search_recursive(state: &State, bp: &Blueprint, time: usize, limits: &Resources) -> isize {
    if time == 24 {
        return state.res.geode;
    }

    find_new_states(&state.res, &state.robos, bp, limits)
        .into_iter()
        .map(|s| search_recursive(&s, bp, time + 1, limits))
        .max()
        .unwrap_or_default()
}

fn search_iterative(initial_state: State, bp: &Blueprint, limits: &Resources, end: isize) -> isize {
    let mut queue: Vec<(State, isize)> = vec![(initial_state, 0)];

    let mut max: isize = 0;
    while let Some(q) = queue.pop() {
        if q.1 >= end {
            if max <= q.0.res.geode {
                max = q.0.res.geode;
            }
            continue;
        }
        let new_states = find_new_states(&q.0.res, &q.0.robos, bp, limits);

        let max = new_states
            .iter()
            .map(|s| s.res.geode)
            .max()
            .unwrap_or_default();

        for s in new_states.into_iter().filter(|s| s.res.geode >= max) {
            queue.push((s, q.1 + 1));
        }
    }

    max
}

fn robot_limits(blueprint: &Blueprint) -> Resources {
    let mut max_robos = Resources::new_resources();

    max_robos.ore = blueprint.cost_geode.ore + 2;
    max_robos.clay = blueprint.cost_obsidian.clay + 2;
    max_robos.obsidian = blueprint.cost_geode.obsidian + 2;

    max_robos
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &[Blueprint]) -> Result<isize> {
    // guesses: 1498 too low
    //          1579 too low
    let result = input.par_iter()
        .enumerate()
        .map(|(i, bp)| {
            let limits = robot_limits(bp);
            let res = Resources::new_resources();
            let robos = Resources::new_robots();
            let state = State { res, robos };

            ((i as isize) + 1) * search_iterative(state, bp, &limits, 24)
        })
        .sum();

    Ok(result)
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &[Blueprint]) -> Result<isize> {
    let result = input.par_iter()
        .enumerate()
        .take(3)
        .map(|(i, bp)| {
            let limits = robot_limits(bp);
            let res = Resources::new_resources();
            let robos = Resources::new_robots();
            let state = State { res, robos };

            search_iterative(state, bp, &limits, 32)
        })
        .product();

    Ok(result)
}
