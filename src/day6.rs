use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::aoc;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Id(u32);

impl Id {
    fn new(name: &str) -> Id {
        assert!(name.len() <= 4);
        assert!(name.is_ascii());
        Id(name.as_bytes().iter().fold(0, |mem, byte| mem << 8 | (*byte as u32)))
    }
}

struct Map {
    orbits: HashMap::<Id, Id>
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Map, ParseIntError> {
        let mut orbits = HashMap::new();

        for line in s.split_whitespace() {
            let parts: Vec<&str> = line.split(')').collect();
            orbits.insert(Id::new(parts[1]), Id::new(parts[0]));
        }

        Ok(Map { orbits })
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let map = input.parse::<Map>()?;
    let mut orbit_counts = HashMap::<Id, usize>::new();

    orbit_counts.insert(Id::new("COM"), 0);

    fn get_count(orbit_counts: &mut HashMap<Id, usize>, orbits: &HashMap<Id, Id>, id: Id) -> usize {
        match orbit_counts.get(&id) {
            Some(count) => *count,
            None => {
                let count = get_count(orbit_counts, orbits, orbits[&id]) + 1;
                orbit_counts.insert(id, count);
                count
            }
        }
    }

    Ok(map.orbits.keys().map(|k| get_count(&mut orbit_counts, &map.orbits, *k)).sum())
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        assert_eq!(super::part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"), Ok(42));
        assert_eq!(super::part1("E)J\nJ)K\nC)D\nG)H\nD)E\nB)G\nCOM)B\nD)I\nB)C\nK)L\nE)F"), Ok(42));
    }
}
