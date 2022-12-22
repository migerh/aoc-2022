use std::{
    cmp::{max, min},
    collections::HashMap,
};

use anyhow::{Context, Result};

use crate::utils::ParseError;

type Coords = (isize, isize);

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Tile {
    Void,
    Wall,
    Open,
}

impl Tile {
    fn from_char(c: char) -> Result<Tile> {
        Ok(match c {
            ' ' => Tile::Void,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => Err(ParseError::new("Invalid tile"))?,
        })
    }
}

type Map = HashMap<Coords, Tile>;

#[derive(Debug, Clone)]
pub enum Command {
    Left,
    Right,
    Forward(usize),
}

impl Command {
    fn parse(s: &str) -> Result<Vec<Self>> {
        let mut result = vec![];

        let mut buf = vec![];
        for c in s.chars() {
            if c.is_ascii_digit() {
                buf.push(c);
                continue;
            }

            if !buf.is_empty() {
                let f = buf.into_iter().collect::<String>().parse::<usize>()?;
                result.push(Command::Forward(f));
                buf = vec![];
            }

            if c == 'R' {
                result.push(Command::Right);
                continue;
            }

            if c == 'L' {
                result.push(Command::Left);
                continue;
            }

            panic!("foo");
        }

        if !buf.is_empty() {
            let f = buf.into_iter().collect::<String>().parse::<usize>()?;
            result.push(Command::Forward(f));
        }

        Ok(result)
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Result<(Map, Vec<Command>)> {
    let mut split = input.split("\n\n");

    let map = split.next().context("No map found")?;

    let map = map
        .lines()
        .filter(|s| !s.is_empty())
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| -> Result<(Coords, Tile)> {
                    Ok(((x as isize, y as isize), Tile::from_char(c)?))
                })
        })
        .collect::<Result<Map>>()
        .context("Error while parsing input")?;

    let directions = split.next().context("No directions found")?;
    let directions = Command::parse(directions).context("Could not parse directions")?;

    Ok((map, directions))
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
pub struct State {
    pos: Coords,
    dir: Direction,
}

impl State {
    fn new(pos: Coords, dir: Direction) -> Self {
        State { pos, dir }
    }
}

fn max_coords(map: &Map) -> Coords {
    let mut max_x = 0;
    let mut max_y = 0;

    for c in map.keys() {
        max_x = max(max_x, c.0);
        max_y = max(max_y, c.1);
    }

    (max_x, max_y)
}

fn start(map: &Map) -> Coords {
    let max_c = max_coords(map);

    let (mut min_x, mut min_y) = max_c;
    for (c, t) in map {
        if *t != Tile::Open {
            continue;
        }

        if c.1 > 0 {
            continue;
        }

        min_x = min(min_x, c.0);
        min_y = min(min_y, c.1);
    }

    (min_x, min_y)
}

fn is(map: &Map, pos: &Coords, tile: Tile) -> bool {
    if let Some(t) = map.get(pos) {
        *t == tile
    } else {
        false
    }
}

fn is_wall(map: &Map, pos: &Coords) -> bool {
    is(map, pos, Tile::Wall)
}

fn is_void(map: &Map, pos: &Coords) -> bool {
    !map.contains_key(pos) || is(map, pos, Tile::Void)
}

fn mov(pos: &Coords, delta: &Coords, max_c: &Coords) -> Coords {
    let mut new_pos = (pos.0 + delta.0, pos.1 + delta.1);

    if new_pos.0 < 0 {
        new_pos.0 = max_c.0;
    }

    if new_pos.0 > max_c.0 {
        new_pos.0 = 0;
    }

    if new_pos.1 < 0 {
        new_pos.1 = max_c.1;
    }

    if new_pos.1 > max_c.1 {
        new_pos.1 = 0;
    }

    new_pos
}

fn step(mut state: State, map: &Map, max_c: &Coords) -> State {
    let delta = match state.dir {
        Direction::Up => (0, -1),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
    };

    let mut new_pos = mov(&state.pos, &delta, max_c);

    // keep going if we reach a void
    while is_void(map, &new_pos) {
        new_pos = mov(&new_pos, &delta, max_c);
    }

    // reset if we hit a wall
    if is_wall(map, &new_pos) {
        new_pos = state.pos;
    }

    state.pos = new_pos;

    state
}

fn walk(mut state: State, cmds: &Vec<Command>, map: &Map, max_c: &Coords, step_fn: &dyn Fn(State, &Map, &Coords) -> State) -> State {
    let mut path = vec![state.clone()];
    for cmd in cmds {
        match cmd {
            Command::Forward(f) => {
                for _ in 0..*f {
                    state = step_fn(state, map, max_c);
                    path.push(state.clone());
                }
            }
            Command::Right => {
                state.dir = match state.dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
                path.push(state.clone());
            }
            Command::Left => {
                state.dir = match state.dir {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                };
                path.push(state.clone());
            }
        }
    }

    state
}

#[allow(dead_code)]
fn dir_to_char(dir: &Direction) -> char {
    match dir {
        Direction::Up => '^',
        Direction::Right => '>',
        Direction::Down => 'v',
        Direction::Left => '<',
    }
}

#[allow(dead_code)]
fn tile_to_char(tile: &Tile) -> char {
    match tile {
        Tile::Void => ' ',
        Tile::Wall => '#',
        Tile::Open => '.',
    }
}

#[allow(dead_code)]
fn print_path(map: &Map, path: &[State], max_c: &Coords) {
    for y in 0..=max_c.1 {
        for x in 0..=max_c.0 {
            let m = if let Some(m) = map.get(&(x, y)) {
                m
            } else {
                continue;
            };

            if let Some(p) = path.iter().rev().find(|e| e.pos.0 == x && e.pos.1 == y) {
                print!("{}", dir_to_char(&p.dir));
            } else {
                print!("{}", tile_to_char(m));
            }
        }
        println!();
    }
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &(Map, Vec<Command>)) -> Result<isize> {
    let initial_state = State::new(start(&input.0), Direction::Right);
    let max_c = max_coords(&input.0);

    let destination = walk(initial_state, &input.1, &input.0, &max_c, &step);

    let hash = (destination.pos.1 + 1) * 1000
        + 4 * (destination.pos.0 + 1)
        + match destination.dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };

    Ok(hash)
}

fn get_area(pos: &Coords) -> Option<u8> {
    if pos.0 >= 0 && pos.0 < 50 {
        if pos.1 >= 100 && pos.1 < 150 {
            Some(3)
        } else if pos.1 >= 150 && pos.1 < 200 {
            Some(4)
        } else {
            None
        }
    } else if pos.0 >= 50 && pos.0 < 100 {
        if pos.1 >= 0 && pos.1 < 50 {
            Some(1)
        } else if pos.1 >= 50 && pos.1 < 100 {
            Some(2)
        } else if pos.1 >= 100 && pos.1 < 150 {
            Some(5)
        } else {
            None
        }
    } else if pos.1 >= 0 && pos.1 < 50 {
        Some(6)
    } else {
        None
    }
}

//
// My Cube
//
//  -  1  6 
//  -  2  -
//  3  5  -
//  4  -  -
//

fn teleport(mut state: State) -> State {
    let src_area = get_area(&state.pos).expect("This should never happen");
    use Direction::*;

    match (src_area, state.dir.clone()) {
        (1, Left) => {
            state.dir = Right;
            state.pos = (0, 149 - state.pos.1);
        },
        (1, Up) => {
            state.dir = Right;
            state.pos = (0, 100 + state.pos.0);
        }
        (2, Left) => {
            state.dir = Down;
            state.pos = (state.pos.1 - 50, 100);
        },
        (2, Right) => {
            state.dir = Up;
            state.pos = (state.pos.1 + 50, 49);
        },
        (3, Left) => {
            state.dir = Right;
            state.pos = (50, 149 - state.pos.1);
        },
        (3, Up) => {
            state.dir = Right;
            state.pos = (50, 50 + state.pos.0);
        },
        (4, Left) => {
            state.dir = Down;
            state.pos = (state.pos.1 - 100, 0);
        },
        (4, Right) => {
            state.dir = Up;
            state.pos = (state.pos.1 - 100, 149);
        },
        (4, Down) => {
            state.dir = Down;
            state.pos = (state.pos.0 + 100, 0);
        },
        (5, Down) => {
            state.dir = Left;
            state.pos = (49, state.pos.0 + 100);
        },
        (5, Right) => {
            state.dir = Left;
            state.pos = (149, 149 - state.pos.1);
        },
        (6, Up) => {
            state.dir = Up;
            state.pos = (state.pos.0 - 100, 199);
        },
        (6, Right) => {
            state.dir = Left;
            state.pos = (99, 149 - state.pos.1);
        },
        (6, Down) => {
            state.dir = Left;
            state.pos = (99, state.pos.0 - 50);
        },
        _ => panic!("This should not happen")
    }

    state
}

fn step2(state: State, map: &Map, _max_c: &Coords) -> State {
    use Direction::*;

    let delta = match state.dir {
        Up => (0, -1),
        Right => (1, 0),
        Down => (0, 1),
        Left => (-1, 0),
    };

    let mut new_state = state.clone();
    new_state.pos = (state.pos.0 + delta.0, state.pos.1 + delta.1);

    // teleport to the right tile when we hit a void
    while is_void(map, &new_state.pos) {
        new_state = teleport(state.clone());
    }

    // reset if we hit a wall
    if is_wall(map, &new_state.pos) {
        new_state = state;
    }

    new_state
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &(Map, Vec<Command>)) -> Result<isize> {
    use Direction::*;

    let initial_state = State::new(start(&input.0), Direction::Right);
    let max_c = max_coords(&input.0);

    let destination = walk(initial_state, &input.1, &input.0, &max_c, &step2);

    let hash = (destination.pos.1 + 1) * 1000
        + 4 * (destination.pos.0 + 1)
        + match destination.dir {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        };

    Ok(hash)
}
