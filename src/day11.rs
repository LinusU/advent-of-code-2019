use std::collections::{HashMap, VecDeque};
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use aoc_runner_derive::aoc;

struct Memory(Vec<i64>);

impl Index<usize> for Memory {
    type Output = i64;

    fn index(&self, index: usize) -> &i64 {
        if index < self.0.len() {
            self.0.index(index)
        } else {
            &0
        }
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut i64 {
        while self.0.len() <= index {
            self.0.push(0);
        }

        self.0.index_mut(index)
    }
}

#[derive(Clone, Copy)]
enum Parameter {
    Position(usize),
    Immediate(i64),
    Relative(isize),
}

impl Parameter {
    fn new(mode: i64, value: i64) -> Parameter {
        match mode {
            0 => Parameter::Position(value as usize),
            1 => Parameter::Immediate(value),
            2 => Parameter::Relative(value as isize),
            _ => panic!("Invalid parameter mode {}", mode),
        }
    }

    fn read(memory: &Memory, eip: usize, offset: usize) -> Parameter {
        let mode = (memory[eip] / 10i64.pow(1 + (offset as u32))) % 10;
        let value = memory[eip + offset];

        Parameter::new(mode, value)
    }

    fn load(self, memory: &Memory, rbo: usize) -> i64 {
        match self {
            Parameter::Position(pos) => memory[pos],
            Parameter::Immediate(value) => value,
            Parameter::Relative(offset) => memory[((rbo as isize) + offset) as usize],
        }
    }

    fn store(self, value: i64, memory: &mut Memory, rbo: usize) {
        match self {
            Parameter::Position(pos) => memory[pos] = value,
            Parameter::Immediate(_) => panic!("Cannot store to an immediate mode parameter"),
            Parameter::Relative(offset) => memory[((rbo as isize) + offset) as usize] = value,
        }
    }
}

struct Process {
    memory: Memory,
    eip: usize,
    rbo: usize,
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
                    out.store(lhs.load(&self.memory, self.rbo) + rhs.load(&self.memory, self.rbo), &mut self.memory, self.rbo);
                    self.eip += 4;
                }
                2 => {
                    let lhs = Parameter::read(&self.memory, self.eip, 1);
                    let rhs = Parameter::read(&self.memory, self.eip, 2);
                    let out = Parameter::read(&self.memory, self.eip, 3);
                    out.store(lhs.load(&self.memory, self.rbo) * rhs.load(&self.memory, self.rbo), &mut self.memory, self.rbo);
                    self.eip += 4;
                }
                3 => {
                    let input = match self.input_buffer.pop_front() {
                        None => return ProcessRunResult::WouldBlock,
                        Some(value) => value,
                    };

                    let out = Parameter::read(&self.memory, self.eip, 1);
                    out.store(input, &mut self.memory, self.rbo);
                    self.eip += 2;
                }
                4 => {
                    let src = Parameter::read(&self.memory, self.eip, 1);
                    self.output_buffer.push_back(src.load(&self.memory, self.rbo));
                    self.eip += 2;
                }
                5 => {
                    let test = Parameter::read(&self.memory, self.eip, 1);
                    let jump = Parameter::read(&self.memory, self.eip, 2);

                    if test.load(&self.memory, self.rbo) != 0 {
                        self.eip = jump.load(&self.memory, self.rbo) as usize;
                    } else {
                        self.eip += 3;
                    }
                }
                6 => {
                    let test = Parameter::read(&self.memory, self.eip, 1);
                    let jump = Parameter::read(&self.memory, self.eip, 2);

                    if test.load(&self.memory, self.rbo) == 0 {
                        self.eip = jump.load(&self.memory, self.rbo) as usize;
                    } else {
                        self.eip += 3;
                    }
                }
                7 => {
                    let lhs = Parameter::read(&self.memory, self.eip, 1);
                    let rhs = Parameter::read(&self.memory, self.eip, 2);
                    let out = Parameter::read(&self.memory, self.eip, 3);
                    out.store(if lhs.load(&self.memory, self.rbo) < rhs.load(&self.memory, self.rbo) { 1 } else { 0 }, &mut self.memory, self.rbo);
                    self.eip += 4;
                }
                8 => {
                    let lhs = Parameter::read(&self.memory, self.eip, 1);
                    let rhs = Parameter::read(&self.memory, self.eip, 2);
                    let out = Parameter::read(&self.memory, self.eip, 3);
                    out.store(if lhs.load(&self.memory, self.rbo) == rhs.load(&self.memory, self.rbo) { 1 } else { 0 }, &mut self.memory, self.rbo);
                    self.eip += 4;
                }
                9 => {
                    let val = Parameter::read(&self.memory, self.eip, 1);
                    self.rbo = ((self.rbo as i64) + val.load(&self.memory, self.rbo)) as usize;
                    self.eip += 2;
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
            memory: Memory(self.0.clone()),
            eip: 0,
            rbo: 0,
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Clone, Copy, PartialEq)]
enum Turn {
    Left,
    Right,
}

impl From<i64> for Turn {
    fn from(v: i64) -> Turn {
        match v {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("Invalid turn: {}", v),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Color {
    Black,
    White,
}

impl From<i64> for Color {
    fn from(v: i64) -> Color {
        match v {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Invalid color: {}", v),
        }
    }
}

impl From<Color> for i64 {
    fn from(v: Color) -> i64 {
        match v {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

struct HullPaintingRobot {
    direction: Direction,
    position: (isize, isize),
}

impl HullPaintingRobot {
    fn new() -> HullPaintingRobot {
        HullPaintingRobot { direction: Direction::Up, position: (0, 0) }
    }

    fn step(&mut self, turn: Turn) {
        match (self.direction, turn) {
            (Direction::Up, Turn::Left) => { self.direction = Direction::Left; self.position.0 -= 1; },
            (Direction::Up, Turn::Right) => { self.direction = Direction::Right; self.position.0 += 1; },
            (Direction::Left, Turn::Left) => { self.direction = Direction::Down; self.position.1 -= 1; },
            (Direction::Left, Turn::Right) => { self.direction = Direction::Up; self.position.1 += 1; },
            (Direction::Down, Turn::Left) => { self.direction = Direction::Right; self.position.0 += 1; },
            (Direction::Down, Turn::Right) => { self.direction = Direction::Left; self.position.0 -= 1; },
            (Direction::Right, Turn::Left) => { self.direction = Direction::Up; self.position.1 += 1; },
            (Direction::Right, Turn::Right) => { self.direction = Direction::Down; self.position.1 -= 1; },
        }
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let program = input.parse::<Program>()?;
    let mut process = program.spawn();
    let mut panels = HashMap::<(isize, isize), Color>::new();
    let mut robot = HullPaintingRobot::new();

    loop {
        match process.run() {
            ProcessRunResult::Complete => break,
            ProcessRunResult::WouldBlock => {},
        }

        while let Some(paint) = process.read() {
            panels.insert(robot.position, paint.into());
            robot.step(process.read().unwrap().into());
        }

        process.feed((*panels.get(&robot.position).unwrap_or(&Color::Black)).into());
    }

    Ok(panels.len())
}

#[cfg(test)]
mod test {
    use super::Program;

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

        while let Some(foo) = process.read() {
            output.push(foo);
        }

        output
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
    fn day9_part1() {
        assert_eq!(run("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99", vec![]), vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);
        assert_eq!(run("1102,34915192,34915192,7,4,7,99,0", vec![]), vec![34915192 * 34915192]);
        assert_eq!(run("104,1125899906842624,99", vec![]), vec![1125899906842624]);
    }
}
