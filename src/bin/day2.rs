const FILE: &str = "inputs/day2.txt";

#[derive(Debug, Clone, Copy)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((direction, amount)) = s.split_once(' ') {
            let amount = amount
                .parse()
                .map_err(|e: std::num::ParseIntError| e.to_string())?;
            match direction {
                "forward" => Ok(Command::Forward(amount)),
                "down" => Ok(Command::Down(amount)),
                "up" => Ok(Command::Up(amount)),
                _ => Err(format!("Invalid command: {}", direction)),
            }
        } else {
            Err(format!("Invalid command: {}", s))
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Position {
    depth: i32,
    distance: i32,
}

impl Position {
    fn command(self, command: Command) -> Self {
        match command {
            Command::Forward(d) => Position {
                depth: self.depth,
                distance: self.distance + d,
            },
            Command::Down(d) => Position {
                depth: self.depth + d,
                distance: self.distance,
            },
            Command::Up(d) => Position {
                depth: self.depth - d,
                distance: self.distance,
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Submarine {
    depth: i32,
    distance: i32,
    aim: i32,
}

impl Submarine {
    fn command(self, command: Command) -> Self {
        match command {
            Command::Forward(d) => Submarine {
                depth: self.depth + d * self.aim,
                distance: self.distance + d,
                aim: self.aim,
            },
            Command::Down(d) => Submarine {
                depth: self.depth,
                distance: self.distance,
                aim: self.aim + d,
            },
            Command::Up(d) => Submarine {
                depth: self.depth,
                distance: self.distance,
                aim: self.aim - d,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commands = std::fs::read_to_string(FILE)?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Command>, _>>()?;

    // Calculate the horizontal position and depth you would have after
    // following the planned course. What do you get if you multiply your final
    // horizontal position by your final depth?
    let part1 = commands
        .iter()
        .fold(Position::default(), |p, c| p.command(*c));
    println!("Part 1: {}", part1.depth * part1.distance);

    // Using this new interpretation of the commands, calculate the horizontal
    // position and depth you would have after following the planned course.
    // What do you get if you multiply your final horizontal position by your
    // final depth?
    let part2 = commands
        .iter()
        .fold(Submarine::default(), |s, c| s.command(*c));
    println!("Part 2: {}", part2.depth * part2.distance);

    Ok(())
}
