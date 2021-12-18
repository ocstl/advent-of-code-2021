use std::str::FromStr;

const FILE: &str = "inputs/day18.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Comma,
    Regular(u32),
}

impl From<&Token> for char {
    fn from(token: &Token) -> char {
        match token {
            Token::Open => '[',
            Token::Close => ']',
            Token::Comma => ',',
            Token::Regular(n) => char::from_digit(*n, 10).unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SnailfishNumber(Vec<Token>);

impl SnailfishNumber {
    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        let mut current_level = 0;
        for idx in 0..self.0.len() {
            match self.0[idx] {
                Token::Open => current_level += 1,
                Token::Close => current_level -= 1,
                Token::Comma => (),
                Token::Regular(left) => {
                    if current_level > 4 && matches!(self.0[idx + 1], Token::Comma) {
                        if let Token::Regular(right) = self.0[idx + 2] {
                            if let Some(Token::Regular(l)) = self.0[0..idx]
                                .iter_mut()
                                .rev()
                                .find(|t| matches!(t, Token::Regular(_)))
                            {
                                *l += left;
                            }
                            if let Some(Token::Regular(r)) = self.0[idx + 3..]
                                .iter_mut()
                                .find(|t| matches!(t, Token::Regular(_)))
                            {
                                *r += right;
                            }
                            self.0.splice(idx - 1..idx + 4, [Token::Regular(0)]);
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        for idx in 0..self.0.len() {
            match self.0[idx] {
                Token::Open | Token::Close | Token::Comma => (),
                Token::Regular(n) => {
                    if n > 9 {
                        let left = n / 2;
                        let right = n - left;
                        self.0.splice(
                            idx..idx + 1,
                            [
                                Token::Open,
                                Token::Regular(left),
                                Token::Comma,
                                Token::Regular(right),
                                Token::Close,
                            ],
                        );
                        return true;
                    }
                }
            }
        }

        false
    }

    fn magnitude(&self) -> u32 {
        let mut total = 0;
        let mut multiplier = 1;

        for token in &self.0 {
            match token {
                Token::Open => multiplier *= 3,
                Token::Close => multiplier /= 2,
                Token::Comma => {
                    multiplier /= 3;
                    multiplier *= 2;
                }
                Token::Regular(v) => total += v * multiplier,
            }
        }

        total
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut s = SnailfishNumber(
            std::iter::once(Token::Open)
                .chain(self.0.into_iter())
                .chain(std::iter::once(Token::Comma))
                .chain(other.0.into_iter())
                .chain(std::iter::once(Token::Close))
                .collect(),
        );

        s.reduce();
        s
    }
}

impl FromStr for SnailfishNumber {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tokens = input
            .chars()
            .filter_map(|c| match c {
                '[' => Some(Token::Open),
                ']' => Some(Token::Close),
                ',' => Some(Token::Comma),
                c if c.is_ascii_digit() => Some(Token::Regular(c.to_digit(10).unwrap())),
                _ => unreachable!(),
            })
            .collect();

        Ok(SnailfishNumber(tokens))
    }
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(char::from).collect::<String>())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers: Vec<SnailfishNumber> = std::fs::read_to_string(FILE)?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    // Add up all of the snailfish numbers from the homework assignment in the
    // order they appear. What is the magnitude of the final sum?
    let mut part1 = numbers[0].clone();
    for number in &numbers[1..] {
        part1 = part1 + number.clone();
    }

    println!("Part 1: {}", part1.magnitude());

    // What is the largest magnitude of any sum of two different snailfish
    // numbers from the homework assignment?
    let part2 = numbers
        .iter()
        .flat_map(|first| {
            // Needs a `filter` to avoid reusing the same number.
            numbers
                .iter()
                .map(|second| (first.clone() + second.clone()).magnitude())
        })
        .max()
        .unwrap();

    println!("Part 2: {}", part2);

    Ok(())
}
