fn main() {
    let elf = include_str!("input.txt")
        .lines()
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .filter_map(|v| v.parse::<u64>().ok())
                .sum::<u64>()
        })
        .max();
    println!("{elf:?}");
}
