#![feature(slice_patterns)]

use std::fs;

fn main() {
    let mut program: Vec<isize> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.trim().parse::<isize>().ok())
        .collect();

    println!("Code: {}", run(&mut program, 5).last().unwrap());
}

fn run(p: &mut [isize], i: isize) -> Vec<isize> {
    let mut n = 0;
    let mut o = vec![];
    let get = |p: &[_], i, m| if let Some(1) = m { p[i as usize] } else { p[p[i as usize] as usize] };
    while n < p.len() {
        let mut inst = format!("{}", p[n])
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<u8>>();
        n = match (inst.pop().unwrap(), inst.pop().unwrap_or(0)) {
            (1, _) => { p[p[n + 3] as usize] = get(p, n + 1, inst.pop()) + get(p, n + 2, inst.pop()); n + 4 }
            (2, _) => { p[p[n + 3] as usize] = get(p, n + 1, inst.pop()) * get(p, n + 2, inst.pop()); n + 4 }
            (3, _) => { p[p[n + 1] as usize] = i; n + 2 },
            (4, _) => { o.push(get(p, n + 1, inst.pop())); n + 2 }
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
    o
}
