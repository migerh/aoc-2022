use std::{cmp::max, collections::VecDeque, str::FromStr};

use anyhow::{Context, Error, Result};
use memoize::memoize;
use rayon::prelude::*;
use regex::Regex;

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

    fn is_better(&self, other: &Self) -> bool {
        self.ore >= other.ore - 2 &&
        self.clay >= other.clay - 2 &&
        self.obsidian >= other.obsidian - 2 &&
        self.geode >= other.geode - 2
    }

    fn beyond_limits(&self, limits: &Self) -> bool {
        self.ore >= limits.ore && self.clay >= limits.clay && self.obsidian >= limits.obsidian
    }
}

#[derive(Clone, Copy)]
pub struct State {
    res: Resources,
    robos: Resources,
}

impl State {
    fn is_better(&self, other: &Self) -> bool {
        self.res.is_better(&other.res) && self.robos.is_better(&other.robos)
    }
}

fn find_new_states(
    res: &Resources,
    robos: &Resources,
    bp: &Blueprint,
    limits: &Resources,
) -> Vec<State> {
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

        return new_states;
    }

    // We can already build a new geode robot every minute, no point in building other robots or idling
    // if robos.ore >= bp.cost_geode.ore && robos.obsidian >= bp.cost_geode.obsidian {
    //     return new_states;
    // }

    if res.ore >= bp.cost_obsidian.ore
        && res.clay >= bp.cost_obsidian.clay
        && robos.obsidian < limits.obsidian
    {
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

fn best_case_scenario(resg: isize, robg: isize, time: isize, end: isize) -> isize {
    let mut result = resg;
    for i in time..end {
        result += robg + i;
    }
    result
}

fn search_iterative(initial_state: State, bp: &Blueprint, limits: &Resources, end: isize) -> isize {
    let mut queue: VecDeque<(State, isize)> = vec![(initial_state, 0)]
        .into_iter()
        .collect::<VecDeque<_>>();

    let mut global_max = 0;
    let mut max = vec![0; end as usize + 1];
    let mut best = vec![State { res: Resources::new_resources(), robos: Resources::new_robots() }; end as usize + 1];
    while let Some(q) = queue.pop_back() {
        if max[q.1 as usize] < q.0.res.geode {
            max[q.1 as usize] = q.0.res.geode;
            best[q.1 as usize] = q.0;
        }

        if global_max < q.0.res.geode {
            global_max = q.0.res.geode;
        }

        if q.1 >= end {
            continue;
        }

        let new_states = find_new_states(&q.0.res, &q.0.robos, bp, limits);

        let local_max = new_states
            .iter()
            .map(|s| s.res.geode)
            .max()
            .unwrap_or_default();

        let it = new_states
            .into_iter()
            .filter(|s| s.res.geode >= local_max)
            // .filter(|s| s.is_better(&best[(q.1 + 1) as usize]))
            .filter(|s| s.res.geode + best_case_scenario(s.res.geode, s.robos.geode, q.1 + 1, end) >= global_max);

        for s in it {
            queue.push_back((s, q.1 + 1));
        }

        // queue = queue
        //     .into_iter()
        //     .filter(|s| s.0.res.geode + best_case_scenario(&s.0, s.1 + 1, end) >= max)
        //     .collect::<VecDeque<_>>();
    }

    max[end as usize]
}

fn robot_limits(blueprint: &Blueprint) -> Resources {
    let mut max_robos = Resources::new_resources();

    max_robos.ore = max(
        max(
            max(blueprint.cost_geode.ore, blueprint.cost_obsidian.ore),
            blueprint.cost_clay.ore,
        ),
        blueprint.cost_ore.ore,
    );
    max_robos.clay = blueprint.cost_obsidian.clay;
    max_robos.obsidian = blueprint.cost_geode.obsidian;

    max_robos
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &[Blueprint]) -> Result<isize> {
    let result = input
        .par_iter()
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
    let result = input
        .par_iter()
        .enumerate()
        .take(3)
        .map(|(_, bp)| {
            let limits = robot_limits(bp);
            let res = Resources::new_resources();
            let robos = Resources::new_robots();
            let state = State { res, robos };

            search_iterative(state, bp, &limits, 32)
        })
        .product();

    Ok(result)
}
