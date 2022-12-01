use std::num::ParseIntError;

use crate::utils::ParseError;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    let mut elves: Vec<u32> = vec![];
    let mut buffer: Vec<u32> = vec![];

    for line in input.lines() {
        if line.len() == 0 {
            elves.push(buffer.iter().sum());
            buffer = vec![];
            continue;
        }
        buffer.push(line.parse::<u32>()?);
    }

    if buffer.len() > 0 {
        elves.push(buffer.iter().sum());
    }

    Ok(elves)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<u32>) -> Result<u32, ParseError> {
    Ok(*input
        .iter()
        .max()
        .ok_or(ParseError::new("Could not find max element"))?)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<u32>) -> Result<u32, ParseError> {
    if input.len() < 3 {
        return Err(ParseError::new("Not enough elves"))?;
    }

    let mut list = input.clone();
    list.sort();

    let top3sum = list.iter().rev().take(3).sum();

    Ok(top3sum)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> &'static str {
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
    }

    fn sample() -> Vec<u32> {
        input_generator(sample_input()).unwrap()
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let sample = sample();
        assert_eq!(solve_part1(&sample)?, 24_000);
        Ok(())
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let sample = sample();
        println!("{:?}", sample);
        assert_eq!(solve_part2(&sample)?, 45_000);
        Ok(())
    }
}
