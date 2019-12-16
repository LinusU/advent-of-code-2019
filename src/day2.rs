use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let mut program = input
        .split(',')
        .map(|code| code.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    program[1] = 12;
    program[2] = 2;

    let mut idx = 0usize;
    loop {
        match program[idx] {
            1 => {
                let lhs_idx = program[idx + 1] as usize;
                let rhs_idx = program[idx + 2] as usize;
                let out_idx = program[idx + 3] as usize;
                program[out_idx] = program[lhs_idx] + program[rhs_idx];
                idx += 4;
            }
            2 => {
                let lhs_idx = program[idx + 1] as usize;
                let rhs_idx = program[idx + 2] as usize;
                let out_idx = program[idx + 3] as usize;
                program[out_idx] = program[lhs_idx] * program[rhs_idx];
                idx += 4;
            }
            99 => {
                break;
            }
            _ => {
                unreachable!();
            }
        }
    }

    Ok(program[0])
}
