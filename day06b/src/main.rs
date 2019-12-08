use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let map: HashMap<&str, &str> = file
        .lines()
        .map(|l| l.split(')').collect::<Vec<&str>>())
        .fold(HashMap::new(), |mut map, o| {
            map.insert(o[1], o[0]);
            map
        });

    println!("Distance: {}", distance(&map, "YOU", "SAN"));
}

#[inline]
fn distance(map: &HashMap<&str, &str>, from: &str, to: &str) -> usize {
    let mut route = vec![from];
    while let Some(item) = map.get(route.last().unwrap()) {
        route.push(item);
    }

    let mut count = 0;
    let mut last = to;
    while let Some(item) = map.get(last) {
        if let Some(i) = route.iter().position(|i| item == i) {
            return count + i;
        }
        last = item;
        count += 1;
    }
    panic!("No link found");
}
