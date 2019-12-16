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

fn fuel_cost(mass: u64) -> u64 {
    if mass < 9 {
        0
    } else {
        let fuel = (mass / 3) - 2;
        fuel + fuel_cost(fuel)
    }
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    Ok(input
        .split_whitespace()
        .map(|mass| mass.parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()?
        .iter()
        .map(|mass| fuel_cost(*mass))
        .sum())
}
