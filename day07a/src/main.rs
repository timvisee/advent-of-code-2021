pub fn main() {
    let mut subs = include_str!("../input.txt")
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    let mid = subs.len() / 2;
    let med = *subs.select_nth_unstable(mid).1;

    println!("{}", subs.iter().map(|n| (n - med).abs()).sum::<i32>());
}
