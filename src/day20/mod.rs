use std::{collections::VecDeque, str::FromStr};

use anyhow::{Context, Result};
use itertools::Itertools;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Result<Vec<isize>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| Ok(isize::from_str(s)?))
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

fn mix(input: &[isize], repeat: usize) -> Result<Vec<isize>> {
    let mut with_indexes = input
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (i, v))
        .collect::<VecDeque<_>>();
    let len = input.len() as isize;

    for _ in 0..repeat {
        for index in 0..input.len() {
            let (index_to_move, _) = with_indexes
                .iter()
                .find_position(|(i, _)| *i == index)
                .context("element not found")?;
            let element = with_indexes
                .remove(index_to_move)
                .context("could not remove")?;

            let shift = element.1;
            let mut index_to_move_to = (index_to_move as isize) + shift;
            if index_to_move_to < 0 {
                let factor = (index_to_move_to / (len - 1) + 1).abs();
                index_to_move_to += factor * (len - 1);
            }
            while index_to_move_to < 0 {
                index_to_move_to += len - 1;
            }

            if index_to_move_to >= len {
                let factor = index_to_move_to / (len - 1);
                index_to_move_to -= factor * (len - 1);
            }
            while index_to_move_to >= len {
                index_to_move_to -= len - 1;
            }

            if index_to_move_to == 0 {
                index_to_move_to = len - 1;
            }

            with_indexes.insert(index_to_move_to as usize, element);
        }
    }

    Ok(with_indexes.into_iter().map(|(_, v)| v).collect::<Vec<_>>())
}

fn hash(mixed: &[isize]) -> Result<isize> {
    let len = mixed.len();
    let (index, _) = mixed
        .iter()
        .find_position(|v| **v == 0)
        .context("Could not find 0")?;

    let a = mixed[(index + 1000) % len as usize];
    let b = mixed[(index + 2000) % len as usize];
    let c = mixed[(index + 3000) % len as usize];

    Ok(a + b + c)
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &[isize]) -> Result<isize> {
    let mixed = mix(input, 1)?;
    hash(&mixed)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &[isize]) -> Result<isize> {
    const FOO: isize = 811589153;
    let input = input.iter().map(|v| *v * FOO).collect::<Vec<_>>();
    let mixed = mix(&input, 10)?;
    hash(&mixed)
}
