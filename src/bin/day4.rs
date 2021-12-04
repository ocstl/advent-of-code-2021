use std::convert::TryFrom;

const FILE: &str = "inputs/day4.txt";
const SIZE: usize = 5;

type Card = [u32; 25];
type Marks = [bool; 25];

#[derive(Debug, Clone)]
struct BingoCard {
    card: Card,
    marks: Marks,
}

impl BingoCard {
    fn mark(&mut self, draw: u32) -> Option<u32> {
        if let Some(idx) = self.card.iter().position(|v| *v == draw) {
            self.marks[idx] = true;
            if self.win() {
                Some(self.score())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn win(&self) -> bool {
        self.marks.chunks(SIZE).any(|h| h.iter().all(|v| *v))
            || (0..SIZE).any(|start| self.marks.iter().skip(start).step_by(SIZE).all(|v| *v))
    }

    fn score(&self) -> u32 {
        self.card
            .iter()
            .zip(self.marks.iter())
            .filter_map(|(v, m)| if *m { None } else { Some(v) })
            .sum()
    }
}

impl TryFrom<[&str; 5]> for BingoCard {
    type Error = String;

    fn try_from(lines: [&str; 5]) -> Result<Self, String> {
        let card: Vec<u32> = lines
            .iter()
            .flat_map(|line| line.trim().split_whitespace().map(|c| c.trim().parse()))
            .collect::<Result<_, _>>()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        let card =
            Card::try_from(card).map_err(|e| format!("Wrong number of numbers: {}", e.len()))?;

        Ok(BingoCard {
            card,
            marks: Marks::default(),
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut lines = input.lines().filter(|line| !line.is_empty());

    let draws = lines
        .next()
        .expect("Missing input.")
        .split(',')
        .map(|v| v.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    let mut lines = lines.filter(|line| !line.trim().is_empty());
    let mut cards = Vec::new();
    while let (Some(l1), Some(l2), Some(l3), Some(l4), Some(l5)) = (
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
    ) {
        cards.push(BingoCard::try_from([l1, l2, l3, l4, l5])?);
    }

    // To guarantee victory against the giant squid, figure out which board
    // will win first. What will your final score be if you choose that board?
    let mut cards1 = cards.clone();
    let part1 = draws
        .iter()
        .find_map(|draw| {
            cards1
                .iter_mut()
                .find_map(|card| card.mark(*draw))
                .map(|score| score * draw)
        })
        .expect("Found no winning board.");
    println!("Part 1: {}", part1);

    // Figure out which board will win last. Once it wins, what would its final
    // score be?
    let mut cards2 = cards.clone();
    let mut draw_iter = draws.iter();
    while cards2.len() > 1 {
        let draw = draw_iter.next().unwrap();
        for card in cards2.iter_mut() {
            card.mark(*draw);
        }
        cards2.retain(|card| !card.win());
    }

    let mut remaining_card = cards2.remove(0);
    let part2 = draw_iter
        .find_map(|draw| remaining_card.mark(*draw).map(|score| score * draw))
        .expect("Found more than a single possible last winning board.");
    println!("Part 2: {}", part2);

    Ok(())
}
