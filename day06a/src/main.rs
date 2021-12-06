use std::collections::BTreeMap;

pub fn main() {
    let mut map = include_str!("../input.txt")
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .fold(BTreeMap::new(), |mut map, n| {
            *map.entry(n).or_insert(0) += 1;
            map
        });

    for day in 1..80 {
        if let Some(n) = map.remove(&day) {
            *map.entry(day + 7).or_insert(0) += n;
            map.insert(day + 9, n);
        }
    }

    println!("{}", map.values().sum::<usize>());
}
