use std::str::FromStr;

use crate::utils::ParseError;

#[derive(Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

pub enum GameState{
    Loss,
    Draw,
    Win,
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
            _ => Err(ParseError::new("Could not parse"))
        }
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(v: &str) -> Result<Game, ParseError> {
        let s = v.split(" ")
            .map(|s| RPS::from_str(s))
            .collect::<Result<Vec<_>, ParseError>>()?;
        if s.len() != 2 {
            return Err(ParseError::new("Could not parse"));
        }

        Ok(Game { you: s[1].clone(), opponent: s[0].clone() })
    }
}

impl Game {
    fn game_state(&self) -> GameState {
        match self.you {
            RPS::Rock => GameState::Loss,
            RPS::Paper => GameState::Draw,
            RPS::Scissors => GameState::Win,
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Game>, ParseError> {
    Ok(input
        .lines()
        .map(|l| Game::from_str(l))
        .collect::<Result<Vec<_>, _>>()?)
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Game>) -> Result<u32, ParseError> {
    let mut sum = 0;
    for game in input {
        let score = match (&game.you, &game.opponent) {
            (RPS::Rock, RPS::Rock) => 4,
            (RPS::Rock, RPS::Paper) => 1,
            (RPS::Rock, RPS::Scissors) => 7,
            (RPS::Paper, RPS::Rock) => 8,
            (RPS::Paper, RPS::Paper) => 5,
            (RPS::Paper, RPS::Scissors) => 2,
            (RPS::Scissors, RPS::Rock) => 3,
            (RPS::Scissors, RPS::Paper) => 9,
            (RPS::Scissors, RPS::Scissors) => 6,
        };
        sum += score;
    }
    Ok(sum)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Game>) -> Result<u32, ParseError> {
    let mut sum = 0;
    for game in input {
        let score = match (game.game_state(), &game.opponent) {
            (GameState::Loss, RPS::Rock) => 3,
            (GameState::Loss, RPS::Paper) => 1,
            (GameState::Loss, RPS::Scissors) => 2,
            (GameState::Draw, RPS::Rock) => 4,
            (GameState::Draw, RPS::Paper) => 5,
            (GameState::Draw, RPS::Scissors) => 6,
            (GameState::Win, RPS::Rock) => 8,
            (GameState::Win, RPS::Paper) => 9,
            (GameState::Win, RPS::Scissors) => 7,
        };
        sum += score;
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> &'static str {
        "A Y
B X
C Z"
    }

    fn sample() -> Vec<Game> {
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
