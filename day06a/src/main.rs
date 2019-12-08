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

    println!(
        "Count: {}",
        map.keys().map(|i| count(&map, i)).sum::<usize>()
    );
}

#[inline]
fn count(map: &HashMap<&str, &str>, item: &str) -> usize {
    map.get(item).map(|item| count(map, item) + 1).unwrap_or(0)
}
