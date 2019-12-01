use std::fs;

fn main() {
    println!("Total fuel: {}", calc());
}

fn calc() -> i64 {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .map(|i| i / 3 - 2)
        .sum()
}
