use itertools::Itertools;

fn main() {
    let data = std::fs::read("./input.txt").unwrap();
    (0..25 * 6)
        .map(|i| {
            match data
                .iter()
                .skip(i)
                .step_by(25 * 6)
                .find(|i| **i != b'2')
                .unwrap_or(&b'1')
            {
                b'0' => ' ',
                b'1' => '#',
                b'2' => ' ',
                _ => unreachable!(),
            }
        })
        .chunks(25)
        .into_iter()
        .for_each(|a| println!("{}", a.collect::<String>()));
}
