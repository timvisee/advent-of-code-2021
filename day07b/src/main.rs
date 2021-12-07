pub fn main() {
    let subs = include_str!("../input.txt")
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

    println!(
        "{}",
        (subs.iter().sum::<i32>() / subs.len() as i32..)
            .take(2)
            .map(|t| {
                subs.iter()
                    .map(|n| {
                        let d = (n - t).abs();
                        d * (d + 1) / 2
                    })
                    .sum::<i32>()
            })
            .min()
            .unwrap()
    );
}
