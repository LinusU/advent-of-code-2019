use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::aoc;

fn gcd(mut a: usize, mut b: usize) -> usize {
    assert!(a != 0 || b != 0);

    if a == 0 { return b; }
    if b == 0 { return a; }

    let mut r;
    while b != 0 {
        r = a % b;
        a = b;
        b = r;
    }

    a
}

#[derive(PartialEq)]
enum Position {
    Empty,
    Asteroid,
}

impl Position {
    fn has_asteroid(&self) -> bool {
        self == &Position::Asteroid
    }
}

impl Position {
    fn from_char(c: char) -> Position {
        match c {
            '.' => Position::Empty,
            '#' => Position::Asteroid,
            _ => panic!("Invalid position: {}", c),
        }
    }
}

struct Map(Vec<Vec<Position>>);

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Map, ParseIntError> {
        Ok(Map(s.split_whitespace().map(|line| line.chars().map(Position::from_char).collect()).collect()))
    }
}

impl Map {
    fn asteroids(&self) -> Vec<(usize, usize)> {
        self.0.iter().enumerate().flat_map(|(y, row)| row.iter().enumerate().filter(|(_, p)| p.has_asteroid()).map(move |(x, _)| (x, y))).collect()
    }

    fn has_asteroid_at(&self, pos: (usize, usize)) -> bool {
        self.0[pos.1][pos.0].has_asteroid()
    }
}

fn intermidiates(lhs: (usize, usize), rhs: (usize, usize)) -> Vec<(usize, usize)> {
    assert_ne!(lhs, rhs);

    let delta = (rhs.0 as isize - lhs.0 as isize, rhs.1 as isize - lhs.1 as isize);
    let divider = gcd(delta.0.abs() as usize, delta.1.abs() as usize);
    let step = (delta.0 / (divider as isize), delta.1 / (divider as isize));

    (1..divider).map(|i| ((lhs.0 as isize + step.0 * i as isize) as usize, (lhs.1 as isize + step.1 * i as isize) as usize)).collect()
}

fn can_see_asteroid(map: &Map, lhs: (usize, usize), rhs: (usize, usize)) -> bool {
    intermidiates(lhs, rhs).iter().all(|pos| !map.has_asteroid_at(*pos))
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let map = input.parse::<Map>()?;
    let asteroids = map.asteroids();

    let mut max_reachable = 0;

    for asteroid in &asteroids {
        let mut reachable = 0;

        for other in &asteroids {
            if asteroid == other { continue; }

            if can_see_asteroid(&map, *asteroid, *other) {
                reachable += 1;
            }
        }

        if reachable > max_reachable {
            max_reachable = reachable;
        }
    }

    Ok(max_reachable)
}

#[cfg(test)]
mod test {
    #[test]
    fn intermidiates() {
        assert_eq!(super::intermidiates((0, 0), (2, 2)), vec![(1, 1)]);
        assert_eq!(super::intermidiates((5, 5), (7, 7)), vec![(6, 6)]);
        assert_eq!(super::intermidiates((5, 5), (9, 7)), vec![(7, 6)]);
        assert_eq!(super::intermidiates((5, 5), (13, 9)), vec![(7, 6), (9, 7), (11, 8)]);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(".#..#\n.....\n#####\n....#\n...##"), Ok(8));
    }
}
