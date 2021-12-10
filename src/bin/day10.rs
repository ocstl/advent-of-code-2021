use std::convert::TryFrom;

const FILE: &str = "inputs/day10.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BracketSubType {
    Parenthesis,
    SquareBracket,
    CurlyBracket,
    AngleBracket,
}

impl BracketSubType {
    fn syntax_error_score(self) -> u64 {
        match self {
            BracketSubType::Parenthesis => 3,
            BracketSubType::SquareBracket => 57,
            BracketSubType::CurlyBracket => 1197,
            BracketSubType::AngleBracket => 25137,
        }
    }

    fn completion_score(self) -> u64 {
        match self {
            BracketSubType::Parenthesis => 1,
            BracketSubType::SquareBracket => 2,
            BracketSubType::CurlyBracket => 3,
            BracketSubType::AngleBracket => 4,
        }
    }
}

impl TryFrom<char> for BracketSubType {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' | ')' => Ok(BracketSubType::Parenthesis),
            '[' | ']' => Ok(BracketSubType::SquareBracket),
            '{' | '}' => Ok(BracketSubType::CurlyBracket),
            '<' | '>' => Ok(BracketSubType::AngleBracket),
            _ => Err(c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BracketType {
    Opening(BracketSubType),
    Closing(BracketSubType),
}

impl TryFrom<char> for BracketType {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let sub_type = BracketSubType::try_from(c)?;
        match c {
            '(' | '[' | '{' | '<' => Ok(BracketType::Opening(sub_type)),
            ')' | ']' | '}' | '>' => Ok(BracketType::Closing(sub_type)),
            _ => Err(c),
        }
    }
}

fn balanced_brackets(line: &[BracketType]) -> Result<Vec<BracketSubType>, BracketSubType> {
    let mut stack = Vec::new();

    for &b in line {
        match b {
            BracketType::Opening(subtype) => stack.push(subtype),
            BracketType::Closing(subtype) => match stack.pop() {
                Some(ref other) if *other == subtype => (),
                _ => return Err(subtype),
            },
        }
    }

    // Return the opening brackets without corresponding closing brackets.
    Ok(stack)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code: Vec<Vec<BracketType>> = std::fs::read_to_string(FILE)?
        .lines()
        .map(|line| line.chars().map(BracketType::try_from).collect())
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    // Find the first illegal character in each corrupted line of the navigation
    // subsystem. What is the total syntax error score for those errors?
    let part1: u64 = code
        .iter()
        .filter_map(|line| {
            balanced_brackets(line)
                .err()
                .map(BracketSubType::syntax_error_score)
        })
        .sum();

    println!("Part 1: {}", part1);

    // Find the completion string for each incomplete line, score the completion
    // strings, and sort the scores. What is the middle score?
    let mut completion_scores: Vec<u64> = code
        .iter()
        .filter_map(|line| balanced_brackets(line).ok())
        .map(|subtypes| {
            subtypes
                .iter()
                .rev()
                .fold(0, |acc, s| acc * 5 + s.completion_score())
        })
        .collect();
    completion_scores.sort_unstable();
    let part2 = completion_scores[completion_scores.len() / 2];

    println!("Part 2: {}", part2);

    Ok(())
}
