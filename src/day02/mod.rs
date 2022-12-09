use std::str::FromStr;

use crate::utils::ParseError;

#[derive(Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

pub struct Game {
    you: RPS,
    opponent: RPS,
}

impl FromStr for RPS {
    type Err = ParseError;
    fn from_str(v: &str) -> Result<RPS, ParseError> {
        match v {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(ParseError::new("Could not parse")),
        }
    }
}

impl Game {
    fn from_choices(s: &str) -> Result<Game, ParseError> {
        let s = s
            .split(' ')
            .map(RPS::from_str)
            .collect::<Result<Vec<_>, ParseError>>()?;
        if s.len() != 2 {
            return Err(ParseError::new("Could not parse"));
        }

        Ok(Game {
            you: s[1].clone(),
            opponent: s[0].clone(),
        })
    }

    fn from_result(s: &str) -> Result<Game, ParseError> {
        let s = s.split(' ').collect::<Vec<&str>>();

        if s.len() != 2 {
            return Err(ParseError::new("Could not parse"));
        }

        let opponent = RPS::from_str(s[0])?;

        let you = match (s[1], &opponent) {
            ("X", RPS::Rock) => Ok(RPS::Scissors),
            ("Y", RPS::Rock) => Ok(RPS::Rock),
            ("Z", RPS::Rock) => Ok(RPS::Paper),
            ("X", RPS::Paper) => Ok(RPS::Rock),
            ("Y", RPS::Paper) => Ok(RPS::Paper),
            ("Z", RPS::Paper) => Ok(RPS::Scissors),
            ("X", RPS::Scissors) => Ok(RPS::Paper),
            ("Y", RPS::Scissors) => Ok(RPS::Scissors),
            ("Z", RPS::Scissors) => Ok(RPS::Rock),
            _ => Err(ParseError::new("Could not parse")),
        }?;

        Ok(Game { you, opponent })
    }

    fn score(&self) -> u32 {
        match (&self.you, &self.opponent) {
            (RPS::Rock, RPS::Rock) => 4,
            (RPS::Rock, RPS::Paper) => 1,
            (RPS::Rock, RPS::Scissors) => 7,
            (RPS::Paper, RPS::Rock) => 8,
            (RPS::Paper, RPS::Paper) => 5,
            (RPS::Paper, RPS::Scissors) => 2,
            (RPS::Scissors, RPS::Rock) => 3,
            (RPS::Scissors, RPS::Paper) => 9,
            (RPS::Scissors, RPS::Scissors) => 6,
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<String>, ParseError> {
    Ok(input.lines().map(|l| l.to_owned()).collect::<Vec<_>>())
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[String]) -> Result<u32, ParseError> {
    let input = input
        .iter()
        .map(|g| Game::from_choices(g))
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(input.iter().map(|g| g.score()).sum())
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[String]) -> Result<u32, ParseError> {
    let input = input
        .iter()
        .map(|g| Game::from_result(g))
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(input.iter().map(|g| g.score()).sum())
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> &'static str {
        "A Y
B X
C Z"
    }

    fn sample() -> Vec<String> {
        input_generator(sample_input()).unwrap()
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let sample = sample();
        assert_eq!(solve_part1(&sample)?, 15);
        Ok(())
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let sample = sample();
        assert_eq!(solve_part2(&sample)?, 12);
        Ok(())
    }
}
