pub fn main() {
    let subs = include_str!("../input.txt")
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mean = (subs.iter().sum::<i32>() as f32 / subs.len() as f32).round() as i32;
    println!(
        "{}",
        subs.iter()
            .map(|n| {
                let d = (n - mean).abs();
                d * (d + 1) / 2
            })
            .sum::<i32>()
    );
}
