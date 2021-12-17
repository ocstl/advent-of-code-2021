use std::cmp::{Ordering, PartialOrd};
use std::ops::RangeInclusive;
use std::str::FromStr;

const FILE: &str = "inputs/day17.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl std::ops::Add<Velocity> for Position {
    type Output = Self;

    fn add(self, rhs: Velocity) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    fn new(x: i32, y: i32) -> Self {
        Velocity { x, y }
    }

    fn drag(self) -> Self {
        Velocity::new((self.x - 1).max(0), self.y - 1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Probe {
    position: Position,
    velocity: Velocity,
}

impl Probe {
    fn new(dx: i32, dy: i32) -> Self {
        Probe {
            position: Position::default(),
            velocity: Velocity::new(dx, dy),
        }
    }
}

impl Iterator for Probe {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity.drag();
        Some(self.position)
    }
}

#[derive(Debug, Clone)]
struct TargetArea {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

// Use this to determine whether a position is within the target area.
impl PartialEq<TargetArea> for Position {
    fn eq(&self, area: &TargetArea) -> bool {
        area.x_range.contains(&self.x) && area.y_range.contains(&self.y)
    }
}

// Use this to determine whether a position is within the target area (Equal) or
// past it (Greater), defined as:
//      y < min(y_range) OR x > max(x_range)
// This makes sense as x is non-decreasing, while y is eventually decreasing
// with time.
impl PartialOrd<TargetArea> for Position {
    fn partial_cmp(&self, area: &TargetArea) -> Option<Ordering> {
        Some(
            self.y
                .cmp(area.y_range.start())
                .reverse()
                .then(self.x.cmp(area.x_range.end())),
        )
    }
}

impl FromStr for TargetArea {
    type Err = String;

    fn from_str(area: &str) -> Result<Self, Self::Err> {
        let area = area.trim().trim_start_matches("target area: ");

        if let Some((x, y)) = area.split_once(", ") {
            let x_range = x
                .trim_start_matches("x=")
                .split_once("..")
                .map(|(min_x, max_x)| {
                    RangeInclusive::new(min_x.parse().unwrap(), max_x.parse().unwrap())
                });
            let y_range = y
                .trim_start_matches("y=")
                .split_once("..")
                .map(|(min_y, max_y)| {
                    RangeInclusive::new(min_y.parse().unwrap(), max_y.parse().unwrap())
                });
            if let (Some(x_range), Some(y_range)) = (x_range, y_range) {
                return Ok(TargetArea { x_range, y_range });
            }
        }

        Err(format!("Invalid input: {}", area.to_string()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let target: TargetArea = input.parse()?;

    // Find the initial velocity that causes the probe to reach the highest y
    // position and still eventually be within the target area after any step.
    // What is the highest y position it reaches on this trajectory?
    // We can treat the x and y positions separately, which simplifies greatly
    // the analysis. A positive y velocity will draw a parabola which will end
    // up back at 0 (0 -> 1 -> 1 -> 0) with -(y + 1) velocity: thus, at the next
    // step, it will reach -(y + 1). By setting this as the lowest point of the
    // range over y, we can determine the initial y (positive).
    let initial_y = -(target.y_range.start() + 1);
    let part1 = (initial_y * (initial_y + 1)) / 2;
    println!("Part 1: {}", part1);

    // How many distinct initial velocity values cause the probe to be within
    // the target area after any step?
    // Let's simulate all reasonable probe trajectories.
    let min_dx = (2.0 * *target.x_range.start() as f64).sqrt() as i32;
    let max_dx = *target.x_range.end();
    let min_dy = *target.y_range.start();

    let part2 = (min_dx..=max_dx)
        .flat_map(|dx| (min_dy..=min_dy.abs()).map(move |dy| Probe::new(dx, dy)))
        .filter(|probe| {
            probe
                .into_iter()
                .take_while(|p| p <= &target)
                .any(|p| p == target)
        })
        .count();
    println!("Part 2: {}", part2);

    Ok(())
}
