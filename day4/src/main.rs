use std::{ops::Range, str::FromStr};

use color_eyre::Report;

struct CleaningPair {
    first: Range<u32>,
    second: Range<u32>,
}

impl CleaningPair {
    fn is_overlapped(&self) -> bool {
        ((self.first.start <= self.second.start) && (self.first.end >= self.second.end))
            || ((self.second.start <= self.first.start) && (self.second.end >= self.first.end))
    }
}

impl FromStr for CleaningPair {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_range(s: &str) -> Result<Range<u32>, Report> {
            let (start, end) = s
                .split_once('-')
                .ok_or(color_eyre::eyre::eyre!("expected <start>-<end> got {s:?}"))?;
            Ok(start.parse()?..end.parse()?)
        }
        let (first, second) = s.split_once(',').ok_or(color_eyre::eyre::eyre!(
            "expected <pair1>,<pair2> got {s:?}"
        ))?;
        Ok(CleaningPair {
            first: parse_range(first)?,
            second: parse_range(second)?,
        })
    }
}

fn main() -> Result<(), Report> {
    color_eyre::install().unwrap();
    let input = include_str!("input.txt");
    let overlapped: Vec<bool> = input
        .lines()
        .map(|line| Ok::<bool, Report>(line.parse::<CleaningPair>()?.is_overlapped()))
        .collect::<Result<Vec<_>, _>>()?;
    let overlapped = overlapped.iter().filter(|b| **b).count();
    Ok(println!("{overlapped:?}"))
}
