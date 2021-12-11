use std::collections::HashSet;
use std::str::FromStr;

const FILE: &str = "inputs/day11.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn adjacent_positions(self) -> [Option<Self>; 8] {
        [
            self.y
                .checked_sub(1)
                .and_then(|y| self.x.checked_sub(1).map(|x| Position::new(x, y))),
            self.y
                .checked_sub(1)
                .and_then(|y| self.x.checked_sub(0).map(|x| Position::new(x, y))),
            self.y
                .checked_sub(1)
                .and_then(|y| self.x.checked_add(1).map(|x| Position::new(x, y))),
            self.y
                .checked_sub(0)
                .and_then(|y| self.x.checked_sub(1).map(|x| Position::new(x, y))),
            self.y
                .checked_sub(0)
                .and_then(|y| self.x.checked_add(1).map(|x| Position::new(x, y))),
            self.y
                .checked_add(1)
                .and_then(|y| self.x.checked_sub(1).map(|x| Position::new(x, y))),
            self.y
                .checked_add(1)
                .and_then(|y| self.x.checked_sub(0).map(|x| Position::new(x, y))),
            self.y
                .checked_add(1)
                .and_then(|y| self.x.checked_add(1).map(|x| Position::new(x, y))),
        ]
    }
}

#[derive(Debug, Clone)]
struct DumboOctopusGrid([u8; 100]);

impl DumboOctopusGrid {
    const HEIGHT: usize = 10;
    const WIDTH: usize = 10;

    fn index_to_position(idx: usize) -> Position {
        Position::new(idx % Self::WIDTH, idx / Self::WIDTH)
    }

    fn position_to_index(position: Position) -> Option<usize> {
        if position.x < Self::WIDTH && position.y < Self::HEIGHT {
            Some(position.y * Self::WIDTH + position.x)
        } else {
            None
        }
    }

    fn nbr_flashes(&self) -> usize {
        self.0.iter().filter(|o| **o == 0).count()
    }
}

impl Iterator for DumboOctopusGrid {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment the energy level. Grab those that will initially flash at
        // the same time.
        let mut flashers: Vec<usize> = self
            .0
            .iter_mut()
            .enumerate()
            .filter_map(|(idx, o)| {
                *o += 1;
                if *o > 9 {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        let mut flashed = HashSet::new();
        while let Some(idx) = flashers.pop() {
            if flashed.insert(idx) {
                let targets = Self::index_to_position(idx)
                    .adjacent_positions()
                    .into_iter()
                    .flatten()
                    .filter_map(Self::position_to_index);

                for target in targets {
                    self.0[target] += 1;
                    if self.0[target] > 9 {
                        flashers.push(target);
                    }
                }
            }
        }

        for o in self.0.iter_mut() {
            if *o > 9 {
                *o = 0;
            }
        }

        Some(self.clone())
    }
}

impl FromStr for DumboOctopusGrid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = [0; 100];
        for (p, v) in grid.iter_mut().zip(
            input
                .lines()
                .flat_map(|line| line.trim().bytes())
                .map(|b| b - b'0'),
        ) {
            *p = v;
        }

        Ok(DumboOctopusGrid(grid))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grid: DumboOctopusGrid = std::fs::read_to_string(FILE)?.parse().unwrap();

    // Given the starting energy levels of the dumbo octopuses in your cavern,
    // simulate 100 steps. How many total flashes are there after 100 steps?
    let part1: usize = grid.clone().take(100).map(|g| g.nbr_flashes()).sum();
    println!("Part 1: {}", part1);

    // If you can calculate the exact moments when the octopuses will all flash
    // simultaneously, you should be able to navigate through the cavern. What
    // is the first step during which all octopuses flash?
    // Add 1 because it is 0-indexed.
    let part2: usize = grid.position(|g| g.nbr_flashes() == 100).unwrap() + 1;
    println!("Part 2: {}", part2);

    Ok(())
}
