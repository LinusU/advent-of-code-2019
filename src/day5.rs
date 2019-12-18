use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::aoc;

#[derive(Clone, Copy)]
enum Parameter {
    Position(usize),
    Immediate(i64),
}

impl Parameter {
    fn new(mode: i64, value: i64) -> Parameter {
        match mode {
            0 => Parameter::Position(value as usize),
            1 => Parameter::Immediate(value),
            _ => panic!("Invalid parameter mode {}", mode),
        }
    }

    fn read(memory: &[i64], eip: usize, offset: usize) -> Parameter {
        let mode = (memory[eip] / 10i64.pow(1 + (offset as u32))) % 10;
        let value = memory[eip + offset];

        Parameter::new(mode, value)
    }

    fn load(self, memory: &[i64]) -> i64 {
        match self {
            Parameter::Position(pos) => memory[pos],
            Parameter::Immediate(value) => value,
        }
    }

    fn store(self, value: i64, memory: &mut [i64]) {
        match self {
            Parameter::Position(pos) => memory[pos] = value,
            Parameter::Immediate(_) => panic!("Cannot store to an immediate mode parameter"),
        }
    }
}

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
                    let lhs = Parameter::read(&memory, eip, 1);
                    let rhs = Parameter::read(&memory, eip, 2);
                    let out = Parameter::read(&memory, eip, 3);
                    out.store(lhs.load(&memory) + rhs.load(&memory), &mut memory);
                    eip += 4;
                }
                2 => {
                    let lhs = Parameter::read(&memory, eip, 1);
                    let rhs = Parameter::read(&memory, eip, 2);
                    let out = Parameter::read(&memory, eip, 3);
                    out.store(lhs.load(&memory) * rhs.load(&memory), &mut memory);
                    eip += 4;
                }
                3 => {
                    let out = Parameter::read(&memory, eip, 1);
                    out.store(*input.next().unwrap(), &mut memory);
                    eip += 2;
                }
                4 => {
                    let src = Parameter::read(&memory, eip, 1);
                    output.push(src.load(&memory));
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
