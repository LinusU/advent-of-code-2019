use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::aoc;

struct Program(Vec<u64>);

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',').map(|v| v.parse::<u64>()).collect::<Result<Vec<_>, _>>().map(|r| Program(r))
    }
}

impl Program {
    fn run(&self, noun: u64, verb: u64) -> u64 {
        let mut memory = self.0.clone();
        let mut eip = 0usize;

        memory[1] = noun;
        memory[2] = verb;

        loop {
            match memory[eip] {
                1 => {
                    let lhs_idx = memory[eip + 1] as usize;
                    let rhs_idx = memory[eip + 2] as usize;
                    let out_idx = memory[eip + 3] as usize;
                    memory[out_idx] = memory[lhs_idx] + memory[rhs_idx];
                    eip += 4;
                }
                2 => {
                    let lhs_idx = memory[eip + 1] as usize;
                    let rhs_idx = memory[eip + 2] as usize;
                    let out_idx = memory[eip + 3] as usize;
                    memory[out_idx] = memory[lhs_idx] * memory[rhs_idx];
                    eip += 4;
                }
                99 => {
                    break;
                }
                _ => {
                    unreachable!();
                }
            }
        }

        memory[0]
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let program = input.parse::<Program>()?;
    Ok(program.run(12, 2))
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let program = input.parse::<Program>()?;

    for noun in 0..=99 {
        for verb in 0..=99 {
            if program.run(noun, verb) == 19690720 {
                return Ok(100 * noun + verb)
            }
        }
    }

    unreachable!();
}
