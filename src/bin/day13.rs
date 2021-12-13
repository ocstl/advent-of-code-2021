use std::fmt;
use std::str::FromStr;

const FILE: &str = "inputs/day13.txt";
const DOT: char = '#';
const EMPTY: char = '.';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = input.split_once(',') {
            let x = x
                .parse()
                .map_err(|e: std::num::ParseIntError| e.to_string())?;
            let y = y
                .parse()
                .map_err(|e: std::num::ParseIntError| e.to_string())?;
            Ok(Position::new(x, y))
        } else {
            Err(input.to_string())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some((d, location)) = input
            .split_whitespace()
            .last()
            .and_then(|i| i.split_once('='))
        {
            let location = location
                .parse()
                .map_err(|e: std::num::ParseIntError| e.to_string())?;
            match d {
                "x" => Ok(Fold::Vertical(location)),
                "y" => Ok(Fold::Horizontal(location)),
                _ => Err(input.to_string()),
            }
        } else {
            Err(input.to_string())
        }
    }
}

#[derive(Debug, Clone, Default)]
struct TransparentPaper(Vec<Vec<bool>>);

impl TransparentPaper {
    fn nbr_dots(&self) -> usize {
        self.0
            .iter()
            .flat_map(|row| row.iter())
            .filter(|b| **b)
            .count()
    }

    fn apply_fold(&mut self, fold: Fold) -> &Self {
        match fold {
            Fold::Horizontal(location) => self.horizontal_fold(location),
            Fold::Vertical(location) => self.vertical_fold(location),
        }
    }

    fn horizontal_fold(&mut self, location: usize) -> &Self {
        let below = self.0.split_off(location);
        for (above, below) in self.0.iter_mut().rev().zip(below.iter().skip(1)) {
            for (a, b) in above.iter_mut().zip(below.iter()) {
                *a |= b;
            }
        }

        self
    }

    fn vertical_fold(&mut self, location: usize) -> &Self {
        for row in self.0.iter_mut() {
            let right = row.split_off(location);
            for (l, r) in row.iter_mut().rev().zip(right.iter().skip(1)) {
                *l |= r;
            }
        }

        self
    }
}

impl fmt::Display for TransparentPaper {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for line in &self.0 {
            writeln!(
                f,
                "{}",
                line.iter()
                    .map(|&v| if v { DOT } else { EMPTY })
                    .collect::<String>()
            )?;
        }

        Ok(())
    }
}

impl<T: AsRef<[Position]>> From<T> for TransparentPaper {
    fn from(dots: T) -> Self {
        let dots = dots.as_ref();
        let max_x = dots.iter().map(|p| p.x).max().unwrap_or_default();
        let max_y = dots.iter().map(|p| p.y).max().unwrap_or_default();

        let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
        for &Position { x, y } in dots {
            grid[y][x] = true;
        }

        TransparentPaper(grid)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut lines = input.lines();

    let dots: Vec<Position> = lines
        .by_ref()
        .map_while(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.parse())
            }
        })
        .collect::<Result<_, _>>()?;
    let instructions: Vec<Fold> = lines.map(|line| line.parse()).collect::<Result<_, _>>()?;
    let mut paper = TransparentPaper::from(&dots);

    // How many dots are visible after completing just the first fold
    // instruction on your transparent paper?
    let part1 = paper.apply_fold(instructions[0]).nbr_dots();
    println!("Part 1: {}", part1);

    // What code do you use to activate the infrared thermal imaging camera
    // system?
    for &instruction in &instructions[1..] {
        paper.apply_fold(instruction);
    }
    println!("{}", paper);

    Ok(())
}
