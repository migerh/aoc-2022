use std::num::ParseIntError;

use anyhow::{Result, Context};

#[aoc_generator(dayXX)]
pub fn input_generator(input: &str) -> Result<Vec<i32>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
        .context("Error while parsing input")
}

#[aoc(dayXX, part1)]
pub fn solve_part1(input: &[i32]) -> Result<usize> {
    Ok(input.len())
}

#[aoc(dayXX, part2)]
pub fn solve_part2(input: &[i32]) -> Result<usize> {
    Ok(input.len())
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<i32>> {
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
