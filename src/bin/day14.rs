use counter::Counter;
use std::collections::HashMap;
use std::iter::FromIterator;

const FILE: &str = "inputs/day14.txt";

type Element = u8;
type Pair = (Element, Element);

#[derive(Debug, Clone)]
struct Polymer {
    elements: Counter<Element>,
    pairs: Counter<Pair>,
}

impl Polymer {
    fn strengthen(&mut self, rules: &PairInsertionRules) {
        // We need to replace the pairs entirely: for example, CH yields CB and
        // BH, so they are no longer CH pairs. Much simpler to generate a new
        // counter.
        // On the other hand, we can keep the former element counter, only
        // updating with the new insertions (the C and H elements do not
        // disappear).
        let mut pairs = Counter::new();
        for (pair, count) in self.pairs.iter() {
            let insertion = *rules.0.get(pair).unwrap();
            *pairs.entry((pair.0, insertion)).or_default() += count;
            *pairs.entry((insertion, pair.1)).or_default() += count;
            *self.elements.entry(insertion).or_default() += count;
        }

        self.pairs = pairs;
    }
}

impl<T: AsRef<[u8]>> From<T> for Polymer {
    fn from(polymer: T) -> Self {
        let polymer = polymer.as_ref();
        let elements = polymer.iter().copied().collect();
        let pairs = polymer
            .iter()
            .copied()
            .zip(polymer.iter().skip(1).copied())
            .collect();

        Polymer { elements, pairs }
    }
}

#[derive(Debug, Clone)]
struct PairInsertionRules(HashMap<Pair, Element>);

impl<'a> FromIterator<&'a str> for PairInsertionRules {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut h = HashMap::new();
        for line in iter {
            if let Some((left, right)) = line.split_once(" -> ") {
                let left = left.as_bytes();
                let right = right.as_bytes();
                h.insert((left[0], left[1]), right[0]);
            }
        }

        PairInsertionRules(h)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut lines = input.lines();
    let mut polymer = Polymer::from(lines.next().unwrap().as_bytes());
    let rules: PairInsertionRules = lines.collect();

    // Apply 10 steps of pair insertion to the polymer template and find the
    // most and least common elements in the result. What do you get if you
    // take the quantity of the most common element and subtract the quantity
    // of the least common element?
    for _ in 0..10 {
        polymer.strengthen(&rules);
    }
    let part1 = polymer.elements.values().max().unwrap() - polymer.elements.values().min().unwrap();
    println!("Part 1: {}", part1);

    // Apply 40 steps of pair insertion to the polymer template and find the
    // most and least common elements in the result. What do you get if you
    // take the quantity of the most common element and subtract the quantity
    // of the least common element?
    for _ in 10..40 {
        polymer.strengthen(&rules);
    }
    let part2 = polymer.elements.values().max().unwrap() - polymer.elements.values().min().unwrap();
    println!("Part 2: {}", part2);

    Ok(())
}
