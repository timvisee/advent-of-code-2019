use std::fs;

fn main() {
    let mut program: Vec<usize> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.parse::<usize>().ok())
        .collect();

    program[1] = 12;
    program[2] = 2;

    println!("Result: {}", run(&mut program));
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
