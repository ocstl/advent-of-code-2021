use std::str::FromStr;

const FILE: &str = "inputs/day25.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Empty,
    East,
    South,
}

impl From<char> for Location {
    fn from(c: char) -> Self {
        match c {
            '.' => Location::Empty,
            '>' => Location::East,
            'v' => Location::South,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SeaFloor {
    map: Vec<Vec<Location>>,
    height: usize,
    width: usize,
}

impl SeaFloor {
    fn north(&self, idx: usize, idy: usize) -> Location {
        self.map[(idy + self.height - 1) % self.height][idx]
    }

    fn south(&self, idx: usize, idy: usize) -> Location {
        self.map[(idy + self.height + 1) % self.height][idx]
    }

    fn west(&self, idx: usize, idy: usize) -> Location {
        self.map[idy][(idx + self.width - 1) % self.width]
    }

    fn east(&self, idx: usize, idy: usize) -> Location {
        self.map[idy][(idx + self.width + 1) % self.width]
    }
}

impl FromStr for SeaFloor {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<Location>> = input
            .lines()
            .map(|line| line.trim().chars().map(Location::from).collect())
            .collect();
        let height = map.len();
        let width = map.first().unwrap_or(&Vec::new()).len();
        Ok(SeaFloor { map, height, width })
    }
}

impl Iterator for SeaFloor {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        // Move the east-facing sea cucumbers.
        let mut east: Vec<Vec<Location>> = self
            .map
            .iter()
            .enumerate()
            .map(|(idy, row)| {
                row.iter()
                    .enumerate()
                    .map(|(idx, l)| match l {
                        Location::Empty if self.west(idx, idy) == Location::East => Location::East,
                        Location::East if self.east(idx, idy) == Location::Empty => Location::Empty,
                        l => *l,
                    })
                    .collect()
            })
            .collect();

        // We can use the `north`, `south`, etc. functions of SeaFloor, and
        // keep a version to compare, returning `None` if there hasn't been a
        // change.
        std::mem::swap(&mut east, &mut self.map);

        // Move the south-facing sea cucumbers.
        let south: Vec<Vec<Location>> = self
            .map
            .iter()
            .enumerate()
            .map(|(idy, row)| {
                row.iter()
                    .enumerate()
                    .map(|(idx, l)| match l {
                        Location::Empty if self.north(idx, idy) == Location::South => {
                            Location::South
                        }
                        Location::South if self.south(idx, idy) == Location::Empty => {
                            Location::Empty
                        }
                        l => *l,
                    })
                    .collect()
            })
            .collect();

        self.map = south;

        if self.map == east {
            None
        } else {
            Some(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let sea_floor: SeaFloor = input.parse()?;

    // Find somewhere safe to land your submarine. What is the first step on
    // which no sea cucumbers move?
    let part1 = sea_floor.count() + 1;
    println!("Part 1: {}", part1);

    Ok(())
}
