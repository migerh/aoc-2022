use std::str::FromStr;

use anyhow::{Context, Error, Result};
use pathfinding::prelude::dijkstra_all;
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

#[derive(Clone, Copy)]
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
}

pub struct State {
    res: Resources,
    rob: Resources,
    time: usize,
}

fn collect_resources(res: &Resources, rob: &Resources) -> Resources {
    let mut new_res = res.to_owned();
    new_res.add(rob);

    new_res
}

fn build_robot(res: &Resources, rob: &Resources, bp: &Blueprint) -> Vec<(Resources, Resources)> {
    let mut result = vec![(*res, *rob)];

    if res.ore >= bp.cost_geode.ore && res.obsidian >= bp.cost_geode.obsidian {
        let mut new_res = *res;
        new_res.ore -= bp.cost_geode.ore;
        new_res.obsidian -= bp.cost_geode.obsidian;

        let mut new_rob = *rob;
        new_rob.geode += 1;
        result.push((new_res, new_rob));
    }

    if res.ore >= bp.cost_obsidian.ore && res.clay >= bp.cost_obsidian.clay {
        let mut new_res = *res;
        new_res.ore -= bp.cost_obsidian.ore;
        new_res.clay -= bp.cost_obsidian.clay;

        let mut new_rob = *rob;
        new_rob.obsidian += 1;
        result.push((new_res, new_rob));
    }

    if res.ore >= bp.cost_clay.ore {
        let mut new_res = *res;
        new_res.ore -= bp.cost_clay.ore;

        let mut new_rob = *rob;
        new_rob.clay += 1;

        result.push((new_res, new_rob));
    }

    if res.ore >= bp.cost_ore.ore {
        let mut new_res = *res;
        new_res.ore -= bp.cost_ore.ore;

        let mut new_rob = *rob;
        new_rob.ore += 1;
        
        result.push((new_res, new_rob));
    }

    result
}

fn get_neighbors(state: &State, res: &Resources, rob: &Resources, bp: &Blueprint) -> Vec<(State, isize)> {
    build_robot(res, rob, bp)
        .into_iter()
        .map(|s| (State { res: s.0, rob: s.1, time: state.time }, -s.0.geode))
        .collect::<Vec<_>>()
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &[Blueprint]) -> Result<usize> {
    for b in input {
        let mut robots = Resources::new_robots();
        let mut resources = Resources::new_resources();

        println!("Blueprint #{}", b.number);
        // for i in 0..24 {
        //     println!("== Minute {} ==", i + 1);
        //     let new_res = collect_resources(&resources, &robots);
        //     let res_diff = build_robot(&resources, &mut robots, b);
        //     resources = new_res;
        //     resources.add(&res_diff);

        //     println!(
        //         "res: ore {}, clay {}, obs {}, geo {}",
        //         resources.ore, resources.clay, resources.obsidian, resources.geode
        //     );
        //     println!(
        //         "rob: ore {}, clay {}, obs {}, geo {}",
        //         robots.ore, robots.clay, robots.obsidian, robots.geode
        //     );
        //     println!();
        // }
        // println!();
        // println!();
    }

    Ok(input.len())
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &[Blueprint]) -> Result<usize> {
    Ok(input.len())
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Blueprint>> {
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
