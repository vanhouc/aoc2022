use std::env;

struct Elf {
    total_calories: u32,
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    println!("{input}")
}
