use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    Ok(input
        .split_whitespace()
        .map(|mass| mass.parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()?
        .iter()
        .map(|mass| (mass / 3) - 2)
        .sum())
}
