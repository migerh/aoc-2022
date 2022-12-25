mod valve;

use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Context, Result};
use indicatif::ProgressBar;
use pathfinding::prelude::dijkstra;
use rayon::prelude::*;

use self::valve::Valve;

type DistanceMap<'a> = HashMap<&'a Valve, Vec<(&'a Valve, usize)>>;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Vec<Valve>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Valve::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

fn resolve_valve<'a>(valves: &'a [Valve], name: &String) -> Option<&'a Valve> {
    valves.iter().find(|v| &v.name == name)
}

fn get_neighbors<'a>(valves: &'a [Valve], valve: &Valve) -> Vec<(&'a Valve, usize)> {
    valve
        .valves
        .iter()
        .filter_map(|v| Some((resolve_valve(valves, v)?, 1)))
        .collect::<Vec<_>>()
}

fn pre_calc_distances(valves: &[Valve]) -> DistanceMap {
    let names = valves.iter().map(|v| v.name.to_owned()).collect::<Vec<_>>();
    let mut map = HashMap::new();

    for start in names.iter() {
        let mut distances = vec![];
        let start_valve = if let Some(node) = resolve_valve(valves, start) {
            node
        } else {
            continue;
        };

        for end in names.iter().filter(|&v| v != start) {
            let end_valve = if let Some(node) = resolve_valve(valves, end) {
                node
            } else {
                continue;
            };

            if let Some(distance) = dijkstra(
                &start_valve,
                |valve| get_neighbors(valves, valve),
                |valve| &valve.name == end,
            ) {
                distances.push((end_valve, distance.1));
            }
        }

        distances.sort_by(|a, b| b.0.flow_rate.cmp(&a.0.flow_rate));
        let distances = distances
            .into_iter()
            .filter(|&(v, _)| v.flow_rate > 0)
            .collect::<Vec<_>>();
        map.entry(start_valve).or_insert(distances);
    }

    map
}

#[derive(Debug, Clone)]
pub struct State<'a> {
    opened: HashSet<&'a Valve>,
    visited: HashSet<&'a Valve>,
    time: usize,
    location: &'a Valve,
    total_flow: usize,
}

impl<'a> State<'a> {
    fn new(start: &'a Valve) -> Self {
        State {
            opened: HashSet::new(),
            visited: HashSet::new(),
            time: 0,
            location: start,
            total_flow: 0,
        }
    }

    fn flow_during_time(&self, time: usize) -> usize {
        self.opened.iter().map(|v| v.flow_rate).sum::<usize>() * time
    }

    fn next(
        &self,
        new_location: &'a Valve,
        distance: usize,
        open: bool,
        limit: usize,
    ) -> Option<Self> {
        if self.opened.contains(&new_location) && open {
            return None;
        }

        let time_consumed = distance + usize::from(open);
        if self.time + time_consumed > limit {
            return None;
        }

        let flow = self.flow_during_time(time_consumed);

        let new_opened_valves = if open {
            let mut new_opened = self.opened.clone();
            new_opened.insert(new_location);
            new_opened
        } else {
            self.opened.clone()
        };

        let mut new_visited = self.visited.clone();
        new_visited.insert(new_location);

        Some(State::<'a> {
            opened: new_opened_valves,
            visited: new_visited,
            time: self.time + time_consumed,
            location: new_location,
            total_flow: self.total_flow + flow,
        })
    }

    fn wait(&self, time: usize) -> Self {
        let mut result = self.clone();
        result.time += time;
        result.total_flow += self.flow_during_time(time);

        result
    }

    fn follow_up_states(&self, map: &'a DistanceMap, limit: usize) -> Vec<Self> {
        let next = if let Some(n) = map.get(self.location) {
            n
        } else {
            return vec![];
        };

        if self.opened.len() > next.len() {
            return vec![self.wait(limit - self.time)];
        }

        let opened_iter = next
            .iter()
            .filter(|&(v, _)| !self.opened.contains(v))
            .filter(|&(v, _)| !self.visited.contains(v))
            .filter_map(|&(v, d)| self.next(v, d, true, limit));

        let mut result = opened_iter.collect::<Vec<_>>();
        result.push(self.wait(1));
        result
    }
}

fn total_flow<'a>(valves: &'a [Valve], map: &'a DistanceMap, limit: usize, opened: Option<HashSet<&'a Valve>>) -> Option<(usize, Vec<State<'a>>)> {
    let start = valves.iter().find(|v| &v.name == "AA")?;

    let mut state = State::new(start);
    if let Some(opened) = opened {
        state.opened = opened;
    }
    let mut queue = vec![state.clone()];
    let mut max_flow = 0;

    let largest_flow = valves.iter().map(|v| v.flow_rate).sum::<usize>();
    let mut states = Vec::with_capacity(23310000);
    states.push(state);

    while let Some(q) = queue.pop() {
        max_flow = max(max_flow, q.total_flow);

        if q.time >= limit {
            continue;
        }

        let best_case_remaining_flow = q.total_flow + (limit - q.time) * largest_flow;
        if limit > 26 && best_case_remaining_flow < max_flow {
            continue;
        }

        let mut follow_up_states = q.follow_up_states(map, limit);

        states.append(&mut follow_up_states.clone());
        queue.append(&mut follow_up_states);
    }

    Some((max_flow, states))
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[Valve]) -> Result<usize> {
    let limit = 30;
    let map = pre_calc_distances(input);
    let (result, _) = total_flow(input, &map, limit, None).context("Could not calculate")?;

    Ok(result)
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[Valve]) -> Result<usize> {
    let limit = 26;
    let map = pre_calc_distances(input);
    let (_, mut states) = total_flow(input, &map, limit, None).context("Could not calculate")?;

    states.sort_by(|a, b| b.total_flow.cmp(&a.total_flow));

    let valves_to_open = input.iter().filter(|v| v.flow_rate > 0).count();
    let pb = ProgressBar::new(states.len() as u64);
    let max_flow = states.par_iter().map(|state_a| {
        pb.inc(1);
        let mut max_flow = 0;
        for state_b in states.iter() {
            if state_a.opened.len() + state_b.opened.len() > valves_to_open {
                continue;
            }

            if state_a.opened.intersection(&state_b.opened).count() > 0 {
                continue;
            }

            max_flow = max(max_flow, state_a.total_flow + state_b.total_flow);

            // we have a sorted list, it's not going to get any better
            break;
        }

        max_flow
    })
    .max().context("Could not determine max flow")?;
    pb.finish_and_clear();

    Ok(max_flow)
}
