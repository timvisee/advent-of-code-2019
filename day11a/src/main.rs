use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
    let mut c = Computer::from_file("./input.txt");
    let (mut x, mut y, mut dir) = (0, 0, 0b00010001u8);
    let mut hull = HashMap::new();

    while {
        c.i.send(*hull.get(&(x, y)).unwrap_or(&0)).unwrap();
        !c.run()
    } {
        *hull.entry((x, y)).or_default() = c.o.try_recv().unwrap();
        dir = dir.rotate_right(c.o.try_recv().unwrap() as u32 * 6 + 1);
        x += ((dir & 1 << 1) >> 1) as isize - ((dir & 1 << 3) >> 3) as isize;
        y += -((dir & 1) as isize) + ((dir & 1 << 2) >> 2) as isize;
    }

    println!("Painted panels: {}", hull.len());
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

impl Computer {
    pub fn from_file(path: &str) -> Self {
        let ((i, _i), (_o, o)) = (channel(), channel());
        Self {
            p: std::fs::read_to_string(path)
                .unwrap()
                .split(',')
                .filter_map(|l| l.trim().parse().ok())
                .collect(),
            n: 0,
            rb: 0,
            i,
            o,
            _i,
            _o,
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
                (5, _) if *self.acc(self.n + 1, inst.pop()) != 0 => {
                    *self.acc(self.n + 2, inst.pop())
                }
                (5, _) => self.n + 3,
                (6, _) if *self.acc(self.n + 1, inst.pop()) == 0 => {
                    *self.acc(self.n + 2, inst.pop())
                }
                (6, _) => self.n + 3,
                (7, _) => {
                    let v = if *self.acc(self.n + 1, inst.pop()) < *self.acc(self.n + 2, inst.pop())
                    {
                        1
                    } else {
                        0
                    };
                    *self.acc(self.n + 3, inst.pop()) = v;
                    self.n + 4
                }
                (8, _) => {
                    let v =
                        if *self.acc(self.n + 1, inst.pop()) == *self.acc(self.n + 2, inst.pop()) {
                            1
                        } else {
                            0
                        };
                    *self.acc(self.n + 3, inst.pop()) = v;
                    self.n + 4
                }
                (9, 9) => break,
                (9, _) => {
                    self.rb += *self.acc(self.n + 1, inst.pop());
                    self.n + 2
                }
                _ => panic!("Unknown OPCODE"),
            };
        }
        true
    }
}
