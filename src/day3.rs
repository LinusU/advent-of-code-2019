use std::iter::IntoIterator;
use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::aoc;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    const ZERO: Point = Point { x: 0, y: 0 };

    pub const fn manhattan_distance(self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

enum Instruction {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            'U' => s[1..].parse().map(Instruction::Up),
            'D' => s[1..].parse().map(Instruction::Down),
            'L' => s[1..].parse().map(Instruction::Left),
            'R' => s[1..].parse().map(Instruction::Right),
            _ => unreachable!(),
        }
    }
}

struct InstructionSet(Vec<Instruction>);

impl InstructionSet {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for InstructionSet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',').map(|i| Instruction::from_str(i)).collect::<Result<Vec<_>, _>>().map(InstructionSet)
    }
}

impl IntoIterator for InstructionSet {
    type Item = Instruction;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

struct ProgramInput {
    first: InstructionSet,
    second: InstructionSet,
}

impl FromStr for ProgramInput {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        Ok(ProgramInput {
            first: iter.next().unwrap().parse()?,
            second: iter.next().unwrap().parse()?,
        })
    }
}

#[derive(Clone, Copy)]
enum PathSegmentDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Copy)]
enum PathSegment {
    Horizontal { y: isize, x_lo: isize, x_hi: isize, dir: PathSegmentDirection },
    Vertical { x: isize, y_lo: isize, y_hi: isize, dir: PathSegmentDirection },
}

impl PathSegment {
    pub fn len(self) -> usize {
        match self {
            PathSegment::Horizontal { x_lo, x_hi, .. } => (x_hi - x_lo) as usize,
            PathSegment::Vertical { y_lo, y_hi, .. } => (y_hi - y_lo) as usize,
        }
    }

    pub fn offset_of(self, point: Point) -> usize {
        match self {
            PathSegment::Horizontal { dir: PathSegmentDirection::Ascending, x_lo, .. } => (point.x - x_lo) as usize,
            PathSegment::Horizontal { dir: PathSegmentDirection::Descending, x_hi, .. } => (x_hi - point.x) as usize,
            PathSegment::Vertical { dir: PathSegmentDirection::Ascending, y_lo, .. } => (point.y - y_lo) as usize,
            PathSegment::Vertical { dir: PathSegmentDirection::Descending, y_hi, .. } => (y_hi - point.y) as usize,
        }
    }

    pub fn intersection(self, other: PathSegment) -> Option<Point> {
        match (self, other) {
            (PathSegment::Horizontal { y, x_lo, x_hi, .. }, PathSegment::Vertical { y_lo, y_hi, x, .. }) => {
                if y_lo <= y && y <= y_hi && x_lo <= x && x <= x_hi { Some(Point { x, y }) } else { None }
            }
            (PathSegment::Vertical { x, y_lo, y_hi, .. }, PathSegment::Horizontal { x_lo, x_hi, y, .. }) => {
                if x_lo <= x && x <= x_hi && y_lo <= y && y <= y_hi { Some(Point { x, y }) } else { None }
            }
            _ => None
        }
    }
}

struct Path(Vec<PathSegment>);

impl Path {
    fn iter(&self) -> std::slice::Iter<PathSegment> {
        self.0.iter()
    }
}

impl From<InstructionSet> for Path {
    fn from(input: InstructionSet) -> Self {
        let mut result = Vec::with_capacity(input.len());

        let mut x: isize = 0;
        let mut y: isize = 0;
        for item in input {
            match item {
                Instruction::Up(v) => {
                    result.push(PathSegment::Vertical { x, y_lo: y, y_hi: y + v, dir: PathSegmentDirection::Ascending });
                    y += v;
                }
                Instruction::Down(v) => {
                    result.push(PathSegment::Vertical { x, y_lo: y - v, y_hi: y, dir: PathSegmentDirection::Descending });
                    y -= v;
                }
                Instruction::Left(v) => {
                    result.push(PathSegment::Horizontal { y, x_lo: x - v, x_hi: x, dir: PathSegmentDirection::Descending });
                    x -= v;
                }
                Instruction::Right(v) => {
                    result.push(PathSegment::Horizontal { y, x_lo: x, x_hi: x + v, dir: PathSegmentDirection::Ascending });
                    x += v;
                }
            }
        }

        Path(result)
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let input: ProgramInput = input.parse()?;

    let first_path: Path = input.first.into();
    let second_path: Path = input.second.into();

    let mut min_distance = usize::max_value();

    for segment in first_path.iter() {
        for other in second_path.iter() {
            if let Some(intersection) = segment.intersection(*other) {
                if intersection == Point::ZERO { continue; }
                let distance = intersection.manhattan_distance();
                min_distance = std::cmp::min(min_distance, distance);
            }
        }
    }

    Ok(min_distance as u64)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let input: ProgramInput = input.parse()?;

    let first_path: Path = input.first.into();
    let second_path: Path = input.second.into();

    let mut min_distance = usize::max_value();

    let mut first_len = 0;
    for segment in first_path.iter() {
        let mut second_len = 0;

        for other in second_path.iter() {
            if let Some(intersection) = segment.intersection(*other) {
                if intersection == Point::ZERO { continue; }

                let first_offset = segment.offset_of(intersection);
                let second_offset = other.offset_of(intersection);

                let distance = first_len + first_offset + second_len + second_offset;
                min_distance = std::cmp::min(min_distance, distance);
            }

            second_len += other.len();
        }

        first_len += segment.len();
    }

    Ok(min_distance as u64)
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1_should_give_6() {
        assert_eq!(super::part1("R8,U5,L5,D3\nU7,R6,D4,L4"), Ok(6));
    }

    #[test]
    fn part_1_should_give_159() {
        assert_eq!(super::part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), Ok(159));
    }

    #[test]
    fn part_2_should_give_610() {
        assert_eq!(super::part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), Ok(610));
    }

    #[test]
    fn part_2_should_give_410() {
        assert_eq!(super::part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), Ok(410));
    }
}
