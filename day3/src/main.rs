use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let items: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let input = include_str!("input.txt");

    let sum: usize = input
        .lines()
        .map(|line| -> HashSet<char> { line.chars().collect() })
        .chunks(3)
        .into_iter()
        .filter_map(|chunk| {
            let (first, second, third) = chunk.collect_tuple()?;
            Some::<usize>(
                first
                    .intersection(&second)
                    .copied()
                    .collect::<HashSet<char>>()
                    .intersection(&third)
                    .filter_map(|c| items.iter().position(|i| i == c))
                    .sum::<usize>()
                    + 1,
            )
        })
        .sum::<usize>();
    println!("{sum}")
}
