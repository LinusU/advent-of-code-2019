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
    fn count_valid_passwords(&self, test: &dyn Fn(Vec<u64>) -> bool) -> usize {
        (self.lo..=self.hi).filter(|n| test(number_to_digits(*n))).count()
    }
}

fn number_to_digits(num: u64) -> Vec<u64> {
    let count = ((num as f32).log10() as usize) + 1;
    let mut digits = Vec::with_capacity(count);
    let mut num = num;

    while num > 9 {
        digits.push(num % 10);
        num = num / 10;
    }

    digits.push(num);
    digits.reverse();

    assert_eq!(digits.len(), count);

    digits
}

fn password_is_valid_v1(digits: Vec<u64>) -> bool {
    let has_two_adjecent = digits.windows(2).any(|window| window[0] == window[1]);
    let never_decreases = digits.windows(2).all(|window| window[0] <= window[1]);

    has_two_adjecent && never_decreases
}

fn has_exactly_two_adjecent(digits: Vec<u64>) -> bool {
    let mut tally = 1;
    let mut last = digits[0];

    for digit in &digits[1..] {
        if *digit == last {
            tally += 1;
        } else if tally == 2 {
            return true;
        } else {
            last = *digit;
            tally = 1;
        }
    }

    tally == 2
}

fn password_is_valid_v2(digits: Vec<u64>) -> bool {
    let never_decreases = digits.windows(2).all(|window| window[0] <= window[1]);

    has_exactly_two_adjecent(digits) && never_decreases
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
    fn number_to_digits() {
        assert_eq!(super::number_to_digits(123456), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(super::number_to_digits(937593), vec![9, 3, 7, 5, 9, 3]);
    }

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
