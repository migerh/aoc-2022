use regex::Regex;
use std::str::FromStr;

use crate::utils::ParseError;

pub type Stack = Vec<char>;
pub type State = Vec<Vec<char>>;

#[derive(Clone, Debug)]
pub struct Instruction {
    number: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^move (?P<number>\d+)? from (?P<from>\d) to (?P<to>\d)$").unwrap();
        }

        let (number, from, to) = RE
            .captures(s)
            .and_then(|cap| {
                let number = cap
                    .name("number")
                    .map(|v| v.as_str().parse::<usize>())?
                    .ok()?;

                let from = cap
                    .name("from")
                    .map(|v| v.as_str().parse::<usize>())?
                    .ok()?;

                let to = cap.name("to").map(|v| v.as_str().parse::<usize>())?.ok()?;

                Some((number, from, to))
            })
            .ok_or(ParseError::new("Error during parse"))?;

        Ok(Instruction { number, from, to })
    }
}

impl Instruction {
    fn apply(&self, state: &mut State, is9001: bool) -> Result<(), ParseError> {
        let source_length = state.get(self.from - 1)
            .ok_or(ParseError::new("Source stack not found"))?
            .len();

        let mut containers = state
            .get_mut(self.from - 1)
            .ok_or(ParseError::new("Sourcce stack not found"))?
            .drain((source_length - self.number)..)
            .collect::<Vec<_>>();

        if !is9001 {
            containers.reverse();
        }

        state
            .get_mut(self.to - 1)
            .ok_or(ParseError::new("Target stack not found"))?
            .extend(containers);

        Ok(())
    }
}

#[derive(Clone)]
pub struct Operation {
    initial_state: State,
    instructions: Vec<Instruction>,
}

#[aoc_generator(day05)]
pub fn input_generator(input: &str) -> Result<Operation, ParseError> {
    let mut split = input.split("\n\n");

    let _top = split
        .next()
        .ok_or(ParseError::new("Initial state not found"))?;
    let bottom = split
        .next()
        .ok_or(ParseError::new("Instructions not found"))?;

    let instructions = bottom
        .lines()
        .filter(|s| *s != "")
        .map(|s| Instruction::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()?;

    let initial_state = vec![
        "TDWZVP".chars().collect::<Vec<char>>(),
        "LSWVFJD".chars().collect::<Vec<char>>(),
        "ZMLSVTBH".chars().collect::<Vec<char>>(),
        "RSJ".chars().collect::<Vec<char>>(),
        "CZBGFMLW".chars().collect::<Vec<char>>(),
        "QWVHZRGB".chars().collect::<Vec<char>>(),
        "VJPCBDN".chars().collect::<Vec<char>>(),
        "PTBQ".chars().collect::<Vec<char>>(),
        "HGZRC".chars().collect::<Vec<char>>(),
    ];

    Ok(Operation {
        initial_state,
        instructions,
    })
}

#[aoc(day05, part1)]
pub fn solve_part1(input: &Operation) -> Result<String, ParseError> {
    let mut state = input.initial_state.clone();
    let is9001 = false;

    for instruction in &input.instructions {
        instruction.apply(&mut state, is9001)?;
    }

    let result = state
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    Ok(result)
}

#[aoc(day05, part2)]
pub fn solve_part2(input: &Operation) -> Result<String, ParseError> {
    let mut state = input.initial_state.clone();
    let is9001 = true;

    for instruction in &input.instructions {
        instruction.apply(&mut state, is9001)?;
    }

    let result = state
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "<state>

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    }

    fn input() -> Result<Operation, ParseError> {
        let initial_state = vec![
            "ZN".chars().collect::<Vec<char>>(),
            "MCD".chars().collect::<Vec<char>>(),
            "P".chars().collect::<Vec<char>>(),
        ];
        let mut operation = input_generator(sample())?;
        operation.initial_state = initial_state;

        Ok(operation)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!("CMZ", &solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!("MCD", &solve_part2(&data)?))
    }
}
