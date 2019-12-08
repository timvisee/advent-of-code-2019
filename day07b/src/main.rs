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
        "Signal: {}",
        (5..10).permutations(5).map(|i|
            run_amps(&mut program.clone(), i.to_vec())
        ).max().unwrap(),
    );
}

fn run_amps(p: &mut [isize], mut i: Vec<isize>) -> isize {
    let mut a = vec![(p.to_vec(), 0); i.len()];
    let mut o = 0;
    loop {
        for (ref mut m, ref mut p) in &mut a {
            let mut s = vec![o];
            if *p == 0 {
                s.insert(0, i.pop().unwrap());
            }

            match run(m, p, s) {
                Some(s) => o = s,
                None => return o,
            }
        }
    }
}

fn run(p: &mut [isize], n: &mut usize, mut i: Vec<isize>) -> Option<isize> {
    let get = |p: &[_], i, m| if let Some(1) = m { p[i as usize] } else { p[p[i as usize] as usize] };
    while *n < p.len() {
        let mut inst = format!("{}", p[*n])
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<u8>>();
        match (inst.pop().unwrap(), inst.pop().unwrap_or(0)) {
            (1, _) => { p[p[*n + 3] as usize] = get(p, *n + 1, inst.pop()) + get(p, *n + 2, inst.pop()); *n += 4 }
            (2, _) => { p[p[*n + 3] as usize] = get(p, *n + 1, inst.pop()) * get(p, *n + 2, inst.pop()); *n += 4 }
            (3, _) => { p[p[*n + 1] as usize] = i.remove(0); *n += 2 },
            (4, _) => { *n += 2; return Some(get(p, *n - 1, inst.pop())) },
            (5, _) if get(p, *n + 1, inst.pop()) != 0 => *n = get(p, *n + 2, inst.pop()) as usize,
            (5, _) => *n += 3,
            (6, _) if get(p, *n + 1, inst.pop()) == 0 => *n = get(p, *n + 2, inst.pop()) as usize,
            (6, _) => *n += 3,
            (7, _) => {
                p[p[*n + 3] as usize] = if get(p, *n + 1, inst.pop()) < get(p, *n + 2, inst.pop()) { 1 } else { 0 };
                *n += 4
            }
            (8, _) => {
                p[p[*n + 3] as usize] = if get(p, *n + 1, inst.pop()) == get(p, *n + 2, inst.pop()) { 1 } else { 0 };
                *n += 4
            }
            (9, 9) => return None,
            _ => panic!("Unknown OPCODE"),
        };
    }
    panic!("No output");
}
