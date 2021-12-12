use counter::Counter;
use std::collections::HashMap;
use std::str::FromStr;

const FILE: &str = "inputs/day12.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cave {
    Small(String),
    Large(String),
}

impl Cave {
    fn is_small(&self) -> bool {
        matches!(self, Cave::Small(_))
    }
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(cave: &str) -> Result<Self, Self::Err> {
        let cave = cave.to_string();
        if cave.chars().all(char::is_lowercase) {
            Ok(Cave::Small(cave))
        } else {
            Ok(Cave::Large(cave))
        }
    }
}

#[derive(Debug, Clone)]
struct CaveSystem(HashMap<Cave, Vec<Cave>>);

impl CaveSystem {
    const START_CAVE: &'static str = "start";
    const END_CAVE: &'static str = "end";

    fn generate_all_paths(&self) -> Vec<Vec<Cave>> {
        let end_cave: Cave = Self::END_CAVE.parse().unwrap();

        let mut to_visit: Vec<Vec<Cave>> = vec![vec![Self::START_CAVE.parse().unwrap()]];
        let mut paths: Vec<Vec<Cave>> = Vec::new();

        while let Some(path) = to_visit.pop() {
            let last = path.last().unwrap();
            if *last == end_cave {
                paths.push(path);
            } else if let Some(new_caves) = self.0.get(last) {
                for new_cave in new_caves {
                    if !new_cave.is_small() || !path.contains(new_cave) {
                        let mut new_path = path.clone();
                        new_path.push(new_cave.clone());
                        to_visit.push(new_path);
                    }
                }
            }
        }

        paths
    }

    fn generate_paths(&self, max_visits: usize) -> Vec<Vec<Cave>> {
        let start_cave: Cave = Self::START_CAVE.parse().unwrap();
        let end_cave: Cave = Self::END_CAVE.parse().unwrap();

        let mut to_visit: Vec<Vec<Cave>> = vec![vec![Self::START_CAVE.parse().unwrap()]];
        let mut paths: Vec<Vec<Cave>> = Vec::new();

        while let Some(path) = to_visit.pop() {
            let last = path.last().unwrap();
            if *last == end_cave {
                paths.push(path);
            } else if let Some(new_caves) = self.0.get(last) {
                for new_cave in new_caves {
                    match new_cave {
                        c if *c == start_cave => (),
                        Cave::Large(_) => {
                            let mut new_path = path.clone();
                            new_path.push(new_cave.clone());
                            to_visit.push(new_path);
                        }
                        Cave::Small(_) => {
                            let mut new_path = path.clone();
                            new_path.push(new_cave.clone());

                            // This is very hacky.
                            if new_path
                                .iter()
                                .filter(|c| c.is_small())
                                .collect::<Counter<_>>()
                                .values()
                                .product::<usize>()
                                <= max_visits
                            {
                                to_visit.push(new_path);
                            }
                        }
                    }
                }
            }
        }

        paths
    }
}

impl FromStr for CaveSystem {
    type Err = String;

    fn from_str(system: &str) -> Result<Self, Self::Err> {
        let mut h: HashMap<Cave, Vec<Cave>> = HashMap::new();
        for line in system.lines() {
            if let Some((left, right)) = line.split_once('-') {
                let left: Cave = left.parse().unwrap();
                let right: Cave = right.parse().unwrap();
                h.entry(left.clone()).or_default().push(right.clone());
                h.entry(right).or_default().push(left);
            } else {
                return Err(line.to_string());
            }
        }

        Ok(CaveSystem(h))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cave_system: CaveSystem = std::fs::read_to_string(FILE)?.parse()?;

    // How many paths through this cave system are there that visit small caves
    // at most once?
    let part1 = cave_system.generate_all_paths().len();
    println!("Part 1: {}", part1);

    // Given these new rules, how many paths through this cave system are there?
    let part2 = cave_system.generate_paths(2).len();
    println!("Part 2: {}", part2);

    Ok(())
}
