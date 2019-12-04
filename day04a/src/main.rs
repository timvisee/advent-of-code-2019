use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .unwrap()
        .split('-')
        .filter_map(|n| n.trim().parse().ok())
        .collect::<Vec<u32>>();
    let min = input[0];
    let max = input[1];

    println!("Valid entries: {}", (min..=max).filter(is_valid).count());
}

fn is_valid(i: &u32) -> bool {
    let b = format!("{}", i).into_bytes();
    b.windows(2).any(|b| b[0] == b[1]) && b.windows(2).all(|b| b[0] <= b[1])
}
