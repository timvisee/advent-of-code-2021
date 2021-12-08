pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .flat_map(|l| l.split_once('|').unwrap().1.split_ascii_whitespace())
            .filter(|d| matches!(d.len(), 2 | 3 | 4 | 7))
            .count()
    );
}
