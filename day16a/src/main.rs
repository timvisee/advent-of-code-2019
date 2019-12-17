use rayon::prelude::*;
use std::mem;

fn main() {
    let mut data: Vec<isize> = std::fs::read("./input.txt")
        .unwrap()
        .into_iter()
        .filter(|b| *b >= b'0')
        .map(|b| (b - b'0') as isize)
        .collect();

    for _ in 0..100 {
        run_phase(&mut data);
    }

    println!(
        "Message: {}",
        data[0..8]
            .iter()
            .map(|b| (*b as u8 + b'0') as char)
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    );
}

fn run_phase(data: &mut Vec<isize>) {
    mem::swap(
        data,
        &mut (0..data.len())
            .into_par_iter()
            .map(|row| {
                ((0..)
                    .map(|i| i * 4 * (row + 1) + row)
                    .take_while(|i| *i < data.len())
                    .flat_map(|i| data[i..].iter().take(row + 1))
                    .sum::<isize>()
                    - (0..)
                        .map(|i| i * 4 * (row + 1) + row + 2 * (row + 1))
                        .take_while(|i| *i < data.len())
                        .flat_map(|i| data[i..].iter().take(row + 1))
                        .sum::<isize>())
                .abs()
                    % 10
            })
            .collect::<Vec<_>>(),
    );
}
