use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    let mut c = Computer::from_file("./input.txt");
    c.p[0] = 2;
    let mut map = HashMap::new();
    let (mut pad, mut ball, mut score) = (0, 0, 0);

    while {
        let done = !c.run();
        while let Ok(x) = c.o.try_recv() {
            let y = c.o.try_recv().unwrap();
            let b = c.o.try_recv().unwrap();
            match (x, y, b) {
                (-1, 0, _) => score = b,
                (_, _, 3) => pad = x,
                (_, _, 4) => ball = x,
                _ => {}
            }
            map.entry((x, y)).and_modify(|v| *v = b).or_insert(b);
        }
        c.i.send((ball - pad).signum()).unwrap();
        done
    } {}

    println!("Score: {}", score);
}

struct Computer {
    p: Vec<isize>,
    n: isize,
    rb: isize,
    pub i: Sender<isize>,
    pub o: Receiver<isize>,
    _i: Receiver<isize>,
    _o: Sender<isize>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl Computer {
    pub fn from_file(path: &str) -> Self {
        let ((i, _i), (_o, o)) = (channel(), channel());
        Self {
            p: std::fs::read_to_string(path)
                .unwrap()
                .split(',')
                .filter_map(|l| l.trim().parse().ok())
                .collect(),
            n: 0, rb: 0, i, o, _i, _o,
        }
    }

    #[must_use]
    #[inline(always)]
    fn acc(&mut self, i: isize, m: Option<isize>) -> &mut isize {
        let i = match m {
            Some(0) | None => self.p[i as usize] as usize,
            Some(1) => return &mut self.p[i as usize],
            Some(2) => (self.rb + self.p[i as usize]) as usize,
            _ => unreachable!(),
        };
        if i >= self.p.len() {
            self.p.resize(i + 1, 0);
        }
        &mut self.p[i]
    }

    fn run(&mut self) -> bool {
        loop {
            let mut inst = vec![
                (self.p[self.n as usize] / 10000) % 10,
                (self.p[self.n as usize] / 1000) % 10,
                (self.p[self.n as usize] / 100) % 10,
                (self.p[self.n as usize] / 10) % 10,
                self.p[self.n as usize] % 10,
            ];
            self.n = match (inst.pop().unwrap(), inst.pop().unwrap_or(0)) {
                (1, _) => {
                    let v = *self.acc(self.n + 1, inst.pop()) + *self.acc(self.n + 2, inst.pop());
                    *self.acc(self.n + 3, inst.pop()) = v;
                    self.n + 4
                }
                (2, _) => {
                    let v = *self.acc(self.n + 1, inst.pop()) * *self.acc(self.n + 2, inst.pop());
                    *self.acc(self.n + 3, inst.pop()) = v;
                    self.n + 4
                }
                (3, _) => {
                    match self._i.try_recv() {
                        Ok(i) => *self.acc(self.n + 1, inst.pop()) = i,
                        Err(_) => return false,
                    }
                    self.n + 2
                }
                (4, _) => {
                    let v = *self.acc(self.n + 1, inst.pop());
                    self._o.send(v).unwrap();
                    self.n + 2
                }
                (5, _) if *self.acc(self.n + 1, inst.pop()) != 0 => *self.acc(self.n + 2, inst.pop()) as isize,
                (5, _) => self.n + 3,
                (6, _) if *self.acc(self.n + 1, inst.pop()) == 0 => *self.acc(self.n + 2, inst.pop()) as isize,
                (6, _) => self.n + 3,
                (7, _) => {
                    let v = if *self.acc(self.n + 1, inst.pop()) < *self.acc(self.n + 2, inst.pop()) { 1 } else { 0 };
                    *self.acc(self.n + 3, inst.pop()) = v;
                    self.n + 4
                }
                (8, _) => {
                    let v = if *self.acc(self.n + 1, inst.pop()) == *self.acc(self.n + 2, inst.pop()) { 1 } else { 0 };
                    *self.acc(self.n + 3, inst.pop()) = v;
                    self.n + 4
                }
                (9, 9) => return true,
                (9, _) => { self.rb += *self.acc(self.n + 1, inst.pop()); self.n + 2 }
                _ => panic!("Unknown OPCODE"),
            };
        }
    }
}
