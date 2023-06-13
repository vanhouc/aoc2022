use thiserror::Error;

enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Rps {
    type Error = RockPaperScissorsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Rps::Rock),
            "B" => Ok(Rps::Paper),
            "C" => Ok(Rps::Scissors),
            _ => Err(RockPaperScissorsError::InvalidCharacter),
        }
    }
}
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl TryFrom<&str> for Outcome {
    type Error = RockPaperScissorsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(RockPaperScissorsError::InvalidCharacter),
        }
    }
}

#[derive(Debug, Error)]
enum RockPaperScissorsError {
    #[error("invalid syntax in strategy")]
    ParsingError,
    #[error("invalid character found in strategy")]
    InvalidCharacter,
}

fn score_round(opponent: &Rps, player: &Rps) -> u32 {
    match player {
        Rps::Rock => {
            1 + match opponent {
                Rps::Rock => 3,
                Rps::Paper => 0,
                Rps::Scissors => 6,
            }
        }
        Rps::Paper => {
            2 + match opponent {
                Rps::Rock => 6,
                Rps::Paper => 3,
                Rps::Scissors => 0,
            }
        }
        Rps::Scissors => {
            3 + match opponent {
                Rps::Rock => 0,
                Rps::Paper => 6,
                Rps::Scissors => 3,
            }
        }
    }
}

fn score_outcome(opponent: Rps, outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Win => match opponent {
            Rps::Rock => score_round(&opponent, &Rps::Paper),
            Rps::Paper => score_round(&opponent, &Rps::Scissors),
            Rps::Scissors => score_round(&opponent, &Rps::Rock),
        },
        Outcome::Lose => match opponent {
            Rps::Rock => score_round(&opponent, &Rps::Scissors),
            Rps::Paper => score_round(&opponent, &Rps::Rock),
            Rps::Scissors => score_round(&opponent, &Rps::Paper),
        },
        Outcome::Draw => score_round(&opponent, &opponent),
    }
}

fn main() {
    let strategy = include_str!("input.txt");
    let score: Result<u32, RockPaperScissorsError> = strategy
        .lines()
        .map(|line| {
            let (opponent, outcome) = line
                .split_once(' ')
                .ok_or(RockPaperScissorsError::ParsingError)?;
            let opponent: Rps = opponent.try_into()?;
            let outcome: Outcome = outcome.try_into()?;
            Ok(score_outcome(opponent, outcome))
        })
        .sum();
    println!("{score:?}")
}
