pub fn main() {
    let subs = include_str!("../input.txt")
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..=*subs.iter().max().unwrap())
            .map(|t| subs.iter().map(|n| (n - t).abs()).sum::<i32>())
            .min()
            .unwrap()
    );
}
