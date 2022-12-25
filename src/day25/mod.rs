use anyhow::Result;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<char>>> {
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

fn to_dec(s: &[char]) -> isize {
    let mut result = 0;

    for (i, c) in s.iter().rev().enumerate() {
        let fives = isize::pow(5, i as u32);

        result += match c {
            '2' => 2 * fives,
            '1' => fives,
            '0' => 0,
            '-' => -fives,
            '=' => -2 * fives,
            _ => unreachable!(),
        };
    }

    result
}

fn to_snafu(mut n: isize) -> Vec<char> {
    let mut result = vec![];

    while n > 0 {
        let (c, add) = match n % 5 {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1),
            4 => ('-', 1),
            _ => unreachable!()
        };
        result.push(c);

        n /= 5;
        n += add;
    }

    result.into_iter().rev().collect::<Vec<_>>()
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> Result<String> {
    let sum = input.iter().map(|l| to_dec(l)).sum();
    let result = to_snafu(sum).into_iter().collect::<String>();

    Ok(result)
}

#[aoc(day25, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> Result<usize> {
    Ok(input.len())
}
