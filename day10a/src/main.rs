use itertools::Itertools;

fn main() {
    let map: Vec<Vec<bool>> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.bytes().map(|b| b == b'#').collect::<Vec<_>>())
        .collect();

    println!(
        "Max: {}",
        (0..map[0].len())
            .cartesian_product(0..map.len())
            .filter(|(x, y)| map[*x][*y])
            .map(|(x, y)| {
                (0..map[0].len())
                    .cartesian_product(0..map.len())
                    .filter(|(xx, yy)| map[*xx][*yy] && !(*xx == x && *yy == y))
                    .map(|(xx, yy)| (x as f64 - xx as f64).atan2(y as f64 - yy as f64))
                    .map(integer_decode)
                    .unique()
                    .count()
            })
            .max()
            .unwrap(),
    );
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Parts(u64, i16, i8);

fn integer_decode(val: f64) -> Parts {
    let bits: u64 = unsafe { std::mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    Parts(mantissa, exponent, sign)
}
