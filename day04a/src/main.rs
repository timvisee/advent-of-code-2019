#![feature(is_sorted)]
use std::fs;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .unwrap()
        .split('-')
        .filter_map(|n| n.trim().parse().ok())
        .collect::<Vec<u32>>();

    println!(
        "Valid entries: {}",
        (input[0]..=input[1]).into_par_iter().filter(is_valid).count(),
    );
}

fn is_valid(i: &u32) -> bool {
    let b = [
        i / 100000 % 10,
        i / 10000 % 10,
        i / 1000 % 10,
        i / 100 % 10,
        i / 10 % 10,
        i / 1 % 10,
    ];
    b.is_sorted() && b.windows(2).any(|w| w[0] == w[1])
}
