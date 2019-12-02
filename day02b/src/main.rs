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
        .find(|(noun, verb)| {
            let mut program = program.clone();
            program[1] = *noun;
            program[2] = *verb;

            run(&mut program) == 19_690_720
        })
        .unwrap();

    println!("Result: {}", 100 * noun + verb);
}

fn run(p: &mut [usize]) -> usize {
    for i in (0..).step_by(4) {
        match p[i] {
            1 => p[p[i + 3]] = p[p[i + 1]] + p[p[i + 2]],
            2 => p[p[i + 3]] = p[p[i + 1]] * p[p[i + 2]],
            99 => break,
            _ => panic!("Unknown OPCODE"),
        }
    }
    p[0]
}
