use std::str::FromStr;
use crate::utils::ParseError;

#[derive(Debug)]
pub struct Section {
    start: u32,
    end: u32,
}

impl FromStr for Section {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let mut range = s.split('-');

        let start = u32::from_str(range.next()
            .ok_or_else(|| ParseError::new("Invalid number of ranges"))?.trim())?;
        let end = u32::from_str(range.next()
            .ok_or_else(|| ParseError::new("Invalid number of ranges"))?.trim())?;

        Ok(Section { start, end })
    }
}

#[derive(Debug)]
pub struct Pair {
    first: Section,
    second: Section,
}

impl FromStr for Pair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let mut pairs = s.split(',');

        let first = Section::from_str(pairs.next()
            .ok_or(ParseError::new("Invalid number of elves"))?)?;
        let second = Section::from_str(pairs.next()
            .ok_or(ParseError::new("Invalid number of elves"))?)?;

        Ok(Pair { first, second })
    }
}

impl Pair {
    fn fully_contained(&self) -> bool {
        let first_contains_second = self.first.start <= self.second.start &&
            self.second.end <= self.first.end;
        let second_contains_first = self.second.start <= self.first.start &&
            self.first.end <= self.second.end;

        first_contains_second || second_contains_first
    }

    fn overlap(&self) -> bool {
        (self.second.start >= self.first.start && self.second.start <= self.first.end) ||
        (self.first.start >= self.second.start && self.first.start <= self.second.end)
    }
}

#[aoc_generator(day04)]
pub fn input_generator(input: &str) -> Result<Vec<Pair>, ParseError> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Pair::from_str)
        .collect::<Result<Vec<_>, ParseError>>()
}

#[aoc(day04, part1)]
pub fn solve_part1(input: &[Pair]) -> Result<usize, ParseError> {
    Ok(input.iter()
        .filter(|p| p.fully_contained())
        .count())
}

#[aoc(day04, part2)]
pub fn solve_part2(input: &[Pair]) -> Result<usize, ParseError> {
    Ok(input.iter()
        .filter(|p| p.overlap())
        .count())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"
    }

    fn input() -> Result<Vec<Pair>, ParseError> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(2, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(4, solve_part2(&data)?))
    }
}
