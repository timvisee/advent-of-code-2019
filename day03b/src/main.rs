use std::fs;
use std::mem;

use itertools::Itertools;

fn main() {
    let intersect = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(Wire::parse)
        .tuple_combinations()
        .flat_map(|(a, b)| a.intersections(&b))
        .min();

    println!("Distance: {:?}", intersect.unwrap());
}

#[derive(Clone, Debug)]
struct Wire {
    lines: Vec<Line>,
}

impl Wire {
    pub fn parse(data: &str) -> Self {
        let lines = data
            .split(',')
            .map(|i| {
                let d = i
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<isize>()
                    .unwrap();
                match i.chars().next().unwrap() {
                    'L' => Point(-d, 0),
                    'R' => Point(d, 0),
                    'U' => Point(0, d),
                    'D' => Point(0, -d),
                    _ => panic!("Unknown direction"),
                }
            })
            .scan((Point(0, 0), 0), |(a, dist), b| {
                let m = Point(a.0 + b.0, a.1 + b.1);
                let min = Point(a.0.min(m.0), a.1.min(m.1));
                let max = Point(a.0.max(m.0), a.1.max(m.1));
                let l = Line {
                    min,
                    max,
                    min_cost: *dist + min.distance(a),
                    max_cost: *dist + max.distance(a),
                };
                *a = m;
                *dist += min.distance(a).max(max.distance(a));
                Some(l)
            })
            .collect();
        Wire { lines }
    }

    pub fn intersections(&self, other: &Self) -> Vec<isize> {
        self.lines
            .iter()
            .cartesian_product(other.lines.iter())
            .filter_map(|(a, b)| a.intersect(b))
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Line {
    min: Point,
    max: Point,
    min_cost: isize,
    max_cost: isize,
}

impl Line {
    fn intersect(&self, other: &Line) -> Option<isize> {
        if self.horizontal() == other.horizontal() {
            return None;
        }

        let mut a = self;
        let mut b = other;
        if self.horizontal() {
            mem::swap(&mut a, &mut b);
        }

        if a.min.0 <= b.min.0 && a.max.0 >= b.min.0 && b.min.1 <= a.min.1 && b.max.1 >= a.min.1 {
            let p = Point(b.min.0, a.min.1);
            Some(
                (self.min_cost + self.min.distance(&p)).min(self.max_cost + self.max.distance(&p))
                    + (other.min_cost + other.min.distance(&p))
                        .min(other.max_cost + other.max.distance(&p)),
            )
        } else {
            None
        }
    }

    fn horizontal(&self) -> bool {
        self.min.0 == self.max.0
    }
}

#[derive(Copy, Clone, Debug)]
struct Point(isize, isize);

impl Point {
    pub fn distance(&self, other: &Point) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}
