use regex::Regex;

fn main() {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let mut pos: Vec<(isize, isize, isize)> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|c| (c[1].parse().unwrap(), c[2].parse().unwrap(), c[3].parse().unwrap()))
        .collect();
    let mut vel = vec![(0, 0, 0); pos.len()];

    for _ in 0..1000 {
        vel.iter_mut().enumerate().for_each(|(i, v)| {
            pos[0..i].iter().chain(pos[i..].iter().skip(1)).for_each(|o| {
                v.0 += (o.0 - pos[i].0).signum();
                v.1 += (o.1 - pos[i].1).signum();
                v.2 += (o.2 - pos[i].2).signum();
            });
        });
        pos.iter_mut().zip(&vel).for_each(|(p, v)| { p.0 += v.0; p.1 += v.1; p.2 += v.2 });
    }

    println!(
        "Energy: {}",
        pos.iter()
            .zip(&vel)
            .map(|(p, v)| (p.0.abs() + p.1.abs() + p.2.abs()) * (v.0.abs() + v.1.abs() + v.2.abs()))
            .sum::<isize>()
    );
}
