use std::num::ParseIntError;

use aoc_runner_derive::aoc;

fn password_is_valid(password: u64) -> bool {
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

#[aoc(day4, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let bounds = input.split('-')
        .map(|mass| mass.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut count = 0;

    for password in bounds[0]..=bounds[1] {
        if password_is_valid(password) {
            count += 1;
        }
    }

    Ok(count)
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
}
