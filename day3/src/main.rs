use std::collections::HashSet;

fn main() {
    let items: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let input = include_str!("input.txt");

    let sum: usize = input
        .lines()
        .filter_map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first: HashSet<char> = first.chars().collect();
            let second: HashSet<char> = second.chars().collect();
            Some(
                first
                    .intersection(&second)
                    .find_map(|c| items.iter().position(|i| i == c))?
                    + 1,
            )
        })
        .sum();

    println!("{:?}", sum)
}
