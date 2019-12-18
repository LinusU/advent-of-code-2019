use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::aoc;

struct Program(Vec<i64>);

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',').map(|v| v.parse::<i64>()).collect::<Result<Vec<_>, _>>().map(|r| Program(r))
    }
}

impl Program {
    fn run(&self, input: Vec<i64>) -> Vec<i64> {
        let mut memory = self.0.clone();
        let mut eip = 0usize;

        let mut input = input.iter();
        let mut output = Vec::new();

        loop {
            match memory[eip] % 100 {
                1 => {
                    let lhs_mode = (memory[eip] / 100) % 10;
                    let rhs_mode = (memory[eip] / 1000) % 10;
                    let lhs_idx = if lhs_mode == 0 { memory[eip + 1] as usize } else { eip + 1 };
                    let rhs_idx = if rhs_mode == 0 { memory[eip + 2] as usize } else { eip + 2 };
                    let out_idx = memory[eip + 3] as usize;
                    memory[out_idx] = memory[lhs_idx] + memory[rhs_idx];
                    eip += 4;
                }
                2 => {
                    let lhs_mode = (memory[eip] / 100) % 10;
                    let rhs_mode = (memory[eip] / 1000) % 10;
                    let lhs_idx = if lhs_mode == 0 { memory[eip + 1] as usize } else { eip + 1 };
                    let rhs_idx = if rhs_mode == 0 { memory[eip + 2] as usize } else { eip + 2 };
                    let out_idx = memory[eip + 3] as usize;
                    memory[out_idx] = memory[lhs_idx] * memory[rhs_idx];
                    eip += 4;
                }
                3 => {
                    let mode = (memory[eip] / 100) % 10;
                    let ptr = if mode == 0 { memory[eip + 1] as usize } else { eip + 1 };
                    memory[ptr] = *input.next().unwrap();
                    eip += 2;
                }
                4 => {
                    let ptr = memory[eip + 1] as usize;
                    let val = memory[ptr];
                    output.push(val);
                    eip += 2;
                }
                99 => {
                    break;
                }
                op => {
                    panic!("Unknown op code: {}", op);
                }
            }
        }

        output
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Result<i64, ParseIntError> {
    let program = input.parse::<Program>()?;
    Ok(*program.run(vec![1]).last().unwrap())
}

#[cfg(test)]
mod test {
    use super::Program;

    fn run(source: &str, input: Vec<i64>) -> Vec<i64> {
        source.parse::<Program>().unwrap().run(input)
    }

    #[test]
    fn part_1() {
        assert_eq!(run("1002,4,3,4,33", vec![]), vec![]);
        assert_eq!(run("1101,100,-1,4,0", vec![]), vec![]);
        assert_eq!(run("1002,6,3,6,4,0,33", vec![]), vec![1002]);
        assert_eq!(run("3,5,4,5,99,0", vec![-16]), vec![-16]);
    }
}
