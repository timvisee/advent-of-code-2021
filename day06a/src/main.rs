pub fn main() {
    let mut map = include_str!("../input.txt")
        .split(',')
        .fold([0; 9], |mut map, n| {
            map[n.parse::<usize>().unwrap()] += 1;
            map
        });

    (1..80).for_each(|day| map[(day + 7) % 9] += map[day % 9]);

    println!("{}", map.iter().sum::<usize>());
}
