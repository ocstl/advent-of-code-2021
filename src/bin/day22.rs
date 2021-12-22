use std::str::FromStr;

const FILE: &str = "inputs/day22.txt";
const INITIALIZATION_AREA: Cuboid = Cuboid {
    x: (-50, 50),
    y: (-50, 50),
    z: (-50, 50),
};

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Cuboid {
    fn volume(&self) -> i64 {
        (self.x.1 + 1 - self.x.0) * (self.y.1 + 1 - self.y.0) * (self.z.1 + 1 - self.z.0)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let x = (self.x.0.max(other.x.0), self.x.1.min(other.x.1));
        let y = (self.y.0.max(other.y.0), self.y.1.min(other.y.1));
        let z = (self.z.0.max(other.z.0), self.z.1.min(other.z.1));
        if (x.0 <= x.1) && (y.0 <= y.1) && (z.0 <= z.1) {
            Some(Cuboid { x, y, z })
        } else {
            None
        }
    }

    fn restrict_to_initialization_area(&self) -> Option<Cuboid> {
        self.intersection(&INITIALIZATION_AREA)
    }
}

#[derive(Debug, Clone, Copy)]
struct RebootStep {
    state: bool,
    area: Cuboid,
}

impl RebootStep {
    fn restrict_to_initialization_area(&self) -> Option<Self> {
        self.area
            .restrict_to_initialization_area()
            .map(|area| RebootStep {
                state: self.state,
                area,
            })
    }
}

impl FromStr for RebootStep {
    type Err = String;

    fn from_str(step: &str) -> Result<Self, Self::Err> {
        if let Some((state, ranges)) = step.trim().split_once(' ') {
            let state = state == "on";
            let mut ranges = ranges.split(',');
            if let (Some(x), Some(y), Some(z)) = (ranges.next(), ranges.next(), ranges.next()) {
                let x = x.trim_start_matches("x=").split_once("..");
                let y = y.trim_start_matches("y=").split_once("..");
                let z = z.trim_start_matches("z=").split_once("..");
                if let (Some(x), Some(y), Some(z)) = (x, y, z) {
                    let x = (x.0.parse().unwrap(), x.1.parse().unwrap());
                    let y = (y.0.parse().unwrap(), y.1.parse().unwrap());
                    let z = (z.0.parse().unwrap(), z.1.parse().unwrap());
                    return Ok(RebootStep {
                        state,
                        area: Cuboid { x, y, z },
                    });
                }
            }
        }

        Err(step.to_string())
    }
}

#[derive(Debug, Default, Clone)]
struct Part2(Vec<(i64, Cuboid)>);

impl Part2 {
    fn nbr_cubes_on(&self) -> i64 {
        self.0
            .iter()
            .map(|(value, cuboid)| value * cuboid.volume())
            .sum()
    }

    fn apply(&mut self, step: &RebootStep) {
        // Check against previous cuboids, before inserting the cuboid (only
        // those that turn on lights though).
        // If we are turning on cubes:
        // - a positive cuboid means it was already lit, so insert a negative
        //   cuboid to compensate.
        // - a negative cuboid means it was turned off, after having been turned
        //   on. So we know there is a previous negative cuboid (see above),
        //   and we reverse the correction.
        // If we are turning off cubes:
        // - a positive cuboid means it was lit, so insert a negative cuboid.
        // - a negative cuboid means it was turned off, after having been turned
        //   on, so we add a positive cuboid to reverse the correction.
        //
        // Afterwards, only insert the lighting cuboids.
        let mut new_cuboids = self
            .0
            .iter()
            .filter_map(|(turn_on, cuboid)| {
                cuboid
                    .intersection(&step.area)
                    .map(|c| match (step.state, *turn_on == 1) {
                        (true, true) => (-1, c),
                        (true, false) => (1, c),
                        (false, true) => (-1, c),
                        (false, false) => (1, c),
                    })
            })
            .collect();

        if step.state {
            self.0.push((1, step.area));
        }

        // TODO: Prune equal cuboids with opposite signs.
        self.0.append(&mut new_cuboids);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let reboot_steps = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<RebootStep>, _>>()?;

    // Execute the reboot steps. Afterward, considering only cubes in the
    // region x=-50..50,y=-50..50,z=-50..50, how many cubes are on?
    let mut reactor = Part2::default();
    for step in reboot_steps
        .iter()
        .filter_map(RebootStep::restrict_to_initialization_area)
    {
        reactor.apply(&step);
    }
    let part1 = reactor.nbr_cubes_on();
    println!("Part 1: {}", part1);

    // Starting again with all cubes off, execute all reboot steps. Afterward,
    // considering all cubes, how many cubes are on?
    let mut reactor = Part2::default();
    for step in &reboot_steps {
        reactor.apply(step);
    }
    let part2 = reactor.nbr_cubes_on();
    println!("Part 2: {}", part2);

    Ok(())
}
