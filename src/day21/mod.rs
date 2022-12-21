use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Error, Result};

use crate::utils::ParseError;

type MonkeyPair = (String, String);

#[derive(Debug, Clone)]
pub enum Monkey {
    Value(isize),
    Add(MonkeyPair),
    Sub(MonkeyPair),
    Mul(MonkeyPair),
    Div(MonkeyPair),
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if !s.trim().contains(' ') {
            let num = s.parse::<isize>()?;
            return Ok(Monkey::Value(num));
        }

        let mut split = s.split(' ');

        let left = split
            .next()
            .context("Could not find left")?
            .trim()
            .to_owned();
        let op = split.next().context("Could not find left")?.trim();
        let right = split
            .next()
            .context("Could not find left")?
            .trim()
            .to_owned();

        let pair = (left, right);
        Ok(match op {
            "+" => Monkey::Add(pair),
            "-" => Monkey::Sub(pair),
            "*" => Monkey::Mul(pair),
            "/" => Monkey::Div(pair),
            _ => Err(ParseError::new("foo"))?,
        })
    }
}

fn parse_line(s: &str) -> Result<(String, Monkey)> {
    let mut split = s.split(": ");

    let monkey = split.next().context("No monkey found")?.trim().to_owned();
    let op = split.next().context("No operation found")?.trim();

    Ok((monkey, Monkey::from_str(op)?))
}

fn reduce(monkeys: &mut HashMap<String, Monkey>) -> Result<()> {
    let mut replacements = vec![];

    let ms = monkeys.clone();
    for (monkey, operation) in ms.iter() {
        match operation {
            Monkey::Value(_) => continue,
            Monkey::Add(pair) => {
                let m1 = ms.get(&pair.0).context("monkey not found")?;
                let m2 = ms.get(&pair.1).context("monkey not found")?;

                if let Monkey::Value(m1) = m1 {
                    if let Monkey::Value(m2) = m2 {
                        replacements.push((monkey, Monkey::Value(m1 + m2)));
                    }
                }
            }
            Monkey::Sub(pair) => {
                let m1 = ms.get(&pair.0).context("monkey not found")?;
                let m2 = ms.get(&pair.1).context("monkey not found")?;

                if let Monkey::Value(m1) = m1 {
                    if let Monkey::Value(m2) = m2 {
                        replacements.push((monkey, Monkey::Value(m1 - m2)));
                    }
                }
            }
            Monkey::Mul(pair) => {
                let m1 = ms.get(&pair.0).context("monkey not found")?;
                let m2 = ms.get(&pair.1).context("monkey not found")?;

                if let Monkey::Value(m1) = m1 {
                    if let Monkey::Value(m2) = m2 {
                        replacements.push((monkey, Monkey::Value(m1 * m2)));
                    }
                }
            }
            Monkey::Div(pair) => {
                let m1 = ms.get(&pair.0).context("monkey not found")?;
                let m2 = ms.get(&pair.1).context("monkey not found")?;

                if let Monkey::Value(m1) = m1 {
                    if let Monkey::Value(m2) = m2 {
                        replacements.push((monkey, Monkey::Value(m1 / m2)));
                    }
                }
            }
        }
    }

    for (m, op) in replacements {
        monkeys.entry(m.to_string()).and_modify(|v| *v = op);
    }

    Ok(())
}

fn get_root_value(monkeys: &HashMap<String, Monkey>) -> Option<isize> {
    let root = monkeys.get("root")?;

    if let Monkey::Value(val) = root {
        Some(*val)
    } else {
        None
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Result<HashMap<String, Monkey>> {
    input
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(parse_line)
        .collect::<Result<HashMap<_, _>>>()
        .context("Error while parsing input")
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &HashMap<String, Monkey>) -> Result<isize> {
    let mut monkeys = input.to_owned();

    while get_root_value(&monkeys).is_none() {
        reduce(&mut monkeys)?;
    }

    get_root_value(&monkeys).context("Root has no value")
}

fn is_root_match(m1: &Monkey, m2: &Monkey) -> bool {
    if let Monkey::Value(m1) = m1 {
        if let Monkey::Value(m2) = m2 {
            return m1 == m2;
        }
    }

    false
}

fn root_diff(m1: &Monkey, m2: &Monkey) -> Option<isize> {
    if let Monkey::Value(m1) = m1 {
        if let Monkey::Value(m2) = m2 {
            return Some(m1 - m2);
        }
    }

    None
}

fn extract_check_pair(monkeys: &HashMap<String, Monkey>) -> Option<MonkeyPair> {
    let root = monkeys.get("root")?;

    if let Monkey::Add(pair) = root {
        Some(pair.clone())
    } else {
        None
    }
}

fn has_unresolved_monkeys(monkeys: &HashMap<String, Monkey>) -> bool {
    for op in monkeys.values() {
        match op {
            Monkey::Add(_) | Monkey::Sub(_) | Monkey::Mul(_) | Monkey::Div(_) => return true,
            _ => continue,
        }
    }

    false
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &HashMap<String, Monkey>) -> Result<isize> {
    let mut my_value = 0;
    let me = "humn".to_string();

    loop {
        let mut monkeys = input.to_owned();
        let (m1, m2) = extract_check_pair(&monkeys).context("Root not found")?;
        monkeys.remove("root");
        monkeys.entry(me.clone()).and_modify(|v| *v = Monkey::Value(my_value));


        while has_unresolved_monkeys(&monkeys) {
            reduce(&mut monkeys)?;
        }

        let m1 = monkeys.get(&m1).context("m1 not found")?;
        let m2 = monkeys.get(&m2).context("m2 not found")?;
        if is_root_match(m1, m2) {
            break;
        }

        // speed up the search. might be off by one or two for some inputs.
        if let Some(diff) = root_diff(m1, m2) {
            my_value += (diff / 500).abs();
        }
        my_value += 1;
    }

    Ok(my_value)
}
