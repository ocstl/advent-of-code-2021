use std::str::FromStr;

const FILE: &str = "inputs/day21.txt";
// Pairs of the sum of three rolls of a three-sided dice with their frequency.
const DIRAC_DICE: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Player1,
    Player2,
}

#[derive(Debug, Clone, Copy)]
struct Player {
    position: u64,
    score: u64,
}

impl FromStr for Player {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(position) = input.trim().split_whitespace().last() {
            let position = position
                .parse()
                .map_err(|e: std::num::ParseIntError| e.to_string())?;
            Ok(Player { position, score: 0 })
        } else {
            Err(format!("Invalid format: {}", input.to_string()))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DiracDiceGame {
    player1: Player,
    player2: Player,
}

impl DiracDiceGame {
    // Use 0 to 9 positions internally to simplify the code. This means the
    // score added will be (position + 1) instead of position.
    fn new(mut player1: Player, mut player2: Player) -> Self {
        player1.position -= 1;
        player2.position -= 1;

        DiracDiceGame { player1, player2 }
    }

    fn deterministic_game(&mut self, winning_score: u64) -> u64 {
        let mut nbr_rolls = 0;
        let mut die = (1..=100).cycle();

        loop {
            nbr_rolls += 3;
            self.player1.position =
                (self.player1.position + die.by_ref().take(3).sum::<u64>()) % 10;
            self.player1.score += self.player1.position + 1;
            if self.player1.score >= winning_score {
                break;
            }

            nbr_rolls += 3;
            self.player2.position =
                (self.player2.position + die.by_ref().take(3).sum::<u64>()) % 10;
            self.player2.score += self.player2.position + 1;
            if self.player2.score >= winning_score {
                break;
            }
        }

        self.player1.score.min(self.player2.score) * nbr_rolls
    }

    fn dirac_dice_game(&self, winning_score: u64) -> u64 {
        let mut stack = vec![(*self, Turn::Player1, 1)];
        let mut player1_wins = 0;
        let mut player2_wins = 0;

        while let Some((game, turn, freq)) = stack.pop() {
            for (roll, frequency) in DIRAC_DICE {
                let mut next_turn = game;
                match turn {
                    Turn::Player1 => {
                        next_turn.player1.position = (game.player1.position + roll) % 10;
                        next_turn.player1.score += next_turn.player1.position + 1;
                        if next_turn.player1.score >= winning_score {
                            player1_wins += freq * frequency;
                        } else {
                            stack.push((next_turn, Turn::Player2, freq * frequency));
                        }
                    }
                    Turn::Player2 => {
                        next_turn.player2.position = (game.player2.position + roll) % 10;
                        next_turn.player2.score += next_turn.player2.position + 1;
                        if next_turn.player2.score >= winning_score {
                            player2_wins += freq * frequency;
                        } else {
                            stack.push((next_turn, Turn::Player1, freq * frequency));
                        }
                    }
                }
            }
        }

        player1_wins.max(player2_wins)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let mut lines = input.lines();
    let player1: Player = lines.next().unwrap().parse()?;
    let player2: Player = lines.next().unwrap().parse()?;

    // Play a practice game using the deterministic 100-sided die. The moment
    // either player wins, what do you get if you multiply the score of the
    // losing player by the number of times the die was rolled during the game?
    let part1 = DiracDiceGame::new(player1, player2).deterministic_game(1000);
    println!("Part 1: {}", part1);

    // Using your given starting positions, determine every possible outcome.
    // Find the player that wins in more universes; in how many universes does
    // that player win?
    let part2 = DiracDiceGame::new(player1, player2).dirac_dice_game(21);
    println!("Part 2: {}", part2);

    Ok(())
}
