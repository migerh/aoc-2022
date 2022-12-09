use std::num::ParseIntError;

use crate::utils::ParseError;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Result<Vec<String>, ParseIntError> {
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<_>>())
}

fn score(c: char) -> Result<u32, ParseError> {
    match c {
        'a'..='z' => Ok(u32::from(c) - 96),
        'A'..='Z' => Ok(u32::from(c) - 38),
        _ => Err(ParseError::new("Invalid item found")),
    }
}

fn score_item_collections(input: &[String]) -> Result<u32, ParseError> {
    if input.is_empty() {
        return Err(ParseError::new("Not enough elves"));
    }

    for c in input[0].chars() {
        let all_others_contain_c = input.iter().skip(1).all(|v| v.contains(c));

        if all_others_contain_c {
            return score(c);
        }
    }

    Err(ParseError::new("No duplicate found"))
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> Result<u32, ParseError> {
    let score: u32 = input
        .iter()
        .map(|s| -> Vec<String> {
            let len = s.len();
            let compartment1 = s.chars().take(len / 2).collect::<String>();
            let compartment2 = s.chars().skip(len / 2).collect::<String>();
            vec![compartment1, compartment2]
        })
        .map(|k| score_item_collections(&k))
        .collect::<Result<Vec<_>, ParseError>>()?
        .iter()
        .sum();
    Ok(score)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> Result<u32, ParseError> {
    let score: u32 = input
        .chunks(3)
        .map(score_item_collections)
        .collect::<Result<Vec<_>, ParseError>>()?
        .iter()
        .sum();
    Ok(score)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    }

    fn input() -> Result<Vec<String>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(157, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(70, solve_part2(&data)?))
    }
}
