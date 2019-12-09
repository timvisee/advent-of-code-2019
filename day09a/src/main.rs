fn main() {
    let mut program: Vec<isize> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .split(',')
        .filter_map(|l| l.trim().parse::<isize>().ok())
        .collect();

    println!("BOOST code: {:?}", run(&mut program, vec![1]));
}

fn run(p: &mut Vec<isize>, mut i: Vec<isize>) -> isize {
    let mut n = 0;
    let mut rb = 0;
    let get = |p: &[_], i, m, rb| match m {
            Some(0) | None => *p.get(i as usize).and_then(|i| p.get(*i as usize)).unwrap_or(&0),
            Some(1) => *p.get(i as usize).unwrap_or(&0),
            Some(2) => *p.get(i as usize).and_then(|i| p.get((*i + rb) as usize)).unwrap_or(&0),
            _ => unreachable!(),
        };
    let set = |p: &mut Vec<_>, i, m, v, rb| {
            let i = match m {
                Some(0) | None => p[i] as usize,
                Some(2) => (rb + p[i]) as usize,
                _ => unreachable!(),
            };
            if i >= p.len() {
                p.resize(i + 1, 0);
            }
            p[i] = v;
        };
    while n < p.len() {
        let mut inst = format!("{}", p[n])
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        n = match (inst.pop().unwrap(), inst.pop().unwrap_or(0)) {
            (1, _) => {
                let v = get(p, n + 1, inst.pop(), rb) + get(p, n + 2, inst.pop(), rb);
                set(p, n + 3, inst.pop(), v, rb); n + 4 }
            (2, _) => {
                let v = get(p, n + 1, inst.pop(), rb) * get(p, n + 2, inst.pop(), rb);
                set(p, n + 3, inst.pop(), v, rb); n + 4
            }
            (3, _) => { set(p, n + 1, inst.pop(), i.remove(0), rb); n + 2 },
            (4, _) => { return get(p, n + 1, inst.pop(), rb) }
            (5, _) if get(p, n + 1, inst.pop(), rb) != 0 => get(p, n + 2, inst.pop(), rb) as usize,
            (5, _) => n + 3,
            (6, _) if get(p, n + 1, inst.pop(), rb) == 0 => get(p, n + 2, inst.pop(), rb) as usize,
            (6, _) => n + 3,
            (7, _) => {
                let v = if get(p, n + 1, inst.pop(), rb) < get(p, n + 2, inst.pop(), rb) { 1 } else { 0 };
                set(p, n + 3, inst.pop(), v, rb);
                n + 4
            }
            (8, _) => {
                let v = if get(p, n + 1, inst.pop(), rb) == get(p, n + 2, inst.pop(), rb) { 1 } else { 0 };
                set(p, n + 3, inst.pop(), v, rb);
                n + 4
            }
            (9, 9) => break,
            (9, _) => {
                rb += get(p, n + 1, inst.pop(), rb);
                n + 2
            }
            _ => panic!("Unknown OPCODE"),
        };
    }
    panic!("No output");
}
