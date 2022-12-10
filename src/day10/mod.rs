use std::str::FromStr;

use anyhow::{Context, Error, Result};

use crate::utils::ParseError;

#[derive(Debug)]
pub enum Operation {
    Addx(isize),
    Noop,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with("addx") {
            let mut split = s.split(' ');
            let _op = split.next().context("Could not parse")?;
            let value = isize::from_str(split.next().context("Could not parse")?)?;
            Ok(Operation::Addx(value))
        } else if s == "noop" {
            Ok(Operation::Noop)
        } else {
            Err(ParseError::new("Parsing failed"))?
        }
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<Vec<Operation>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Operation::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

fn excute(op: &Operation, value: &mut isize, cycle: &mut isize) -> Vec<(isize, isize)> {
    let mut result = vec![];
    match op {
        Operation::Addx(v) => {
            result.push((*cycle, *value));
            result.push((*cycle + 1, *value));
            *cycle += 2;
            *value += *v;
        }
        Operation::Noop => {
            result.push((*cycle, *value));
            *cycle += 1;
        }
    }

    result
}

fn run(ops: &[Operation]) -> Vec<(isize, isize)> {
    let mut values = vec![];
    let mut value = 1;
    let mut cycle = 1;
    for op in ops {
        let mut add = excute(op, &mut value, &mut cycle);
        values.append(&mut add);
    }

    values
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Operation]) -> Result<isize> {
    let values = run(input);
    let result = values
        .into_iter()
        .skip(19)
        .step_by(40)
        .map(|(cycle, value)| cycle * value)
        .sum::<isize>();

    Ok(result)
}

fn print_screen(screen: &[bool]) {
    for y in 0..6 {
        for x in 0..40 {
            if screen[y * 40 + x] {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Operation]) -> Result<usize> {
    let pos = run(input);
    let pixel = pos
        .into_iter()
        .map(|(cycle, value)| {
            let c = cycle % 40;
            value <= c && c <= value + 2
        })
        .collect::<Vec<_>>();

    print_screen(&pixel);

    Ok(0)
}
