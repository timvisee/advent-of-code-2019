use std::fs;

fn main() {
    let mut program: Vec<usize> = fs::read_to_string("./input.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.parse::<usize>().ok())
        .collect();

    program[1] = 12;
    program[2] = 2;

    run(&mut program);

    println!("Result: {}", program[0]);
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
