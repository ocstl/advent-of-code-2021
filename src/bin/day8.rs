use counter::Counter;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::str::FromStr;

const FILE: &str = "inputs/day8.txt";
const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

thread_local! {
    static DIGIT_SETS: Vec<HashSet<char>> = {
        DIGITS.iter().map(|d| d.chars().collect()).collect()
    };
}

#[derive(Debug, Clone)]
struct Day8Entry {
    patterns: [String; 10],
    output: [String; 4],
}

impl Day8Entry {
    fn decode(&self) -> usize {
        let mut map = HashMap::new();
        let frequencies: Counter<char> = self.patterns.iter().flat_map(|p| p.chars()).collect();

        // 'b', 'e' and 'f' have specific frequencies.
        for (&k, v) in frequencies.iter() {
            match v {
                4 => map.insert(k, 'e'),
                6 => map.insert(k, 'b'),
                9 => map.insert(k, 'f'),
                _ => None,
            };
        }

        // We can proceed by elimination for the rest. For example, 1 has a
        // unique length (2) and contains "cf"; since 'f' is already known, the
        // remaining `char` is 'c'.
        let one = self.patterns.iter().find(|p| p.len() == 2).unwrap();
        let c = one.chars().find(|s| !map.contains_key(s)).unwrap();
        map.insert(c, 'c');

        // Four = "bcdf", so 'd' will be the odd char out.
        let four = self.patterns.iter().find(|p| p.len() == 4).unwrap();
        let d = four.chars().find(|s| !map.contains_key(s)).unwrap();
        map.insert(d, 'd');

        #[allow(clippy::map_entry)]
        // 'a' has a frequency of 8, 'g' of 7, so we can find them this way.
        for (&k, v) in frequencies.iter() {
            if !map.contains_key(&k) {
                match v {
                    7 => map.insert(k, 'g'),
                    8 => map.insert(k, 'a'),
                    _ => None,
                };
            }
        }

        self.output
            .iter()
            .map(|o| {
                let s: HashSet<char> = o.chars().map(|d| map.get(&d).unwrap()).copied().collect();
                DIGIT_SETS.with(|ds| ds.iter().position(|d| d == &s).unwrap())
            })
            .fold(0, |acc, d| acc * 10 + d)
    }
}

impl FromStr for Day8Entry {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = input.split_once('|') {
            let patterns: Vec<String> = left
                .trim()
                .split_whitespace()
                .map(|p| p.chars().collect())
                .collect();
            let output: Vec<String> = right
                .trim()
                .split_whitespace()
                .map(|p| p.chars().collect())
                .collect();

            let patterns = <[String; 10]>::try_from(patterns).map_err(|_| "Missing patterns.")?;
            let output = <[String; 4]>::try_from(output).map_err(|_| "Missing patterns.")?;

            Ok(Day8Entry { patterns, output })
        } else {
            Err("Missing patterns.")
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entries = std::fs::read_to_string(FILE)?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Day8Entry>, _>>()?;

    // In the output values, how many times do digits 1, 4, 7, or 8 appear?
    // 1 uses 2 segments, 4 uses 4, 7 uses 3 and 8 uses all 7.
    const UNIQUE_LENGTHS: [usize; 4] = [2, 4, 3, 7];
    let part1 = entries
        .iter()
        .flat_map(|entry| entry.output.iter())
        .filter(|o| UNIQUE_LENGTHS.contains(&o.len()))
        .count();

    println!("Part 1: {}", part1);

    // For each entry, determine all of the wire/segment connections and decode
    // the four-digit output values. What do you get if you add up all of the
    // output values?
    let part2: usize = entries.iter().map(|e| e.decode()).sum();

    println!("Part 2: {}", part2);

    Ok(())
}
