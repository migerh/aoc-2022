use anyhow::{Context, Error, Result};
use regex::Regex;
use std::str::FromStr;

pub type Stack = Vec<char>;
pub type State = Vec<Vec<char>>;

#[derive(Clone, Debug)]
pub struct Instruction {
    number: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
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
            .context("Error during parse")?;

        Ok(Instruction { number, from, to })
    }
}

impl Instruction {
    fn apply(&self, state: &mut State, is9001: bool) -> Result<()> {
        let source_length = state
            .get(self.from - 1)
            .context("Source stack not found")?
            .len();

        let mut containers = state
            .get_mut(self.from - 1)
            .context("Sourcce stack not found")?
            .drain((source_length - self.number)..)
            .collect::<Vec<_>>();

        if !is9001 {
            containers.reverse();
        }

        state
            .get_mut(self.to - 1)
            .context("Target stack not found")?
            .extend(containers);

        Ok(())
    }
}

#[derive(Clone)]
pub struct Operation {
    initial_state: State,
    instructions: Vec<Instruction>,
}

fn parse_state(s: &str) -> Option<State> {
    let mut lines = s.lines().rev();

    // parse line with container numbers
    let last_stack_label = lines.next()?.trim().chars().last()?;

    let number_of_stacks = last_stack_label.to_digit(10)? as usize;

    let mut stacks: State = vec![vec![]; number_of_stacks];

    for line in lines {
        let mut chars = line.chars();
        chars.next();

        let mut stack = 0;
        while let Some(c) = chars.next() {
            if c != ' ' {
                stacks.get_mut(stack)?.push(c);
            }

            // skip() would consume the iterator :/
            chars.next();
            chars.next();
            chars.next();
            stack += 1;
        }
    }

    Some(stacks)
}

#[aoc_generator(day05)]
pub fn input_generator(input: &str) -> Result<Operation> {
    let mut split = input.split("\n\n");

    let top = split.next().context("Initial state not found")?;

    let initial_state = parse_state(top).context("Could not parse initial state")?;

    let bottom = split.next().context("Instructions not found")?;

    let instructions = bottom
        .lines()
        .filter(|s| !s.is_empty())
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok(Operation {
        initial_state,
        instructions,
    })
}

#[aoc(day05, part1)]
pub fn solve_part1(input: &Operation) -> Result<String> {
    let mut state = input.initial_state.clone();
    let is9001 = false;

    for instruction in &input.instructions {
        instruction.apply(&mut state, is9001)?;
    }

    let result = state
        .iter()
        .map(|stack| stack.last().context("Stack is empty"))
        .collect::<Result<String>>()?;

    Ok(result)
}

#[aoc(day05, part2)]
pub fn solve_part2(input: &Operation) -> Result<String> {
    let mut state = input.initial_state.clone();
    let is9001 = true;

    for instruction in &input.instructions {
        instruction.apply(&mut state, is9001)?;
    }

    let result = state
        .iter()
        .map(|stack| stack.last().context("Stack is empty"))
        .collect::<Result<String>>()?;

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    }

    fn input() -> Result<Operation> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!("CMZ", &solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!("MCD", &solve_part2(&data)?))
    }
}
