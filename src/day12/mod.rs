use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

use anyhow::{Context, Result};

type Coords = (isize, isize);
type Map = HashMap<Coords, char>;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<Map> {
    Ok(input
        .lines()
        .filter(|s| !s.trim().is_empty())
        .enumerate()
        .flat_map(move |(y, l)| {
            l.trim().chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect::<HashMap<_, _>>())
}

fn find_start(map: &Map) -> Option<Coords> {
    map.iter()
        .find(|&(_, &height)| height == 'S')
        .map(|(&pos, _)| pos)
}

fn get_elevation(e: char) -> isize {
    match e {
        'S' => 1,
        'E' => 26,
        'a'..='z' => (e as isize) - 96,
        // make this one unreachable for any point on our map
        _ => 500,
    }
}

fn successors(map: &Map, pos: &Coords) -> Vec<(Coords, usize)> {
    let possible_neighbors = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];

    let mut successors = vec![];
    if let Some(&entry) = map.get(pos) {
        let level = get_elevation(entry);

        for pn in possible_neighbors.iter() {
            let p = (pos.0 + pn.0, pos.1 + pn.1);
            if let Some(&n) = map.get(&p) {
                let l = get_elevation(n);
                if l - level <= 1 {
                    successors.push((p, 1));
                }
            }
        }
    }

    successors
}

fn shortest_path(map: &Map, start: Coords) -> Option<usize> {
    let result = dijkstra(
        &start,
        |pos| successors(map, pos),
        |pos| if let Some(&level) = map.get(pos) {
            level == 'E'
        } else {
            false
        }
    )?;

    Some(result.1)
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Map) -> Result<usize> {
    let start = find_start(input).context("No start found")?;
    let result = shortest_path(input, start).context("Dijkstra failed")?;

    Ok(result)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Map) -> Result<usize> {
    let mut results = input
        .iter()
        .filter(|&(_, &c)| c == 'a' || c == 'S')
        .filter_map(|(s, _)| Some((*s, shortest_path(input, *s)?)))
        .collect::<Vec<_>>();

    results.sort_by(|a, b| a.1.cmp(&b.1));
    let shortest_start = results.first().context("Dijkstra failed")?.1;

    Ok(shortest_start)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi"
    }

    fn input() -> Result<Map> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(31, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(29, solve_part2(&data)?))
    }
}
