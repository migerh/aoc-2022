use std::str::FromStr;

use anyhow::{Context, Error, Result};
use num::integer::lcm;
use pathfinding::prelude::dijkstra;

use crate::utils::ParseError;

const DEBUG: bool = false;

type Coords = (isize, isize);

#[derive(Debug, Clone)]
pub struct Blizzard {
    pos: Coords,
    dir: char,
}

impl Blizzard {
    fn new(x: isize, y: isize, dir: char) -> Self {
        Blizzard { pos: (x, y), dir }
    }

    fn mov(mut self, width: isize, height: isize) -> Self {
        self.pos = match self.dir {
            '>' => ((self.pos.0 + 1) % width, self.pos.1),
            '<' => ((self.pos.0 - 1).rem_euclid(width), self.pos.1),
            '^' => (self.pos.0, (self.pos.1 - 1).rem_euclid(height)),
            'v' => (self.pos.0, (self.pos.1 + 1) % height),
            _ => self.pos,
        };

        self
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    width: isize,
    height: isize,
    blizzards: Vec<Blizzard>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().last().context("Input is empty?")?.chars().count() as isize;

        if !s.lines().all(|l| l.chars().count() == width as usize) {
            return Err(ParseError::new("Input is not a rectangle"))?;
        }

        let height = s.lines().count() as isize;
        let blizzards = s
            .lines()
            .enumerate()
            .flat_map(move |(y, v)| {
                v.chars().enumerate().filter_map(move |(x, c)| {
                    if "<>^v".contains(c) {
                        Some(Blizzard::new(x as isize - 1, y as isize - 1, c))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        Ok(Map {
            width: width - 2,
            height: height - 2,
            blizzards,
        })
    }
}

impl Map {
    fn get(&self, x: isize, y: isize) -> Option<char> {
        let count = self
            .blizzards
            .iter()
            .filter(|b| b.pos.0 == x && b.pos.1 == y)
            .count();

        if count == 1 {
            self.blizzards
                .iter()
                .find(|b| b.pos.0 == x && b.pos.1 == y)
                .map(|b| b.dir)
        } else if count > 0 {
            count.to_string().chars().last()
        } else {
            None
        }
    }

    fn print(&self) {
        for _ in 0..self.width + 2 {
            print!("#");
        }
        println!();

        for y in 0..self.height {
            print!("#");
            for x in 0..self.width {
                let c = if let Some(b) = self.get(x, y) { b } else { '.' };
                print!("{c}");
            }
            println!("#");
        }

        for _ in 0..self.width + 2 {
            print!("#");
        }

        println!();
        println!();
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Result<Map> {
    Map::from_str(input)
}

fn simulate(map: Map, iterations: isize) -> Vec<Map> {
    let mut result = vec![map.clone()];

    if DEBUG {
        println!("Initial state:");
        map.print();
    }
    let mut blizzards = map.blizzards;
    for i in 0..(iterations - 1) {
        blizzards = blizzards
            .into_iter()
            .map(|b| b.mov(map.width, map.height))
            .collect::<Vec<_>>();

        let new_map = Map {
            blizzards: blizzards.clone(),
            ..map
        };

        if DEBUG {
            println!("Minute {}, move ...:", i + 1);
            new_map.print();
        }
        result.push(new_map);
    }

    result
}

fn next_step(state: &(Coords, usize), map: &[Map]) -> Vec<((Coords, usize), usize)> {
    let (pos, dist) = state;
    let map = &map[(*dist + 1) % map.len()];

    let possible_neighbors = vec![(0, 0), (0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut result = vec![];
    for p in possible_neighbors {
        let new_pos = (pos.0 + p.0, pos.1 + p.1);

        if new_pos.0 == map.width - 1 && new_pos.1 == map.height {
            result.push(((new_pos, dist + 1), 1));
            continue;
        }

        if new_pos.0 == 0 && new_pos.1 == -1 {
            result.push(((new_pos, dist + 1), 1));
            continue;
        }

        // running into a wall
        if new_pos.0 < 0 || new_pos.0 >= map.width {
            continue;
        }

        // running into a wall
        if new_pos.1 < 0 || new_pos.1 >= map.height {
            continue;
        }

        let has_blizzard = map.get(new_pos.0, new_pos.1).is_some();
        if !has_blizzard {
            result.push(((new_pos, dist + 1), 1));
        }
    }

    // we're at the finish line
    result
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Map) -> Result<usize> {
    let number_of_simulations = lcm(input.width, input.height);

    let blizzards = simulate(input.to_owned(), number_of_simulations);
    let start = (0, -1);
    let finish = (input.width - 1, input.height);

    let distance = dijkstra(
        &(start, 0),
        |p| next_step(p, &blizzards),
        |(p, _)| p.0 == finish.0 && p.1 == finish.1,
    )
    .context("Dijkstra failed")?;

    if DEBUG {
        println!("{:?}", distance);

        for step in distance.0 {
            println!("{:?}", step);
            let map = &blizzards[step.1 % blizzards.len()];

            let pos = step.0;
            let has_blizzard = map.get(pos.0, pos.1).is_some();
            if has_blizzard {
                println!("oops: {:?}", step);
                panic!("Found a blizzard!");
            }
        }
    }

    Ok(distance.1)
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &Map) -> Result<usize> {
    let number_of_simulations = lcm(input.width, input.height);

    let blizzards = simulate(input.to_owned(), number_of_simulations);
    let start = (0, -1);
    let finish = (input.width - 1, input.height);

    // there
    let (_, distance_there) = dijkstra(
        &(start, 0),
        |p| next_step(p, &blizzards),
        |(p, _)| p.0 == finish.0 && p.1 == finish.1,
    )
    .context("Dijkstra #1 failed")?;

    // and back again
    let (_, distance_back) = dijkstra(
        &(finish, distance_there),
        |p| next_step(p, &blizzards),
        |(p, _)| p.0 == start.0 && p.1 == start.1,
    )
    .context("Dijkstra #2 failed")?;

    // and there again
    let (_, distance_thereagain) = dijkstra(
        &(start, distance_back + distance_there),
        |p| next_step(p, &blizzards),
        |(p, _)| p.0 == finish.0 && p.1 == finish.1,
    )
    .context("Dijkstra #2 failed")?;

    Ok(distance_there + distance_back + distance_thereagain)
}
