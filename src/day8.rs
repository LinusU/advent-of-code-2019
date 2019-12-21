use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::aoc;

struct Layer {
    pixels: Vec<u8>
}

impl Layer {
    pub fn count_digits(&self, digit: u8) -> usize {
        self.pixels.iter().filter(|px| **px == digit).count()
    }
}

struct Image {
    layers: Vec<Layer>
}

impl FromStr for Image {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Image, ParseIntError> {
        let data = s.chars().map(|c| u8::from_str(&c.to_string())).collect::<Result<Vec<_>, _>>()?;
        let layers = data.chunks(25 * 6).map(|chunk| Layer { pixels: chunk.to_owned() }).collect();

        Ok(Image { layers })
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let img = input.parse::<Image>()?;
    let layer = img.layers.iter().min_by_key(|layer| layer.count_digits(0)).unwrap();

    Ok(layer.count_digits(1) * layer.count_digits(2))
}
