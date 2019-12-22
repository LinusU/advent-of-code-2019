use std::f64::consts::{FRAC_PI_2, PI};
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

    fn best_location(&self) -> ((usize, usize), usize) {
        let asteroids = self.asteroids();
        let mut result = ((0, 0), 0);

        for asteroid in &asteroids {
            let mut reachable = 0;

            for other in &asteroids {
                if asteroid == other { continue; }

                if can_see_asteroid(&self, *asteroid, *other) {
                    reachable += 1;
                }
            }

            if reachable > result.1 {
                result = (*asteroid, reachable);
            }
        }

        result
    }

    fn blast_asteroid(&mut self, pos: (usize, usize)) {
        self.0[pos.1][pos.0] = Position::Empty;
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
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let map = input.parse::<Map>()?;

    Ok(map.best_location().1)
}

fn direction(base: (usize, usize), pos: (usize, usize)) -> f64 {
    let delta = (base.0 as isize - pos.0 as isize, base.1 as isize - pos.1 as isize);
    let atan2 = f64::atan2(delta.1 as f64, delta.0 as f64);

    if atan2 < FRAC_PI_2 { atan2 + FRAC_PI_2 + PI } else { atan2 - FRAC_PI_2 }
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> Result<usize, ParseIntError> {
    let mut map = input.parse::<Map>()?;
    let base = map.best_location().0;
    let asteroids = map.asteroids();

    let mut destroyed = 0;
    let mut queue = asteroids.iter().filter(|pos| **pos != base).map(|pos| (*pos, direction(base, *pos))).collect::<Vec<_>>();
    let mut result = Option::<usize>::None;

    queue.sort_by(|(_, lhs), (_, rhs)| f64::partial_cmp(lhs, rhs).unwrap());

    while result == None {
        assert!(queue.len() > 0);

        let mut marked = Vec::<(usize, usize)>::new();

        queue.retain(|(pos, _)| {
            if !can_see_asteroid(&map, base, *pos) { return true; }

            marked.push(*pos);
            destroyed += 1;

            if destroyed == 200 {
                result = Some(pos.0 * 100 + pos.1);
            }

            false
        });

        for asteroid in marked {
            map.blast_asteroid(asteroid);
        }
    }

    Ok(result.unwrap())
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
    fn direction() {
        use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

        assert_eq!(super::direction((1, 1), (1, 0)), 0f64);
        assert_eq!(super::direction((1, 1), (2, 0)), FRAC_PI_4);
        assert_eq!(super::direction((1, 1), (2, 1)), FRAC_PI_2);
        assert_eq!(super::direction((1, 1), (2, 2)), FRAC_PI_2 + FRAC_PI_4);
        assert_eq!(super::direction((1, 1), (1, 2)), PI);
        assert_eq!(super::direction((1, 1), (0, 2)), PI + FRAC_PI_4);
        assert_eq!(super::direction((1, 1), (0, 1)), PI + FRAC_PI_2);
        assert_eq!(super::direction((1, 1), (0, 0)), PI + FRAC_PI_2 + FRAC_PI_4);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(".#..#\n.....\n#####\n....#\n...##"), Ok(8));
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##"), Ok(802));
    }
}
