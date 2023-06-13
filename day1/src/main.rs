fn main() {
    let mut elf: Vec<u64> = include_str!("input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .filter_map(|v| v.parse::<u64>().ok())
                .sum::<u64>()
        })
        .collect();
    elf.sort();
    println!("{:?}", elf.iter().rev().take(3).sum::<u64>());
}
