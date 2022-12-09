use crate::utils::ParseError;
use std::{collections::HashSet, num::ParseIntError};

#[aoc_generator(day06)]
pub fn input_generator(input: &str) -> Result<Vec<char>, ParseIntError> {
    Ok(input.chars().collect::<Vec<_>>())
}

fn solve(input: &[char], length: usize) -> usize {
    let mut result = length;

    for w in input.windows(length) {
        let h: HashSet<char> = HashSet::from_iter(w.iter().cloned());

        if h.len() == length {
            break;
        }

        result += 1;
    }

    result
}

#[aoc(day06, part1)]
pub fn solve_part1(input: &[char]) -> Result<usize, ParseError> {
    Ok(solve(input, 4))
}

#[aoc(day06, part2)]
pub fn solve_part2(input: &[char]) -> Result<usize, ParseError> {
    Ok(solve(input, 14))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
    }

    fn input() -> Result<Vec<char>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(10, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(29, solve_part2(&data)?))
    }
}
