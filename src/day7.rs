use std::collections::VecDeque;
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

struct Process {
    memory: Vec<i64>,
    eip: usize,
    input_buffer: VecDeque<i64>,
    output_buffer: VecDeque<i64>,
}

#[derive(PartialEq)]
enum ProcessRunResult {
    Complete,
    WouldBlock,
}

impl Process {
    fn feed(&mut self, value: i64) {
        self.input_buffer.push_back(value);
    }

    fn read(&mut self) -> Option<i64> {
        self.output_buffer.pop_front()
    }

    fn run(&mut self) -> ProcessRunResult {
        loop {
            match self.memory[self.eip] % 100 {
                1 => {
                    let lhs = Parameter::read(&self.memory, self.eip, 1);
                    let rhs = Parameter::read(&self.memory, self.eip, 2);
                    let out = Parameter::read(&self.memory, self.eip, 3);
                    out.store(lhs.load(&self.memory) + rhs.load(&self.memory), &mut self.memory);
                    self.eip += 4;
                }
                2 => {
                    let lhs = Parameter::read(&self.memory, self.eip, 1);
                    let rhs = Parameter::read(&self.memory, self.eip, 2);
                    let out = Parameter::read(&self.memory, self.eip, 3);
                    out.store(lhs.load(&self.memory) * rhs.load(&self.memory), &mut self.memory);
                    self.eip += 4;
                }
                3 => {
                    let input = match self.input_buffer.pop_front() {
                        None => return ProcessRunResult::WouldBlock,
                        Some(value) => value,
                    };

                    let out = Parameter::read(&self.memory, self.eip, 1);
                    out.store(input, &mut self.memory);
                    self.eip += 2;
                }
                4 => {
                    let src = Parameter::read(&self.memory, self.eip, 1);
                    self.output_buffer.push_back(src.load(&self.memory));
                    self.eip += 2;
                }
                5 => {
                    let test = Parameter::read(&self.memory, self.eip, 1);
                    let jump = Parameter::read(&self.memory, self.eip, 2);

                    if test.load(&self.memory) != 0 {
                        self.eip = jump.load(&self.memory) as usize;
                    } else {
                        self.eip += 3;
                    }
                }
                6 => {
                    let test = Parameter::read(&self.memory, self.eip, 1);
                    let jump = Parameter::read(&self.memory, self.eip, 2);

                    if test.load(&self.memory) == 0 {
                        self.eip = jump.load(&self.memory) as usize;
                    } else {
                        self.eip += 3;
                    }
                }
                7 => {
                    let lhs = Parameter::read(&self.memory, self.eip, 1);
                    let rhs = Parameter::read(&self.memory, self.eip, 2);
                    let out = Parameter::read(&self.memory, self.eip, 3);
                    out.store(if lhs.load(&self.memory) < rhs.load(&self.memory) { 1 } else { 0 }, &mut self.memory);
                    self.eip += 4;
                }
                8 => {
                    let lhs = Parameter::read(&self.memory, self.eip, 1);
                    let rhs = Parameter::read(&self.memory, self.eip, 2);
                    let out = Parameter::read(&self.memory, self.eip, 3);
                    out.store(if lhs.load(&self.memory) == rhs.load(&self.memory) { 1 } else { 0 }, &mut self.memory);
                    self.eip += 4;
                }
                99 => {
                    return ProcessRunResult::Complete;
                }
                op => {
                    panic!("Unknown op code: {}", op);
                }
            }
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
    fn spawn(&self) -> Process {
        Process {
            memory: self.0.clone(),
            eip: 0,
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
        }
    }
}

fn permutations(mut values: [i64; 5]) -> Vec<[i64; 5]> {
    fn inner(out: &mut Vec<[i64; 5]>, data: &mut [i64; 5], l: usize, r: usize) {
        if l == r {
            out.push(data.clone());
        } else {
            for i in l..=r {
                data.swap(l, i);
                inner(out, data, l + 1, r);
                data.swap(l, i);
            }
        }
    }

    let mut result = Vec::new();
    inner(&mut result, &mut values, 0, 4);
    result
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> Result<i64, ParseIntError> {
    let program = input.parse::<Program>()?;
    let configurations = permutations([0, 1, 2, 3, 4]);

    let mut max = 0;

    for config in configurations {
        let mut a = program.spawn();
        let mut b = program.spawn();
        let mut c = program.spawn();
        let mut d = program.spawn();
        let mut e = program.spawn();

        a.feed(config[0]);
        b.feed(config[1]);
        c.feed(config[2]);
        d.feed(config[3]);
        e.feed(config[4]);

        a.feed(0);
        a.run();
        b.feed(a.read().unwrap());
        b.run();
        c.feed(b.read().unwrap());
        c.run();
        d.feed(c.read().unwrap());
        d.run();
        e.feed(d.read().unwrap());
        e.run();
        let result = e.read().unwrap();

        if result > max {
            max = result;
        }
    }

    Ok(max)
}

#[cfg(test)]
mod test {
    use super::Program;

    fn factorial(num: usize) -> usize {
        match num {
            0 => 1,
            n => factorial(n - 1) * n,
        }
    }

    fn run(source: &str, input: Vec<i64>) -> Vec<i64> {
        let program = source.parse::<Program>().unwrap();

        let mut process = program.spawn();
        let mut input = input.iter();
        let mut output = Vec::new();

        loop {
            match process.run() {
                super::ProcessRunResult::Complete => break,
                super::ProcessRunResult::WouldBlock => process.input_buffer.push_back(*input.next().unwrap()),
            }
        }

        while let Some(foo) = process.output_buffer.pop_front() {
            output.push(foo);
        }

        output
    }

    #[test]
    fn permutations() {
        use std::collections::HashSet;

        let permutations = super::permutations([0, 1, 2, 3, 4]);

        assert_eq!(permutations.len(), factorial(5));

        let mut seen = HashSet::<[i64; 5]>::new();
        for value in permutations { seen.insert(value); }

        assert_eq!(seen.len(), factorial(5));
    }

    #[test]
    fn day5_part1() {
        assert_eq!(run("1002,4,3,4,33", vec![]), vec![]);
        assert_eq!(run("1101,100,-1,4,0", vec![]), vec![]);
        assert_eq!(run("1002,6,3,6,4,0,33", vec![]), vec![1002]);
        assert_eq!(run("3,5,4,5,99,0", vec![-16]), vec![-16]);
    }

    #[test]
    fn day5_part2() {
        assert_eq!(run("3,9,8,9,10,9,4,9,99,-1,8", vec![7]), vec![0]);
        assert_eq!(run("3,9,8,9,10,9,4,9,99,-1,8", vec![8]), vec![1]);

        assert_eq!(run("3,9,7,9,10,9,4,9,99,-1,8", vec![7]), vec![1]);
        assert_eq!(run("3,9,7,9,10,9,4,9,99,-1,8", vec![8]), vec![0]);

        assert_eq!(run("3,3,1108,-1,8,3,4,3,99", vec![7]), vec![0]);
        assert_eq!(run("3,3,1108,-1,8,3,4,3,99", vec![8]), vec![1]);

        assert_eq!(run("3,3,1107,-1,8,3,4,3,99", vec![7]), vec![1]);
        assert_eq!(run("3,3,1107,-1,8,3,4,3,99", vec![8]), vec![0]);

        assert_eq!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![0]), vec![0]);
        assert_eq!(run("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![2]), vec![1]);

        assert_eq!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![0]), vec![0]);
        assert_eq!(run("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![2]), vec![1]);
    }

    #[test]
    fn day7_part1() {
        assert_eq!(super::part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), Ok(43210));
    }
}
