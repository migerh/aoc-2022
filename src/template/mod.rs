use std::str::FromStr;

use anyhow::{Result, Error, Context};

pub struct Foo {
}

impl FromStr for Foo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}

#[aoc_generator(dayXX)]
pub fn input_generator(input: &str) -> Result<Vec<Foo>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Foo::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(dayXX, part1)]
pub fn solve_part1(input: &[Foo]) -> Result<usize> {
    Ok(input.len())
}

#[aoc(dayXX, part2)]
pub fn solve_part2(input: &[Foo]) -> Result<usize> {
    Ok(input.len())
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Foo>> {
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
