use itertools::Itertools;
use std::fs;

fn main() {
    let program: Vec<usize> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.parse::<usize>().ok())
        .collect();

    let (noun, verb) = (0..100)
        .cartesian_product(0..100)
        .filter(|(noun, verb)| {
            let mut program = program.clone();
            program[1] = *noun;
            program[2] = *verb;

            run(&mut program);

            program[0] == 19_690_720
        })
        .next()
        .unwrap();

    println!("Result: {}", 100 * noun + verb);
}

fn run(p: &mut [usize]) {
    for i in (0..).step_by(4) {
        match p[i] {
            1 => p[p[i + 3]] = p[i + 1..i + 3].iter().map(|i| p[*i]).sum(),
            2 => p[p[i + 3]] = p[i + 1..i + 3].iter().map(|i| p[*i]).product(),
            99 => return,
            _ => panic!("Unknown OPCODE"),
        }
    }
}
