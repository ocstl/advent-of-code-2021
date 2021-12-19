use counter::Counter;
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;
use std::str::FromStr;
use Alignment::*;

const FILE: &str = "inputs/day19.txt";
const ALIGNMENTS: [Alignment; 24] = [
    XYZ, XzY, Xyz, XZy, yXZ, ZXY, YXz, zXy, xyZ, xzy, xYz, xZY, YxZ, Zxy, yxz, zxY, zYX, YZX, ZyX,
    yzX, zyx, yZx, ZYx, Yzx,
];

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Alignment {
    XYZ,
    XzY,
    Xyz,
    XZy,
    yXZ,
    ZXY,
    YXz,
    zXy,
    xyZ,
    xzy,
    xYz,
    xZY,
    YxZ,
    Zxy,
    yxz,
    zxY,
    zYX,
    YZX,
    ZyX,
    yzX,
    zyx,
    yZx,
    ZYx,
    Yzx,
}

impl Default for Alignment {
    fn default() -> Self {
        XYZ
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Position { x, y, z }
    }

    fn align(self, alignment: Alignment) -> Position {
        let Self { x, y, z } = self;
        match alignment {
            XYZ => Position::new(x, y, z),
            XzY => Position::new(x, -z, y),
            Xyz => Position::new(x, -y, -z),
            XZy => Position::new(x, z, -y),
            yXZ => Position::new(-y, x, z),
            ZXY => Position::new(z, x, y),
            YXz => Position::new(y, x, -z),
            zXy => Position::new(-z, x, -y),
            xyZ => Position::new(-x, -y, z),
            xzy => Position::new(-x, -z, -y),
            xYz => Position::new(-x, y, -z),
            xZY => Position::new(-x, z, y),
            YxZ => Position::new(y, -x, z),
            Zxy => Position::new(z, -x, -y),
            yxz => Position::new(-y, -x, -z),
            zxY => Position::new(-z, -x, y),
            zYX => Position::new(-z, y, x),
            YZX => Position::new(y, z, x),
            ZyX => Position::new(z, -y, x),
            yzX => Position::new(-y, -z, x),
            zyx => Position::new(-z, -y, -x),
            yZx => Position::new(-y, z, -x),
            ZYx => Position::new(z, y, -x),
            Yzx => Position::new(y, -z, -x),
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Distance;

    fn sub(self, other: Self) -> Self::Output {
        Distance::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(position: &str) -> Result<Self, Self::Err> {
        let mut iter = position.trim().split(',');
        if let (Some(x), Some(y), Some(z)) = (iter.next(), iter.next(), iter.next()) {
            let x = x
                .parse()
                .map_err(|e: std::num::ParseIntError| e.to_string())?;
            let y = y
                .parse()
                .map_err(|e: std::num::ParseIntError| e.to_string())?;
            let z = z
                .parse()
                .map_err(|e: std::num::ParseIntError| e.to_string())?;
            Ok(Position { x, y, z })
        } else {
            Err(format!("Wrong format: {}", position.to_string()))
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Distance {
    dx: i32,
    dy: i32,
    dz: i32,
}

impl Distance {
    fn new(dx: i32, dy: i32, dz: i32) -> Self {
        Distance { dx, dy, dz }
    }

    fn manhattan_distance(self) -> i32 {
        self.dx.abs() + self.dy.abs() + self.dz.abs()
    }

    fn all_alignments(self) -> impl Iterator<Item = (Alignment, Distance)> {
        ALIGNMENTS
            .into_iter()
            .map(move |alignment| (alignment, self.align(alignment)))
    }

    fn align(self, alignment: Alignment) -> Distance {
        let Self {
            dx: x,
            dy: y,
            dz: z,
        } = self;
        match alignment {
            XYZ => Distance::new(x, y, z),
            XzY => Distance::new(x, -z, y),
            Xyz => Distance::new(x, -y, -z),
            XZy => Distance::new(x, z, -y),
            yXZ => Distance::new(-y, x, z),
            ZXY => Distance::new(z, x, y),
            YXz => Distance::new(y, x, -z),
            zXy => Distance::new(-z, x, -y),
            xyZ => Distance::new(-x, -y, z),
            xzy => Distance::new(-x, -z, -y),
            xYz => Distance::new(-x, y, -z),
            xZY => Distance::new(-x, z, y),
            YxZ => Distance::new(y, -x, z),
            Zxy => Distance::new(z, -x, -y),
            yxz => Distance::new(-y, -x, -z),
            zxY => Distance::new(-z, -x, y),
            zYX => Distance::new(-z, y, x),
            YZX => Distance::new(y, z, x),
            ZyX => Distance::new(z, -y, x),
            yzX => Distance::new(-y, -z, x),
            zyx => Distance::new(-z, -y, -x),
            yZx => Distance::new(-y, z, -x),
            ZYx => Distance::new(z, y, -x),
            Yzx => Distance::new(y, -z, -x),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Scanner {
    position: Position,
    beacons: Vec<Position>,
}

impl Scanner {
    fn add_beacon(&mut self, beacon: Position) {
        self.beacons.push(beacon);
    }

    fn pairs(&self) -> impl Iterator<Item = (&Position, &Position)> + '_ {
        self.beacons.iter().flat_map(|first| {
            self.beacons
                .iter()
                .filter(move |second| first != *second)
                .map(move |second| (first, second))
        })
    }

    fn distances(&self) -> impl Iterator<Item = Distance> + '_ {
        self.pairs().map(|(first, second)| *first - *second)
    }
}

#[derive(Debug, Default, Clone)]
struct BeaconSystem {
    scanners: HashSet<Position>,
    beacons: HashSet<Position>,
}

impl FromIterator<Scanner> for BeaconSystem {
    fn from_iter<I: IntoIterator<Item = Scanner>>(iter: I) -> Self {
        let mut scanners = HashSet::new();
        let mut beacons = HashSet::new();

        let mut to_do = VecDeque::new();
        for scanner in iter {
            to_do.push_back(scanner);
        }

        // The first one will be the canonical scanner (center of the grid and
        // alignment).
        if let Some(scanner) = to_do.pop_front() {
            scanners.insert(scanner.position);
            beacons.extend(scanner.beacons);
        }
        let mut pairs: Vec<(Position, Position)> = beacons
            .iter()
            .flat_map(|first| {
                beacons
                    .iter()
                    .filter(move |second| first != *second)
                    .map(move |second| (*first, *second))
            })
            .collect();
        let mut distances: HashSet<Distance> = pairs
            .iter()
            .map(|(first, second)| *first - *second)
            .collect();

        while let Some(scanner) = to_do.pop_front() {
            let counter: Counter<Alignment> = scanner
                .distances()
                .flat_map(|new_distance| {
                    new_distance.all_alignments().filter_map(|(a, d)| {
                        if distances.contains(&d) {
                            Some(a)
                        } else {
                            None
                        }
                    })
                })
                .collect();

            if let Some(&(alignment, count)) = counter.most_common().first() {
                if count >= 66 {
                    let new_beacons: Vec<Position> = scanner
                        .beacons
                        .into_iter()
                        .map(|b| b.align(alignment))
                        .collect();
                    let ((q0, _), (p0, _)) = new_beacons
                        .iter()
                        .flat_map(|first| {
                            new_beacons
                                .iter()
                                .filter(move |second| first != *second)
                                .map(move |second| (first, second))
                        })
                        .find_map(|p| {
                            let new_d = *p.0 - *p.1;
                            pairs.iter().find(|&&q| q.0 - q.1 == new_d).map(|&q| (q, p))
                        })
                        .unwrap();
                    let scanner_position = Position::new(q0.x - p0.x, q0.y - p0.y, q0.z - p0.z);
                    scanners.insert(scanner_position);
                    beacons.extend(new_beacons.into_iter().map(|beacon| {
                        Position::new(
                            scanner_position.x + beacon.x,
                            scanner_position.y + beacon.y,
                            scanner_position.z + beacon.z,
                        )
                    }));
                    pairs = beacons
                        .iter()
                        .flat_map(|first| {
                            beacons
                                .iter()
                                .filter(move |second| first != *second)
                                .map(move |second| (*first, *second))
                        })
                        .collect();
                    distances = pairs
                        .iter()
                        .map(|(first, second)| *first - *second)
                        .collect();
                } else {
                    to_do.push_back(scanner);
                }
            } else {
                to_do.push_back(scanner);
            }
        }

        BeaconSystem { scanners, beacons }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut scanners = Vec::new();

    for line in input.lines() {
        if line.starts_with("---") {
            scanners.push(Scanner::default());
        } else if !line.is_empty() {
            scanners.last_mut().unwrap().add_beacon(line.parse()?);
        }
    }

    // Assemble the full map of beacons. How many beacons are there?
    let system: BeaconSystem = scanners.into_iter().collect();
    println!("Part 1: {}", system.beacons.len());

    // What is the largest Manhattan distance between any two scanners?
    let part2 = system
        .scanners
        .iter()
        .flat_map(|first| {
            system
                .scanners
                .iter()
                .map(|second| (*first - *second).manhattan_distance())
        })
        .max()
        .unwrap();
    println!("Part 2: {}", part2);

    Ok(())
}
