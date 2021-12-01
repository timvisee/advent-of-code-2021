#![feature(array_windows)]

pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>()
            .array_windows()
            .filter(|[a, _, _, d]| a < d)
            .count(),
    );
}
