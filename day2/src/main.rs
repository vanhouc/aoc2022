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
            "A" | "X" => Ok(Rps::Rock),
            "B" | "Y" => Ok(Rps::Paper),
            "C" | "Z" => Ok(Rps::Scissors),
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

fn score_round(opponent: Rps, player: Rps) -> u32 {
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

fn main() {
    let strategy = include_str!("input.txt");
    let score: Result<u32, RockPaperScissorsError> = strategy
        .lines()
        .map(|line| {
            let (opponent, player) = line
                .split_once(' ')
                .ok_or(RockPaperScissorsError::ParsingError)?;
            let opponent: Rps = opponent.try_into()?;
            let player: Rps = player.try_into()?;
            Ok(score_round(opponent, player))
        })
        .sum();
    println!("{score:?}")
}
