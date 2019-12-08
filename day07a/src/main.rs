#![feature(slice_patterns)]

use std::fs;
use itertools::Itertools;

fn main() {
    let program: Vec<isize> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.trim().parse::<isize>().ok())
        .collect();

    println!(
        "Sequence: {:?}",
        (0..5).permutations(5).max_by_key(|i| i.iter()
            .fold(0, |signal, i| run(&mut program.clone(), vec![*i, signal]))
        ).unwrap(),
    );
}

fn run(p: &mut [isize], mut i: Vec<isize>) -> isize {
    let mut n = 0;
    let get = |p: &[_], i, m| if let Some(1) = m { p[i as usize] } else { p[p[i as usize] as usize] };
    while n < p.len() {
        let mut inst = format!("{}", p[n])
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<u8>>();
        n = match (inst.pop().unwrap(), inst.pop().unwrap_or(0)) {
            (1, _) => { p[p[n + 3] as usize] = get(p, n + 1, inst.pop()) + get(p, n + 2, inst.pop()); n + 4 }
            (2, _) => { p[p[n + 3] as usize] = get(p, n + 1, inst.pop()) * get(p, n + 2, inst.pop()); n + 4 }
            (3, _) => { p[p[n + 1] as usize] = i.remove(0); n + 2 },
            (4, _) => { return get(p, n + 1, inst.pop()) }
            (5, _) if get(p, n + 1, inst.pop()) != 0 => get(p, n + 2, inst.pop()) as usize,
            (5, _) => n + 3,
            (6, _) if get(p, n + 1, inst.pop()) == 0 => get(p, n + 2, inst.pop()) as usize,
            (6, _) => n + 3,
            (7, _) => {
                p[p[n + 3] as usize] = if get(p, n + 1, inst.pop()) < get(p, n + 2, inst.pop()) { 1 } else { 0 };
                n + 4
            }
            (8, _) => {
                p[p[n + 3] as usize] = if get(p, n + 1, inst.pop()) == get(p, n + 2, inst.pop()) { 1 } else { 0 };
                n + 4
            }
            (9, 9) => break,
            _ => panic!("Unknown OPCODE"),
        };
    }
    panic!("No output" );
}
