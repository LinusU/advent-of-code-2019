use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::aoc;

struct Bounds {
    lo: u64,
    hi: u64,
}

impl FromStr for Bounds {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-').map(|n| n.parse::<u64>());

        Ok(Bounds { lo: parts.next().unwrap()?, hi: parts.next().unwrap()? })
    }
}

impl Bounds {
    fn count_valid_passwords(&self, test: &dyn Fn(u64) -> bool) -> usize {
        (self.lo..=self.hi).filter(|n| test(*n)).count()
    }
}

fn password_is_valid_v1(password: u64) -> bool {
    let digit0 = (password / 100_000) % 10;
    let digit1 = (password / 10_000) % 10;
    let digit2 = (password / 1_000) % 10;
    let digit3 = (password / 100) % 10;
    let digit4 = (password / 10) % 10;
    let digit5 = password % 10;

    let has_two_adjecent = (digit0 == digit1) || (digit1 == digit2) || (digit2 == digit3) || (digit3 == digit4) || (digit4 == digit5);
    let never_decreases = (digit0 <= digit1) && (digit1 <= digit2) && (digit2 <= digit3) && (digit3 <= digit4) && (digit4 <= digit5);

    has_two_adjecent && never_decreases
}

fn password_is_valid_v2(password: u64) -> bool {
    let digit0 = (password / 100_000) % 10;
    let digit1 = (password / 10_000) % 10;
    let digit2 = (password / 1_000) % 10;
    let digit3 = (password / 100) % 10;
    let digit4 = (password / 10) % 10;
    let digit5 = password % 10;

    let has_two_adjecent = (digit0 == digit1 && digit1 != digit2) || (digit0 != digit1 && digit1 == digit2 && digit2 != digit3) || (digit1 != digit2 && digit2 == digit3 && digit3 != digit4) || (digit2 != digit3 && digit3 == digit4 && digit4 != digit5) || (digit3 != digit4 && digit4 == digit5);
    let never_decreases = (digit0 <= digit1) && (digit1 <= digit2) && (digit2 <= digit3) && (digit3 <= digit4) && (digit4 <= digit5);

    has_two_adjecent && never_decreases
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let bounds: Bounds = input.parse()?;

    Ok(bounds.count_valid_passwords(&password_is_valid_v1) as u64)
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let bounds: Bounds = input.parse()?;

    Ok(bounds.count_valid_passwords(&password_is_valid_v2) as u64)
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1_should_give_0() {
        assert_eq!(super::part1("111110-111110"), Ok(0));
    }

    #[test]
    fn part_1_should_give_1() {
        assert_eq!(super::part1("111111-111111"), Ok(1));
    }

    #[test]
    fn part_1_should_give_3() {
        assert_eq!(super::part1("111111-111113"), Ok(3));
    }

    #[test]
    fn part_2_should_give_0() {
        assert_eq!(super::part2("123444-123444"), Ok(0));
    }

    #[test]
    fn part_2_should_give_1() {
        assert_eq!(super::part2("112233-112233"), Ok(1));
    }

    #[test]
    fn part_2_should_give_22() {
        assert_eq!(super::part2("111122-111322"), Ok(22));
    }
}
