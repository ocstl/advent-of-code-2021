use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

const FILE: &str = "inputs/day15.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
struct Cave {
    map: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl Cave {
    const MAX_RISK: u32 = 9;

    fn get(&self, p: Position) -> Option<&u32> {
        self.map.get(p.y).and_then(|row| row.get(p.x))
    }

    fn start_position(&self) -> Position {
        Position::new(0, 0)
    }

    fn end_position(&self) -> Position {
        Position::new(self.width - 1, self.height - 1)
    }

    fn lowest_risk_path(&self, start: Position, end: Position) -> u32 {
        let mut to_visit: BinaryHeap<(Reverse<u32>, Position)> = BinaryHeap::new();
        to_visit.push((Reverse(0), start));
        let mut visited: HashSet<Position> = HashSet::new();

        while let Some((Reverse(level), p)) = to_visit.pop() {
            if p == end {
                return level;
            }

            let next_positions = p
                .adjacent_positions()
                .into_iter()
                .flatten()
                .filter_map(|new_p| {
                    if visited.insert(new_p) {
                        self.get(new_p).map(|risk| (Reverse(level + risk), new_p))
                    } else {
                        None
                    }
                });
            to_visit.extend(next_positions);
        }
        {
            unreachable!()
        }
    }

    fn expand(&self) -> Self {
        let height = 5 * self.height;
        let width = 5 * self.width;
        let map = self
            .map
            .iter()
            .cycle()
            .take(height)
            .enumerate()
            .map(|(idy, row)| {
                let vertical_increment = (idy / self.height) as u32;
                row.iter()
                    .cycle()
                    .take(width)
                    .enumerate()
                    .map(|(idx, level)| {
                        let new_level = level + vertical_increment + (idx / self.width) as u32;
                        if new_level > Self::MAX_RISK {
                            new_level - Self::MAX_RISK
                        } else {
                            new_level
                        }
                    })
                    .collect()
            })
            .collect();

        Cave { map, height, width }
    }
}

impl FromStr for Cave {
    type Err = &'static str;

    fn from_str(map: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<u32>> = map
            .lines()
            .map(|line| line.trim().bytes().map(|h| u32::from(h - b'0')).collect())
            .collect();
        let height = map.len();
        let width = map.first().map(Vec::len).unwrap_or_default();

        Ok(Cave { map, height, width })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let risk_levels: Cave = std::fs::read_to_string(FILE)?.parse()?;
    let start = risk_levels.start_position();
    let end = risk_levels.end_position();

    // What is the lowest total risk of any path from the top left to the
    // bottom right?
    let part1 = risk_levels.lowest_risk_path(start, end);
    println!("Part 1: {}", part1);

    // Using the full map, what is the lowest total risk of any path from the
    // top left to the bottom right?
    let full_map = risk_levels.expand();
    let part2 = full_map.lowest_risk_path(full_map.start_position(), full_map.end_position());
    println!("Part 2: {}", part2);

    Ok(())
}
