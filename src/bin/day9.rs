use std::cmp::Reverse;
use std::collections::HashSet;
use std::str::FromStr;

const FILE: &str = "inputs/day9.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn adjacent_positions(self) -> [Option<Self>; 4] {
        [
            self.y.checked_sub(1).map(|y| Position::new(self.x, y)),
            Some(Position::new(self.x, self.y + 1)),
            self.x.checked_sub(1).map(|x| Position::new(x, self.y)),
            Some(Position::new(self.x + 1, self.y)),
        ]
    }
}

#[derive(Debug, Clone)]
struct HeightMap {
    map: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl HeightMap {
    fn get(&self, x: usize, y: usize) -> Option<&u32> {
        self.map.get(y).and_then(|row| row.get(x))
    }

    fn adjacent_locations(&self, x: usize, y: usize) -> [Option<&u32>; 4] {
        let up = if y == 0 { None } else { self.get(x, y - 1) };
        let down = if y == self.height {
            None
        } else {
            self.get(x, y + 1)
        };
        let left = if x == 0 { None } else { self.get(x - 1, y) };
        let right = if x == self.width {
            None
        } else {
            self.get(x + 1, y)
        };

        [up, down, left, right]
    }

    fn low_points(&self) -> impl Iterator<Item = &u32> {
        self.map.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if self
                    .adjacent_locations(x, y)
                    .into_iter()
                    .flatten()
                    .all(|h| h > c)
                {
                    Some(c)
                } else {
                    None
                }
            })
        })
    }

    fn basins(&self) -> Vec<HashSet<Position>> {
        let mut basins = Vec::new();
        // Add in the position with height 9 to speed things up.
        let mut visited: HashSet<Position> = self
            .map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, h)| {
                    if *h == 9 {
                        Some(Position::new(x, y))
                    } else {
                        None
                    }
                })
            })
            .collect();

        for y in 0..self.height {
            for x in 0..self.width {
                let p = Position::new(x, y);
                if visited.contains(&p) {
                    continue;
                }

                let mut basin = HashSet::new();
                let mut to_visit = vec![p];
                while let Some(p) = to_visit.pop() {
                    basin.insert(p);
                    to_visit.extend(
                        p.adjacent_positions()
                            .into_iter()
                            .flatten()
                            .filter(|new_p| {
                                visited.insert(*new_p) && self.get(new_p.x, new_p.y).is_some()
                            }),
                    )
                }

                basins.push(basin);
            }
        }

        basins
    }
}

impl FromStr for HeightMap {
    type Err = &'static str;

    fn from_str(map: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<u32>> = map
            .lines()
            .map(|line| line.trim().bytes().map(|h| u32::from(h - b'0')).collect())
            .collect();
        let height = map.len();
        let width = map.first().map(Vec::len).unwrap_or_default();

        Ok(HeightMap { map, height, width })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let height_map: HeightMap = std::fs::read_to_string(FILE)?.parse()?;

    // Find all of the low points on your heightmap. What is the sum of the
    // risk levels of all low points on your heightmap?
    let part1: u32 = height_map.low_points().map(|l| l + 1).sum();
    println!("Part 1: {}", part1);

    // What do you get if you multiply together the sizes of the three largest
    // basins?
    let mut basins = height_map.basins();
    basins.sort_by_key(|basin| Reverse(basin.len()));
    let part2: usize = basins[0..3].iter().map(|basin| basin.len()).product();
    println!("Part 2: {}", part2);

    Ok(())
}
