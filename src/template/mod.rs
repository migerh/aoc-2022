use std::num::ParseIntError;
use crate::utils::ParseError;

#[aoc_generator(dayXX)]
pub fn input_generator(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

#[aoc(dayXX, part1)]
pub fn solve_part1(input: &Vec<i32>) -> Result<i32, ParseError> {
    Ok(0)
}

#[aoc(dayXX, part2)]
pub fn solve_part2(input: &Vec<i32>) -> Result<i32, ParseError> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<i32>, ParseError> {
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
