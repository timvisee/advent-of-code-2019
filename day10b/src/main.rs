#![feature(vec_remove_item)]
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut astroids: Vec<(usize, usize)> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes()
                .enumerate()
                .filter(|(_, p)| p == &b'#')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let ((x, y), _) = &astroids
        .par_iter()
        .map(|(x, y)| {
            (
                (*x, *y),
                astroids
                    .iter()
                    .filter(|(xx, yy)| xx != x || yy != y)
                    .map(|(xx, yy)| {
                        -((*x as f64 - *xx as f64).atan2(*y as f64 - *yy as f64) * 1000.0) as isize
                    })
                    .unique()
                    .count(),
            )
        })
        .max_by_key(|(_, n)| *n)
        .unwrap();
    astroids.remove_item(&(*x, *y));

    let mut angled_map: HashMap<isize, Vec<((usize, usize), isize)>> = astroids
        .iter()
        .map(|(xx, yy)| {
            (
                (xx, yy),
                -((*x as f64 - *xx as f64).atan2(*y as f64 - *yy as f64) * 1000.0) as isize,
                ((*x as isize - *xx as isize).abs() + (*y as isize - *yy as isize).abs()),
            )
        })
        .fold(HashMap::new(), |mut map, ((x, y), angle, dist)| {
            map.entry(angle).or_default().push(((*x, *y), dist));
            map
        });
    angled_map
        .iter_mut()
        .for_each(|(_, v)| v.sort_by_key(|(_, dist)| -dist));

    println!(
        "200th position: {:?}",
        angled_map
            .keys()
            .cloned()
            .sorted()
            .cycle()
            .skip_while(|angle| *angle < 0)
            .filter_map(|ref angle| angled_map.get_mut(angle).and_then(|a| a.pop()).map(|i| i.0))
            .map(|i| i.0 * 100 + i.1)
            .nth(199)
            .unwrap(),
    );
}
