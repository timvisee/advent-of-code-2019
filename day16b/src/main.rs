use std::str;

fn main() {
    let raw = std::fs::read("./input.txt").unwrap();
    let offset: usize = unsafe { str::from_utf8_unchecked(&raw[0..7]) }
        .parse()
        .unwrap();
    let mut data: Vec<isize> = raw
        .iter()
        .filter(|b| **b >= b'0')
        .map(|b| (b - b'0') as isize)
        .cycle()
        .take((raw.len() - 1) * 10_000)
        .skip(offset)
        .collect();

    for _ in 0..100 {
        for i in (0..data.len() - 1).rev() {
            data[i] = (data[i] + data[i + 1]) % 10;
        }
    }

    println!(
        "Message: {}",
        data[0..8]
            .iter()
            .map(|b| (*b as u8 + b'0') as char)
            .collect::<String>()
            .parse::<usize>()
            .unwrap(),
    );
}
