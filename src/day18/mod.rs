use std::{collections::HashMap, isize::MAX, isize::MIN};

use anyhow::{Context, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Position {
    Inside,
    Outside,
    Droplet,
}
type Coords = (isize, isize, isize);
type Droplet = HashMap<Coords, Position>;
const NEIGHBORS6: [Coords; 6] = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
];

fn parse_coords(s: &str) -> Option<Coords> {
    let mut split = s.split(',');

    let x = split.next()?.parse::<isize>().ok()?;
    let y = split.next()?.parse::<isize>().ok()?;
    let z = split.next()?.parse::<isize>().ok()?;

    Some((x, y, z))
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Result<Droplet> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| -> Result<(Coords, Position)> {
            Ok((
                parse_coords(s).context("Parsing failed")?,
                Position::Droplet,
            ))
        })
        .collect::<Result<HashMap<_, _>>>()
        .context("Error while parsing input")
}

fn count_neighbors(pos: &Coords, droplet: &Droplet, cmp: Position) -> usize {
    let mut count = 0;
    for n in NEIGHBORS6 {
        if let Some(d) = droplet.get(&(pos.0 + n.0, pos.1 + n.1, pos.2 + n.2)) {
            count += usize::from(*d == cmp);
        }
    }
    count
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &Droplet) -> Result<usize> {
    let mut result = 0;
    input.iter().for_each(|(coords, _)| {
        result += 6 - count_neighbors(coords, input, Position::Droplet);
    });
    Ok(result)
}

fn dim(droplet: &Droplet) -> (Coords, Coords) {
    let mut min = (MAX, MAX, MAX);
    let mut max = (MIN, MIN, MIN);

    droplet.iter().for_each(|(coords, _)| {
        if coords.0 < min.0 {
            min.0 = coords.0;
        }
        if coords.0 > max.0 {
            max.0 = coords.0;
        }
        if coords.1 < min.1 {
            min.1 = coords.1;
        }
        if coords.1 > max.1 {
            max.1 = coords.1;
        }
        if coords.2 < min.2 {
            min.2 = coords.2;
        }
        if coords.2 > max.2 {
            max.2 = coords.2;
        }
    });

    (min, max)
}

fn connected_to_outside(droplet: &Droplet, pos: &Coords) -> bool {
    for n in NEIGHBORS6 {
        if let Some(d) = droplet.get(&(pos.0 + n.0, pos.1 + n.1, pos.2 + n.2)) {
            if *d == Position::Outside {
                return true;
            }
        }
    }
    false
}

fn neighbors(size: &(Coords, Coords), pos: &Coords, bs: isize) -> Vec<Coords> {
    let (min, max) = size;

    NEIGHBORS6.iter()
        .map(|n| (pos.0 + n.0, pos.1 + n.1, pos.2 + n.2))
        .filter(|p| {
            p.0 >= min.0 - bs
                && p.1 >= min.1 - bs
                && p.2 >= min.2 - bs
                && p.0 <= max.0 + bs
                && p.1 <= max.1 + bs
                && p.2 <= max.2 + bs
        })
        .collect::<Vec<_>>()
}

fn flood_fill_outside(droplet: &mut Droplet, size: &(Coords, Coords)) {
    let bs = 3;
    let (min, _) = size;

    let start = (min.0 - bs, min.1 - bs, min.2 - bs);
    droplet
        .entry(start)
        .and_modify(|v| *v = Position::Outside)
        .or_insert(Position::Outside);
    let mut queue = neighbors(size, &start, bs);

    while let Some(p) = queue.pop() {
        if let Some(d) = droplet.get(&p) {
            if *d == Position::Outside {
                continue;
            }
            if *d == Position::Droplet {
                continue;
            }
        }

        if connected_to_outside(droplet, &p) {
            droplet
                .entry(p)
                .and_modify(|v| *v = Position::Outside)
                .or_insert(Position::Outside);
            queue.append(&mut neighbors(size, &p, bs));
        }
    }
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Droplet) -> Result<usize> {
    let mut droplet = input.clone();
    let size = dim(input);
    flood_fill_outside(&mut droplet, &size);

    Ok(input
        .iter()
        .map(|(coords, _)| count_neighbors(coords, &droplet, Position::Outside))
        .sum())
}
