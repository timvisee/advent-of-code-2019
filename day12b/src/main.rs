use num_integer::lcm;
use rayon::prelude::*;
use regex::Regex;

fn main() {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let pos: Vec<(isize, isize, isize)> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|c| (c[1].parse().unwrap(), c[2].parse().unwrap(), c[3].parse().unwrap()))
        .collect();
    let vel = vec![(0, 0, 0); pos.len()];

    let s = |pos: &[(isize, isize, isize)], vel: &[(_, _, _)], f: fn(&(_, _, _)) -> _| -> usize {
        let mut run_pos = pos.to_vec();
        let mut run_vel = vel.to_vec();
        for i in 1.. {
            run_vel.iter_mut().enumerate().for_each(|(i, v)| {
                run_pos[0..i].iter().chain(run_pos[i..].iter().skip(1)).for_each(|o| {
                    v.0 += (o.0 - run_pos[i].0).signum();
                    v.1 += (o.1 - run_pos[i].1).signum();
                    v.2 += (o.2 - run_pos[i].2).signum();
                });
            });
            run_pos.iter_mut().zip(&run_vel).for_each(|(p, v)| { p.0 += v.0; p.1 += v.1; p.2 += v.2 });
            if (0..run_pos.len()).all(|i| f(&run_pos[i]) == f(&pos[i]) && f(&run_vel[i]) == f(&vel[i])) {
                return i;
            }
        }
        unreachable!()
    };

    let comps: [fn(&(_, _, _)) -> _; 3] = [|v| v.0, |v| v.1, |v| v.2];
    println!(
        "Steps: {}",
        comps.into_par_iter().map(|f| s(&pos, &vel, *f)).reduce(|| 1, lcm),
    );
}
