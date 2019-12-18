use std::collections::HashMap;
use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[derive(Clone, Copy)]
struct Body {
    orbits: u64,
}

impl Body {
    fn child(self) -> Body {
        Body { orbits: self.orbits + 1 }
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let mut bodies = HashMap::<&str, Body>::new();

    bodies.insert("COM", Body { orbits: 0 });

    let mut later = Vec::<Vec<&str>>::new();

    for line in input.split_whitespace() {
        let parts: Vec<&str> = line.split(')').collect();

        if bodies.contains_key(parts[0]) {
            let body = bodies[parts[0]].child();
            bodies.insert(parts[1], body);
        } else {
            later.push(parts);
        }
    }

    while later.len() > 0 {
        let current = later;
        later = Vec::<Vec<&str>>::new();

        for parts in current {
            if bodies.contains_key(parts[0]) {
                let body = bodies[parts[0]].child();
                bodies.insert(parts[1], body);
            } else {
                later.push(parts);
            }
        }
    }

    Ok(bodies.values().map(|b| b.orbits).sum())
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        assert_eq!(super::part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"), Ok(42));
        assert_eq!(super::part1("E)J\nJ)K\nC)D\nG)H\nD)E\nB)G\nCOM)B\nD)I\nB)C\nK)L\nE)F"), Ok(42));
    }
}
