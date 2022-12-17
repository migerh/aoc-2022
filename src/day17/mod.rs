use std::collections::VecDeque;

use anyhow::{Context, Result};

const WIDTH: usize = 7;
const ROCK_FORMS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

type Shape = Vec<Vec<char>>;

pub struct Rock {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    shape: Vec<Vec<char>>,
}

impl Rock {
    fn new(shape: &Shape, map_height: usize) -> Option<Self> {
        let height = shape.len();
        Some(Rock {
            left: 2,
            top: map_height + 2 + height,
            width: shape.iter().map(|v| v.len()).max()?,
            height,
            shape: shape.clone(),
        })
    }

    fn apply_stream(&mut self, jet: &char, map: &VecDeque<Vec<char>>) {
        let old = self.left;

        match jet {
            '>' => self.left += usize::from(self.left + self.width < WIDTH),
            '<' => self.left -= usize::from(self.left > 0),
            _ => (),
        };

        if self.intersects(map) {
            self.left = old;
        }
    }

    fn intersects(&self, map: &VecDeque<Vec<char>>) -> bool {
        for y in 0..self.height {
            if map.len() < self.top - y {
                continue;
            }

            for x in 0..self.width {
                let fs = self.shape[y][x];
                let line = map.get(self.top - y);
                let fk = *line.and_then(|l| l.get(x + self.left)).unwrap_or(&'.');

                if fs == '#' && fk == '#' {
                    return true;
                }
            }
        }

        false
    }

    fn fall_down(&mut self, map: &VecDeque<Vec<char>>) -> bool {
        if self.top == self.height - 1 {
            return false;
        }

        self.top -= 1;

        if self.intersects(map) {
            self.top += 1;
            return false;
        }

        true
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Result<Vec<char>> {
    Ok(input
        .chars()
        .filter(|s| *s == '<' || *s == '>')
        .collect::<Vec<_>>())
}

fn build_rock_forms() -> Vec<Vec<Vec<char>>> {
    ROCK_FORMS
        .split("\n\n")
        .map(|f| {
            f.split('\n')
                .map(|q| q.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_closed(map: &[char]) -> bool {
    map.iter().all(|v| *v == '#')
}

fn drop_rocks(streams: &[char], count: usize) -> Option<usize> {
    let available_rock_forms = build_rock_forms();
    let mut rock_forms = available_rock_forms.iter().cycle();
    let mut jet_streams = streams.iter().cycle();

    let mut map = VecDeque::new();
    let mut cut_off = 0;
    for _ in 0..count {
        let shape = rock_forms.next()?;
        let mut rock = Rock::new(shape, map.len())?;

        loop {
            let jet = jet_streams.next()?;
            rock.apply_stream(jet, &map);

            if !rock.fall_down(&map) {
                break;
            }
        }

        // add rock to map
        for y in 0..rock.height {
            let sy = rock.height - y - 1;
            let gy = rock.top - sy;

            if map.len() < gy + 1 {
                map.push_back(vec!['.'; WIDTH]);
            }

            for x in 0..rock.width {
                let r = rock.shape[sy][x];
                if r == '#' {
                    map[gy][rock.left + x] = r;
                }
            }
        }

        for y in 0..map.len() {
            let ty = map.len() - y - 1;

            if is_closed(&map[ty]) {
                cut_off += ty - 1;
                let it = map.drain(0..(ty - 1));
                drop(it);
                break;
            }
        }
    }

    Some(map.len() + cut_off)
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &[char]) -> Result<usize> {
    let result = drop_rocks(input, 2022).context("failed")?;
    Ok(result)
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &[char]) -> Result<u128> {
    let total_rocks = 1_000_000_000_000_u128;

    // the stack of rocks starts to loop after $warmup rocks
    // with a loop size of 1710 rocks. Each loop adds 2572 to
    // total height of the stack of rocks.
    // to calculate the total height of the tower we use part 1
    // to calculate the height of $warmup + $loop_size + $leftovers
    // rocks falling down.
    // to get the total height we just have to add
    // $omitted_cycles * $loop_height to the result of part 1.
    //
    // values are for my real input
    let warmup = 1700_u128;

    let loop_size = 1710_u128;
    let loop_height = 2572_u128;

    let omitted_cycles = (total_rocks - warmup) / loop_size;
    let leftovers = (total_rocks - warmup) % loop_size;

    let to_simulate = warmup + loop_size + leftovers;
    let result = drop_rocks(input, to_simulate as usize).context("failed")?;
    let height = (result as u128) + (omitted_cycles - 1) * loop_height;

    Ok(height as u128)
}
