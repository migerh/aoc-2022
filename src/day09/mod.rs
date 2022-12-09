use std::{str::FromStr, collections::HashSet};
use crate::utils::ParseError;

type Coords = (isize, isize);

#[derive(Debug)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug)]
pub struct Operation {
    direction: Direction,
    distance: isize,
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");

        let dir = split.next().unwrap();
        let direction = match dir {
            "L" => Direction::Left,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "U" => Direction::Up,
            _ => Err(ParseError::new("Cannot parse direction"))?
        };

        let distance = isize::from_str(split.next()
            .ok_or(ParseError::new("Cannot parse distance"))?)?;

        Ok(Operation { direction, distance })
    }
}

#[aoc_generator(day09)]
pub fn input_generator(input: &str) -> Result<Vec<Operation>, ParseError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.trim())
        .map(|s| Operation::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn move_head(op: &Operation, p: &Coords) -> Coords {
    let d = 1;
    match op.direction {
        Direction::Up => (p.0, p.1 - d),
        Direction::Left => (p.0 - d, p.1),
        Direction::Down => (p.0, d + p.1),
        Direction::Right => (p.0 + d, p.1),
    }
}

fn close_enough(a: &Coords, b: &Coords) -> bool {
    (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1
}

fn update_member(head: &Coords, member: Coords) -> Coords{
    if close_enough(head, &member) {
        return member;
    }

    let diff = (head.0 - member.0, head.1 - member.1);
    (member.0 + diff.0.signum(), member.1 + diff.1.signum())
}

fn update_tail(head: &Coords, mut tail: Vec<Coords>) -> Vec<Coords> {
    tail[0] = update_member(head, tail[0]);

    for i in 1..tail.len() {
        tail[i] = update_member(&tail[i-1], tail[i]);
    }

    tail
}

fn simulate(ops: &Vec<Operation>, mut rope: Vec<Coords>) -> Option<usize> {
    let mut visited: HashSet<Coords> = HashSet::new();
    let mut head = (0, 0);
    visited.insert(rope.last()?.clone());

    for op in ops {
        for _ in 0..op.distance {
            head = move_head(op, &head);
            rope = update_tail(&head, rope);
            visited.insert(rope.last()?.clone());
        }
    }

    Some(visited.len())
}

#[aoc(day09, part1)]
pub fn solve_part1(input: &Vec<Operation>) -> Result<usize, ParseError> {
    let rope = vec![(0, 0)];

    simulate(input, rope)
        .ok_or(ParseError::new("Cannot determine tail positions"))
}

#[aoc(day09, part2)]
pub fn solve_part2(input: &Vec<Operation>) -> Result<usize, ParseError> {
    let rope = vec![(0, 0); 9];

    simulate(input, rope)
        .ok_or(ParseError::new("Cannot determine tail positions"))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample1() -> &'static str {
        "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2"
    }

    fn input1() -> Result<Vec<Operation>, ParseError> {
        Ok(input_generator(sample1())?)
    }

    #[test]
    fn part1_sample1() -> Result<(), ParseError> {
        let data = input1()?;
        Ok(assert_eq!(13, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample1() -> Result<(), ParseError> {
        let data = input1()?;
        Ok(assert_eq!(1, solve_part2(&data)?))
    }

    fn sample2() -> &'static str {
        "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20"
    }

    fn input2() -> Result<Vec<Operation>, ParseError> {
        Ok(input_generator(sample2())?)
    }

    #[test]
    fn part1_sample2() -> Result<(), ParseError> {
        let data = input2()?;
        Ok(assert_eq!(88, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample2() -> Result<(), ParseError> {
        let data = input2()?;
        Ok(assert_eq!(36, solve_part2(&data)?))
    }
}
