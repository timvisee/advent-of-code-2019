use std::fs;

fn main() {
    println!("Total fuel: {}", calc());
}

fn calc() -> i64 {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .map(fuel)
        .sum()
}

fn fuel(i: i64) -> i64 {
    let mut f = i / 3 - 2;
    if f < 0 {
        return 0;
    }
    f += fuel(f);
    f
}
