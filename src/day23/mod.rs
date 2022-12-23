use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{Context, Result};
use itertools::Itertools;

const DEBUG: bool = false;

type Coords = (isize, isize);

#[derive(Clone, Debug)]
pub struct Elf {
    pos: Coords,
}

impl Elf {
    fn new(pos: Coords) -> Self {
        Elf { pos }
    }
}

#[derive(Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Result<Vec<Elf>> {
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Elf::new((x as isize, y as isize)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>())
}

fn search(dir: &Direction) -> Vec<Coords> {
    use Direction::*;

    match dir {
        North => vec![(0, -1), (-1, -1), (1, -1)],
        South => vec![(0, 1), (-1, 1), (1, 1)],
        West => vec![(-1, 0), (-1, 1), (-1, -1)],
        East => vec![(1, 0), (1, -1), (1, 1)],
    }
}

fn neighborhood8() -> Vec<Coords> {
    vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
}

fn propose_new_positions(elves: &[Elf], order: &VecDeque<Direction>) -> Vec<(usize, Coords)> {
    let occupied = elves.iter().map(|e| e.pos).collect::<HashSet<_>>();
    let search_order = order.iter().map(search).collect::<Vec<_>>();
    let n8 = neighborhood8();

    let mut result = vec![];
    for (i, e) in elves.iter().enumerate() {
        let alone = n8.iter().all(|d| {
            let p = (e.pos.0 + d.0, e.pos.1 + d.1);
            !occupied.contains(&p)
        });
        if alone {
            continue;
        }

        for s in search_order.iter() {
            let can_move = s.iter().all(|delta| {
                let pos = (e.pos.0 + delta.0, e.pos.1 + delta.1);
                !occupied.contains(&pos)
            });

            if can_move {
                let proposed_pos = (e.pos.0 + s[0].0, e.pos.1 + s[0].1);
                result.push((i, proposed_pos));
                break;
            }
        }
    }

    result
}

fn scatter_elves(
    mut elves: Vec<Elf>,
    mut order: VecDeque<Direction>,
    max_iteration: usize,
) -> (usize, Vec<Elf>) {
    let mut it = 0;

    for i in 0..max_iteration {
        let proposals = propose_new_positions(&elves, &order);
        if proposals.is_empty() {
            it = i + 1;
            break;
        }

        // filter duplicates
        let mut counts = HashMap::new();
        for (i, p) in proposals.iter().cloned() {
            counts
                .entry(p)
                .and_modify(|(_, c)| *c += 1)
                .or_insert((i, 1));
        }

        // apply non-duplicate proposals
        for p in proposals.into_iter().filter(|p| {
            if let Some(e) = counts.get(&p.1) {
                e.1 <= 1
            } else {
                true
            }
        }) {
            elves[p.0].pos = p.1;
        }

        // shift order
        order.rotate_left(1);

        if DEBUG {
            println!("== End of Round {} ==", i + 1);
            print(&elves);
        }
    }

    (it, elves)
}

fn empty_ground(elves: Vec<Elf>) -> Option<usize> {
    let minmax_w = elves.iter().map(|e| e.pos.0).minmax().into_option()?;
    let minmax_h = elves.iter().map(|e| e.pos.1).minmax().into_option()?;
    let width = minmax_w.1 - minmax_w.0 + 1;
    let height = minmax_h.1 - minmax_h.0 + 1;

    let area = (width * height) as usize;

    Some(area - elves.len())
}

fn print(elves: &[Elf]) -> Option<()> {
    let occupied = elves.iter().map(|e| e.pos).collect::<HashSet<_>>();
    let minmax_w = elves.iter().map(|e| e.pos.0).minmax().into_option()?;
    let minmax_h = elves.iter().map(|e| e.pos.1).minmax().into_option()?;

    for y in (minmax_h.0 - 1)..minmax_h.1 + 2 {
        for x in (minmax_w.0 - 1)..=minmax_w.1 + 2 {
            let c = if occupied.contains(&(x, y)) { '#' } else { '.' };
            print!("{c}");
        }
        println!();
    }
    println!();

    Some(())
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &[Elf]) -> Result<usize> {
    use Direction::*;

    let elves = input.to_owned();
    let order = vec![North, South, West, East]
        .into_iter()
        .collect::<VecDeque<_>>();

    if DEBUG {
        println!("== Initial State ==");
        print(input);
    }

    let (_, elves) = scatter_elves(elves, order, 10);
    let result = empty_ground(elves).context("Could not determine result")?;

    Ok(result)
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &[Elf]) -> Result<usize> {
    use Direction::*;

    let elves = input.to_owned();
    let order = vec![North, South, West, East]
        .into_iter()
        .collect::<VecDeque<_>>();

    if DEBUG {
        println!("== Initial State ==");
        print(input);
    }

    let (it, _) = scatter_elves(elves, order, 1_000_000);

    Ok(it)
}
