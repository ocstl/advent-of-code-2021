use std::str::FromStr;

const FILE: &str = "inputs/day3.txt";
const REPORT_LENGTH: usize = 12;

#[derive(Debug, Default, Clone)]
struct DiagnosticReport(Vec<u32>);

impl DiagnosticReport {
    fn gamma_rate(&self) -> u32 {
        (0..REPORT_LENGTH)
            .rev()
            .map(|idx| {
                let mask = 1_u32 << idx;
                if self.0.iter().filter(|n| *n & mask > 0).count() > self.0.len() / 2 {
                    mask
                } else {
                    0
                }
            })
            .sum()
    }

    fn epsilon_rate(&self) -> u32 {
        self.gamma_rate() ^ ((1 << REPORT_LENGTH) - 1)
    }

    fn power_consumption(&self) -> u32 {
        self.gamma_rate() * self.epsilon_rate()
    }

    fn oxygen_generator_rating(&self) -> u32 {
        let mut values = self.0.clone();
        values.sort_unstable();
        let mut remaining = values.as_slice();
        let mut mask = 1_u32 << REPORT_LENGTH;

        // Keep splitting along the 0/1 axis keeping the larger slice at each
        // step (breaking ties by taking 1), until we have only one remaining
        // value. The values MUST be sorted for this to work.
        while remaining.len() > 1 {
            mask >>= 1;
            match remaining.iter().position(|n| *n & mask == mask) {
                None | Some(0) => (),
                Some(idx) => {
                    let (zeros, ones) = remaining.split_at(idx);
                    if ones.len() >= zeros.len() {
                        remaining = ones;
                    } else {
                        remaining = zeros;
                    }
                }
            }
        }

        remaining[0]
    }

    fn co2_scrubber_rating(&self) -> u32 {
        let mut values = self.0.clone();
        values.sort_unstable();
        let mut remaining = values.as_slice();
        let mut mask = 1_u32 << REPORT_LENGTH;

        // Keep splitting along the 0/1 axis keeping the smaller slice at each
        // step (breaking ties by taking 0), until we have only one remaining
        // value. The values MUST be sorted for this to work.
        while remaining.len() > 1 {
            mask >>= 1;
            match remaining.iter().position(|n| *n & mask == mask) {
                None | Some(0) => (),
                Some(idx) => {
                    let (zeros, ones) = remaining.split_at(idx);
                    if zeros.len() <= ones.len() {
                        remaining = zeros;
                    } else {
                        remaining = ones;
                    }
                }
            }
        }

        remaining[0]
    }

    fn life_support_rating(&self) -> u32 {
        self.oxygen_generator_rating() * self.co2_scrubber_rating()
    }
}

impl FromStr for DiagnosticReport {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DiagnosticReport(
            s.lines()
                .map(|line| u32::from_str_radix(line, 2))
                .collect::<Result<_, _>>()?,
        ))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let report: DiagnosticReport = std::fs::read_to_string(FILE)?.parse()?;

    // Use the binary numbers in your diagnostic report to calculate the gamma
    // rate and epsilon rate, then multiply them together. What is the power
    // consumption of the submarine? (Be sure to represent your answer in
    // decimal, not binary.)
    let part1 = report.power_consumption();
    println!("Part 1: {}", part1);

    // Use the binary numbers in your diagnostic report to calculate the oxygen
    // generator rating and CO2 scrubber rating, then multiply them together.
    // What is the life support rating of the submarine? (Be sure to represent
    // your answer in decimal, not binary.)
    let part2 = report.life_support_rating();
    println!("Part 2: {}", part2);

    Ok(())
}
