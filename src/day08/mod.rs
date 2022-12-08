use std::{collections::HashMap, cmp::max};
use crate::utils::ParseError;

type Coords = (isize, isize);
type Forest = HashMap<Coords, usize>;

#[aoc_generator(day08)]
pub fn input_generator(input: &str) -> Result<Forest, ParseError> {
    Ok(input
        .lines()
        .filter(|s| *s != "")
        .enumerate()
        .map(|(i, r)| -> Option<Vec<(Coords, usize)>> {
            r.chars().enumerate().map(|(j, c)| -> Option<(Coords, usize)> {
                Some(((j as isize, i as isize), c.to_digit(10)? as usize))
            }).collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>().ok_or(ParseError::new("Could not parse"))?
        .into_iter()
        .flatten()
        .collect::<HashMap<_, _>>())
}

fn get_size(forest: &Forest) -> (isize, isize) {
    let mut width = 0;
    let mut height = 0;

    forest.iter().for_each(|(&(x, y), _)| {
        width = max(x, width);
        height = max(y, height);
    });

    (width, height)
}

fn check_tree(forest: &Forest, size: Coords, coords: &Coords) -> bool {
    look_dir(forest, size, coords, (-1, 0)) ||
    look_dir(forest, size, coords, (1, 0)) ||
    look_dir(forest, size, coords, (0, 1)) ||
    look_dir(forest, size, coords, (0, -1))
}

fn look_dir(forest: &Forest, size: Coords, coords: &Coords, direction: Coords) -> bool {
    let mut line = vec![];

    let mut p = coords.clone();
    while p.0 >= 0 && p.1 >= 0 && p.0 <= size.0 && p.1 <= size.1 {
        if let Some(t) = forest.get(&p) {
            line.push(*t);
        }
        p.0 += direction.0;
        p.1 += direction.1;
    }

    let tree_size = line[0];
    for i in 1..line.len() {
        if tree_size <= line[i] {
            return false;
        }
    }

    true
}

#[aoc(day08, part1)]
pub fn solve_part1(input: &Forest) -> Result<usize, ParseError> {
    let forest = input;
    let (width, height) = get_size(input);

    let mut count = 0;
    for (&(x, y), _) in forest {
        if x == 0 || x == width {
            count += 1;
            continue;
        }

        if y == 0 || y == height {
            count += 1;
            continue;
        }

        if check_tree(forest, (width, height), &(x, y)) {
            count += 1;
        }
    }

    Ok(count)
}

fn score(forest: &Forest, size: Coords, coords: &Coords) -> usize {
    score_dir(forest, size, coords, (0, -1)) *
    score_dir(forest, size, coords, (-1, 0)) *
    score_dir(forest, size, coords, (0, 1)) *
    score_dir(forest, size, coords, (1, 0))
}

fn score_dir(forest: &Forest, size: Coords, coords: &Coords, direction: Coords) -> usize {
    let mut line = vec![];

    let mut p = coords.clone();
    while p.0 >= 0 && p.1 >= 0 && p.0 <= size.0 && p.1 <= size.1 {
        if let Some(t) = forest.get(&p) {
            line.push(*t);
        }
        p = (p.0 + direction.0, p.1 + direction.1);
    }

    let tree_size = line[0];
    let mut score = 0;
    for i in 1..line.len() {
        if tree_size > line[i] {
            score += 1;
        } else {
            score += 1;
            break;
        }
    }

    score
}

#[aoc(day08, part2)]
pub fn solve_part2(forest: &Forest) -> Result<usize, ParseError> {
    let size = get_size(forest);

    forest.iter()
        .map(|(p, _)| score(forest, size, p))
        .max()
        .ok_or(ParseError::new("No max value found"))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Forest, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part2(&data)?))
    }
}
