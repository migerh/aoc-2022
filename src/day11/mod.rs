use anyhow::Result;

use crate::utils::ParseError;

#[derive(PartialEq)]
enum Input {
    Sample,
    Real,
}
const INPUT: Input = Input::Real;

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u128>,
    test: u128,
    next_true: usize,
    next_false: usize,
}

impl Monkey {
    fn new(items: Vec<u128>, test: u128, next_true: usize, next_false: usize) -> Monkey {
        Monkey {
            items,
            test,
            next_true,
            next_false,
        }
    }
}

fn sample_input() -> Vec<Monkey> {
    vec![
        Monkey::new(vec![79, 98], 23, 2, 3),
        Monkey::new(vec![54, 65, 75, 74], 19, 2, 0),
        Monkey::new(vec![79, 60, 97], 13, 1, 3),
        Monkey::new(vec![74], 17, 0, 1),
    ]
}

fn input() -> Vec<Monkey> {
    vec![
        Monkey::new(vec![92, 73, 86, 83, 65, 51, 55, 93], 11, 3, 4),
        Monkey::new(vec![99, 67, 62, 61, 59, 98], 2, 6, 7),
        Monkey::new(vec![81, 89, 56, 61, 99], 5, 1, 5),
        Monkey::new(vec![97, 74, 68], 17, 2, 5),
        Monkey::new(vec![78, 73], 19, 2, 3),
        Monkey::new(vec![50], 7, 1, 6),
        Monkey::new(vec![95, 88, 53, 75], 3, 0, 7),
        Monkey::new(vec![50, 77, 98, 85, 94, 56, 89], 13, 4, 0),
    ]
}

#[aoc_generator(day11)]
pub fn input_generator(_input: &str) -> Result<Vec<Monkey>> {
    Ok(if INPUT == Input::Real {
        input()
    } else {
        sample_input()
    })
}

fn sample_operation(monkey: usize, old: u128) -> Result<u128> {
    Ok(match monkey {
        0 => old * 19,
        1 => old + 6,
        2 => old * old,
        3 => old + 3,
        _ => Err(ParseError::new("404: Monkey not found"))?,
    })
}

fn operation(monkey: usize, old: u128) -> Result<u128> {
    if INPUT == Input::Sample {
        return sample_operation(monkey, old);
    }

    Ok(match monkey {
        0 => old * 5,
        1 => old * old,
        2 => old * 7,
        3 => old + 1,
        4 => old + 3,
        5 => old + 5,
        6 => old + 8,
        7 => old + 2,
        _ => Err(ParseError::new("404: Monkey not found"))?,
    })
}

fn conduct_monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, deworry_factor: u128) -> Result<Vec<usize>> {
    let mut inspect = vec![0; monkeys.len()];
    let modulus: u128 = monkeys.iter().map(|m| m.test).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                inspect[i] += 1;
                let item = monkeys[i].items[j];
                let level = if deworry_factor == 1 {
                    operation(i, item)? % modulus
                } else {
                    operation(i, item)? / 3
                };
                let next = if level % monkeys[i].test == 0 {
                    monkeys[i].next_true
                } else {
                    monkeys[i].next_false
                };

                // Prevent a possible endless loop
                if next == i {
                    continue;
                }
                monkeys[next].items.push(level);
            }
            monkeys[i].items = vec![];
        }
    }

    Ok(inspect)
}

fn level(mut inspects: Vec<usize>) -> usize {
    inspects.sort();
    inspects.into_iter().rev().take(2).product()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Monkey]) -> Result<usize> {
    let monkeys = input.to_owned();
    let inspects = conduct_monkey_business(monkeys, 20, 3)?;

    Ok(level(inspects))
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Monkey]) -> Result<usize> {
    let monkeys = input.to_owned();
    let inspects = conduct_monkey_business(monkeys, 10_000, 1)?;

    Ok(level(inspects))
}
