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
        x += ((dir & 1 << 1) >> 1) as isize + ((dir & 1 << 3) >> 3) as isize * -1;
        y += (dir & 1) as isize * -1 + ((dir & 1 << 2) >> 2) as isize;
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
    pub fn new(p: Vec<isize>) -> Self {
        let ((i, _i), (_o, o)) = (channel(), channel());
        Self {
            p,
            n: 0,
            rb: 0,
            i,
            o,
            _i,
            _o,
        }
    }

    pub fn from_file(path: &str) -> Self {
        Self::new(
            std::fs::read_to_string(path)
                .unwrap()
                .split(',')
                .filter_map(|l| l.trim().parse().ok())
                .collect(),
        )
    }

    fn get(&self, i: isize, m: Option<isize>) -> isize {
        match m {
            Some(0) | None => *self
                .p
                .get(i as usize)
                .and_then(|i| self.p.get(*i as usize))
                .unwrap_or(&0),
            Some(1) => *self.p.get(i as usize).unwrap_or(&0),
            Some(2) => *self
                .p
                .get(i as usize)
                .and_then(|i| self.p.get((*i + self.rb) as usize))
                .unwrap_or(&0),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, i: isize, m: Option<isize>, v: isize) {
        let i = match m {
            Some(0) | None => self.p[i as usize] as usize,
            Some(2) => (self.rb + self.p[i as usize]) as usize,
            _ => unreachable!(),
        };
        if i >= self.p.len() {
            self.p.resize(i + 1, 0);
        }
        self.p[i] = v;
    }

    #[must_use]
    #[inline(always)]
    fn acc<'a>(&'a mut self, i: &'a mut isize, m: Option<isize>) -> &'a mut isize {
        let i = match m {
            Some(0) | None => self.p[*i as usize] as usize,
            Some(1) => return i,
            Some(2) => (self.rb + self.p[*i as usize]) as usize,
            _ => unreachable!(),
        };
        if i >= self.p.len() {
            self.p.resize(i + 1, 0);
        }
        &mut self.p[i]
    }

    #[inline(always)]
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
                    let v = *self.acc(&mut (self.n + 1), inst.pop())
                        + *self.acc(&mut (self.n + 2), inst.pop());
                    *self.acc(&mut (self.n + 3), inst.pop()) = v;
                    self.n + 4
                }
                (2, _) => {
                    let v = *self.acc(&mut (self.n + 1), inst.pop())
                        * *self.acc(&mut (self.n + 2), inst.pop());
                    *self.acc(&mut (self.n + 3), inst.pop()) = v;
                    self.n + 4
                }
                (3, _) => {
                    match self._i.try_recv() {
                        Ok(i) => self.set(self.n + 1, inst.pop(), i),
                        Err(_) => return false,
                    }
                    self.n + 2
                }
                (4, _) => {
                    self._o.send(self.get(self.n + 1, inst.pop())).unwrap();
                    self.n + 2
                }
                (5, _) if self.get(self.n + 1, inst.pop()) != 0 => {
                    *self.acc(&mut (self.n + 2), inst.pop())
                }
                (5, _) => self.n + 3,
                (6, _) if self.get(self.n + 1, inst.pop()) == 0 => {
                    *self.acc(&mut (self.n + 2), inst.pop())
                }
                (6, _) => self.n + 3,
                (7, _) => {
                    let v = if self.get(self.n + 1, inst.pop()) < self.get(self.n + 2, inst.pop()) {
                        1
                    } else {
                        0
                    };
                    self.set(self.n + 3, inst.pop(), v);
                    self.n + 4
                }
                (8, _) => {
                    let v = if self.get(self.n + 1, inst.pop()) == self.get(self.n + 2, inst.pop())
                    {
                        1
                    } else {
                        0
                    };
                    self.set(self.n + 3, inst.pop(), v);
                    self.n + 4
                }
                (9, 9) => break,
                (9, _) => {
                    self.rb += self.get(self.n + 1, inst.pop());
                    self.n + 2
                }
                _ => panic!("Unknown OPCODE"),
            };
        }
        true
    }
}
