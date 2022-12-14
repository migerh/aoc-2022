use std::collections::HashMap;

use anyhow::{Context, Result};

type Coords = (isize, isize);

type Rocks = HashMap<Coords, char>;

fn parse_coords(s: &str) -> Result<Coords> {
    let mut split = s.split(',');

    let x = split
        .next()
        .context("Could not parse coords")?
        .parse::<isize>()?;
    let y = split
        .next()
        .context("Could not parse coords")?
        .parse::<isize>()?;

    Ok((x, y))
}

fn collect_rocks(rocks: &mut Rocks, s: &str) -> Result<()> {
    let points = s
        .split(" -> ")
        .map(parse_coords)
        .collect::<Result<Vec<_>>>()?;

    for ps in points.windows(2) {
        let a = ps[0];
        let b = ps[1];

        if a.0 == b.0 {
            let (start, end) = if a.1 < b.1 { (a.1, b.1) } else { (b.1, a.1) };
            for i in start..=end {
                rocks.entry((a.0, i)).or_insert('#');
            }
        } else {
            let (start, end) = if a.0 < b.0 { (a.0, b.0) } else { (b.0, a.0) };
            for i in start..=end {
                rocks.entry((i, a.1)).or_insert('#');
            }
        }
    }

    Ok(())
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Result<Rocks> {
    let mut rocks = Rocks::new();

    for line in input.lines().filter(|s| !s.trim().is_empty()) {
        collect_rocks(&mut rocks, line)?;
    }

    Ok(rocks)
}

fn lowest_point(rocks: &Rocks) -> Option<isize> {
    rocks.keys().map(|&(_, y)| y).max()
}

fn falling_sand(rocks: &mut Rocks, max_y: isize) -> bool {
    let mut p = (500, 0);
    let mut settled = false;

    while p.1 < max_y + 5 && !settled {
        let mut next = (p.0, p.1 + 1);

        if rocks.contains_key(&next) {
            next.0 -= 1;
        }

        if rocks.contains_key(&next) {
            next.0 += 2;
        }

        if rocks.contains_key(&next) {
            rocks.entry(p).and_modify(|v| *v = 'o').or_insert('o');
            settled = true;
            continue;
        }

        p = next;
    }

    let part1 = settled && p.1 < max_y + 1;
    let part2 = settled && p.0 == 500 && p.1 == 0;
    part1 && !part2
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Rocks) -> Result<usize> {
    let mut rocks = input.to_owned();

    let max_y = lowest_point(&rocks).context("Empty rock formation")?;

    let mut count = 0;
    while falling_sand(&mut rocks, max_y) {
        count += 1;
    }

    Ok(count)
}

fn add_floor(rocks: &mut Rocks, max_y: isize) -> Option<()> {
    let min_x = rocks.keys().map(|&(x, _)| x).min()? - 500;
    let max_x = rocks.keys().map(|&(x, _)| x).max()? + 500;

    for x in min_x..=max_x {
        rocks.entry((x, max_y + 2)).or_insert('#');
    }

    Some(())
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Rocks) -> Result<usize> {
    let mut rocks = input.to_owned();
    let max_y = lowest_point(&rocks).context("Empty rock formation")?;
    add_floor(&mut rocks, max_y).context("Could not add floor")?;

    let mut count = 0;
    while falling_sand(&mut rocks, max_y + 2) {
        count += 1;
    }

    Ok(count + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
    }

    fn input() -> Result<Rocks> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(24, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(93, solve_part2(&data)?))
    }
}
