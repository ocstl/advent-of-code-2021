use std::str::FromStr;

const FILE: &str = "inputs/day6.txt";

#[derive(Debug, Default, Clone, Copy)]
struct LanternFishPopulation([u64; 9]);

impl LanternFishPopulation {
    fn total_population(&self) -> u64 {
        self.0.iter().sum()
    }
}

impl Iterator for LanternFishPopulation {
    type Item = LanternFishPopulation;

    fn next(&mut self) -> Option<Self::Item> {
        // Give birth to new lanternfish (timer 8). Don't forget to add the
        // original fishes at timer 6.
        self.0.rotate_left(1);
        self.0[6] += self.0[8];
        Some(*self)
    }
}

impl FromStr for LanternFishPopulation {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut population = [0; 9];
        for fish in input.trim().split(',') {
            let age: usize = fish.parse()?;
            population[age] += 1;
        }

        Ok(LanternFishPopulation(population))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut population: LanternFishPopulation = std::fs::read_to_string(FILE)?.parse()?;

    // Find a way to simulate lanternfish. How many lanternfish would there be
    // after 80 days?
    // `nth(79)` because it is 0-indexed.
    let part1 = population.nth(79).unwrap().total_population();
    println!("Part 1: {}", part1);

    // How many lanternfish would there be after 256 days?
    let part2 = population.nth(255 - 80).unwrap().total_population();
    println!("Part 1: {}", part2);

    Ok(())
}
