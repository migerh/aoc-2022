use std::num::ParseIntError;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input
        .lines()
        .filter(|s| *s != "")
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<u32>) -> Result<usize, std::num::ParseIntError> {
    Ok(input
        .windows(2)
        .filter(|v| v[1] > v[0])
        .count())
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<u32>) -> Result<usize, std::num::ParseIntError> {
    let windowed = input
        .windows(3)
        .map(|v| v.into_iter().sum::<u32>())
        .collect::<Vec<_>>();
    solve_part1(&windowed)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> Vec<u32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn part1_sample() -> Result<(), ParseIntError> {
        let sample = sample();
        assert_eq!(solve_part1(&sample)?, 7);
        Ok(())
    }

    #[test]
    fn part2_sample() -> Result<(), ParseIntError> {
        let sample = sample();
        assert_eq!(solve_part2(&sample)?, 5);
        Ok(())
    }
}
