use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::aoc;

#[derive(Clone, Copy, PartialEq)]
pub enum Pixel {
    Black,
    White,
    Transparent,
}

impl FromStr for Pixel {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Pixel, ParseIntError> {
        match u8::from_str(s) {
            Ok(0) => Ok(Pixel::Black),
            Ok(1) => Ok(Pixel::White),
            Ok(2) => Ok(Pixel::Transparent),
            Ok(_) => panic!("Invalid pixel"),
            Err(err) => Err(err),
        }
    }
}

pub struct Layer {
    pixels: Vec<Pixel>
}

impl Layer {
    pub fn count_pixels(&self, pixel: Pixel) -> usize {
        self.pixels.iter().filter(|px| **px == pixel).count()
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;

        for row in self.pixels.chunks(25) {
            for px in row.iter() {
                match px {
                    Pixel::Black => write!(f, "█")?,
                    Pixel::White => write!(f, " ")?,
                    Pixel::Transparent => write!(f, "▒")?,
                }
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}

struct Image {
    layers: Vec<Layer>
}

impl Image {
    fn flatten(&self) -> Layer {
        let mut result = vec![Pixel::Transparent; 25 * 6];

        for layer in &self.layers {
            for (i, pixel) in layer.pixels.iter().enumerate() {
                if result[i] == Pixel::Transparent {
                    result[i] = *pixel;
                }
            }
        }

        Layer { pixels: result }
    }
}

impl FromStr for Image {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Image, ParseIntError> {
        let data = s.chars().map(|c| Pixel::from_str(&c.to_string())).collect::<Result<Vec<_>, _>>()?;
        let layers = data.chunks(25 * 6).map(|chunk| Layer { pixels: chunk.to_owned() }).collect();

        Ok(Image { layers })
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let img = input.parse::<Image>()?;
    let layer = img.layers.iter().min_by_key(|layer| layer.count_pixels(Pixel::Black)).unwrap();

    Ok(layer.count_pixels(Pixel::White) * layer.count_pixels(Pixel::Transparent))
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> Result<Layer, ParseIntError> {
    input.parse::<Image>().map(|img| img.flatten())
}
