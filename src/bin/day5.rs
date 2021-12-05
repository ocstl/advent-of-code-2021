use counter::Counter;
use std::num::ParseIntError;
use std::str::FromStr;

const FILE: &str = "inputs/day5.txt";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct VentLine {
    start_position: Position,
    end_position: Position,
}

impl VentLine {
    fn is_vertical(self) -> bool {
        self.start_position.x == self.end_position.x
    }

    fn is_horizontal(self) -> bool {
        self.start_position.y == self.end_position.y
    }

    // Only works for horizontal, vertical and 45-degree diagonal lines.
    fn iter(self) -> impl Iterator<Item = Position> {
        let dx = (self.end_position.x - self.start_position.x).signum();
        let dy = (self.end_position.y - self.start_position.y).signum();
        std::iter::successors(Some(self.start_position), move |&prev| {
            if prev != self.end_position {
                Some(Position::new(prev.x + dx, prev.y + dy))
            } else {
                None
            }
        })
    }
}

impl FromStr for VentLine {
    type Err = String;

    fn from_str(vents: &str) -> Result<Self, Self::Err> {
        let mut positions = vents.split(" -> ");
        let start_position =
            positions
                .next()
                .map_or(Err(String::from("Missing start position.")), |s| {
                    if let Some((x, y)) = s.split_once(',') {
                        let x = x.parse().map_err(|e: ParseIntError| e.to_string())?;
                        let y = y.parse().map_err(|e: ParseIntError| e.to_string())?;
                        Ok(Position::new(x, y))
                    } else {
                        Err(String::from("Missing start position."))
                    }
                })?;

        let end_position =
            positions
                .next()
                .map_or(Err(String::from("Missing end position.")), |s| {
                    if let Some((x, y)) = s.split_once(',') {
                        let x = x.parse().map_err(|e: ParseIntError| e.to_string())?;
                        let y = y.parse().map_err(|e: ParseIntError| e.to_string())?;
                        Ok(Position::new(x, y))
                    } else {
                        Err(String::from("Missing end position."))
                    }
                })?;

        Ok(VentLine {
            start_position,
            end_position,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vent_lines: Vec<VentLine> = std::fs::read_to_string(FILE)?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    // Consider only horizontal and vertical lines. At how many points do at
    // least two lines overlap?
    let part1 = vent_lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .flat_map(|line| line.iter())
        .collect::<Counter<_>>()
        .values()
        .filter(|v| **v > 1)
        .count();

    println!("The answer is: {}", part1);

    // Consider all of the lines. At how many points do at least two lines
    // overlap?
    let part2 = vent_lines
        .iter()
        .flat_map(|line| line.iter())
        .collect::<Counter<_>>()
        .values()
        .filter(|v| **v > 1)
        .count();

    println!("The answer is: {}", part2);

    Ok(())
}
