#![feature(is_sorted)]
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .unwrap()
        .split('-')
        .filter_map(|n| n.trim().parse().ok())
        .collect::<Vec<u32>>();

    println!(
        "Valid entries: {}",
        (input[0]..=input[1]).filter(is_valid).count(),
    );
}

fn is_valid(i: &u32) -> bool {
    let b = format!("{}", i).into_bytes();
    b.is_sorted() && b.windows(2).enumerate().any(|(i, w)| w[0] == w[1]
        && *b.get(i - 1).unwrap_or(&0) != w[0]
        && *b.get(i + 2).unwrap_or(&0) != w[0]
    )
}
