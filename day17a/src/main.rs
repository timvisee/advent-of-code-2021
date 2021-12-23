pub fn main() {
    let n = -include_str!("../input.txt")
        .trim()
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .unwrap()
        .1
        .splitn(2, "..")
        .map(|n| n.parse::<i32>().unwrap())
        .min()
        .unwrap()
        - 1;
    println!("{}", (n * (n + 1)) / 2);
}
