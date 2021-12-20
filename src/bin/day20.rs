use std::collections::HashSet;
use std::convert::TryFrom;
use std::iter::FromIterator;

const FILE: &str = "inputs/day20.txt";
const LIGHT_PIXEL: char = '#';
const DARK_PIXEL: char = '.';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Dark,
    Light,
}

impl From<char> for Pixel {
    fn from(c: char) -> Pixel {
        match c {
            DARK_PIXEL => Pixel::Dark,
            LIGHT_PIXEL => Pixel::Light,
            _ => unreachable!(),
        }
    }
}

impl From<bool> for Pixel {
    fn from(b: bool) -> Pixel {
        if b {
            Pixel::Light
        } else {
            Pixel::Dark
        }
    }
}

impl From<Pixel> for usize {
    fn from(pixel: Pixel) -> Self {
        match pixel {
            Pixel::Light => 1,
            Pixel::Dark => 0,
        }
    }
}

type ImageEnhancementAlgorithm = [Pixel; 512];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn square(self) -> impl Iterator<Item = Position> {
        (self.y - 1..=self.y + 1)
            .flat_map(move |y| (self.x - 1..=self.x + 1).map(move |x| Position::new(x, y)))
    }
}

#[derive(Debug, Clone)]
struct Image {
    lit_pixels: HashSet<Position>,
    min_y: i32,
    max_y: i32,
    min_x: i32,
    max_x: i32,
    default_pixel: Pixel,
}

impl Image {
    fn nbr_lit_pixels(&self) -> usize {
        self.lit_pixels.len()
    }

    fn is_lit(&self, position: Position) -> Pixel {
        if position.y < self.min_y
            || position.y > self.max_y
            || position.x < self.min_x
            || position.x > self.max_x
        {
            self.default_pixel
        } else {
            Pixel::from(self.lit_pixels.contains(&position))
        }
    }

    fn enhance(&mut self, algorithm: ImageEnhancementAlgorithm) -> &mut Self {
        let lit_pixels = (self.min_y - 1..=self.max_y + 1)
            .flat_map(|y| (self.min_x - 1..=self.max_x + 1).map(move |x| Position::new(x, y)))
            .filter(|p| {
                let idx = p.square().fold(0, |acc, adjacent| {
                    (acc << 1) + usize::from(self.is_lit(adjacent))
                });
                algorithm[idx] == Pixel::Light
            })
            .collect();

        self.lit_pixels = lit_pixels;
        self.min_y -= 1;
        self.max_y += 1;
        self.min_x -= 1;
        self.max_x += 1;
        // We have to keep track of the pixels "outside" (it's infinite) of the
        // current focus.
        self.default_pixel = if self.default_pixel == Pixel::Light {
            algorithm[511]
        } else {
            algorithm[0]
        };

        self
    }
}

impl<'a> FromIterator<&'a str> for Image {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let lit_pixels: HashSet<Position> = iter
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == LIGHT_PIXEL {
                        Some(Position::new(x as i32, y as i32))
                    } else {
                        None
                    }
                })
            })
            .collect();

        let max_y = lit_pixels.iter().map(|p| p.y).max().unwrap();
        let max_x = lit_pixels.iter().map(|p| p.x).max().unwrap();

        Image {
            lit_pixels,
            min_y: 0,
            max_y,
            min_x: 0,
            max_x,
            default_pixel: Pixel::Dark,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut iter = input.lines();
    let algorithm = <ImageEnhancementAlgorithm>::try_from(
        iter.next()
            .unwrap()
            .chars()
            .map(Pixel::from)
            .collect::<Vec<Pixel>>(),
    )
    .unwrap();
    iter.next();

    let mut image: Image = iter.collect();

    // Start with the original input image and apply the image enhancement
    // algorithm twice, being careful to account for the infinite size of the
    // images. How many pixels are lit in the resulting image?
    for _ in 0..2 {
        image.enhance(algorithm);
    }
    let part1 = image.nbr_lit_pixels();
    println!("Part 1: {}", part1);

    // Start again with the original input image and apply the image enhancement
    // algorithm 50 times. How many pixels are lit in the resulting image?
    for _ in 2..50 {
        image.enhance(algorithm);
    }
    let part2 = image.nbr_lit_pixels();
    println!("Part 2: {}", part2);

    Ok(())
}
