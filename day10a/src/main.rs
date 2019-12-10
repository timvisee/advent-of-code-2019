use itertools::Itertools;

fn main() {
    let astroids: Vec<(usize, usize)> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.bytes().enumerate().filter(|(_, p)| p == &b'#').map(move |(x, _)| (x, y)))
        .collect();

    println!(
        "Max: {}",
        astroids
            .iter()
            .map(|(x, y)| {
                astroids
                    .iter()
                    .filter(|(xx, yy)| xx != x || yy != y)
                    .map(|(xx, yy)| -((*x as f64 - *xx as f64).atan2(*y as f64 - *yy as f64) * 1000.0) as isize)
                    .unique()
                    .count()
            })
            .max()
            .unwrap(),
    );
}
