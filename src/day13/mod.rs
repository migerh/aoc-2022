use serde_json::{from_str, Value};
use std::{
    cmp::{min, Ordering},
    str::FromStr,
};

use anyhow::{Context, Error, Result};

pub struct PacketWithDivider {
    is_divider: bool,
    packet: Packet,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    Val(isize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Val(a), Packet::Val(b)) => a.cmp(b),
            (Packet::List(_), Packet::Val(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::Val(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            (Packet::List(a), Packet::List(b)) => {
                let min = min(a.len(), b.len());
                for i in 0..min {
                    let c = a[i].cmp(&b[i]);

                    if c != Ordering::Equal {
                        return c;
                    }
                }

                if a.len() == b.len() {
                    Ordering::Equal
                } else if a.len() <= min {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let val: Value = from_str(s)?;
        Packet::from_value(val).context("Could not parse")
    }
}

impl Packet {
    fn from_value(s: Value) -> Option<Packet> {
        if let Value::Array(a) = s {
            let packets = a
                .into_iter()
                .map(Packet::from_value)
                .collect::<Option<Vec<_>>>()?;
            Some(Packet::List(packets))
        } else if let Value::Number(a) = s {
            Some(Packet::Val(a.as_u64()? as isize))
        } else {
            None
        }
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<Vec<Packet>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Packet::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[Packet]) -> Result<usize> {
    let mut index = 1;
    let mut result = 0;
    for p in input.chunks(2) {
        if p.len() != 2 {
            continue;
        }

        if p[0].cmp(&p[1]) == Ordering::Less {
            result += index;
        }
        index += 1;
    }
    Ok(result)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[Packet]) -> Result<usize> {
    let mut packets = input
        .iter()
        .cloned()
        .map(|v| PacketWithDivider {
            is_divider: false,
            packet: v,
        })
        .collect::<Vec<_>>();

    packets.push(PacketWithDivider {
        is_divider: true,
        packet: Packet::List(vec![Packet::List(vec![Packet::Val(2)])]),
    });
    packets.push(PacketWithDivider {
        is_divider: true,
        packet: Packet::List(vec![Packet::List(vec![Packet::Val(6)])]),
    });
    packets.sort_by(|a, b| a.packet.cmp(&b.packet));

    let result: usize = packets
        .into_iter()
        .enumerate()
        .filter(|(_, v)| v.is_divider)
        .map(|(i, _)| i + 1)
        .product();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
    }

    fn input() -> Result<Vec<Packet>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(13, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(140, solve_part2(&data)?))
    }
}
