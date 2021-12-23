pub fn main() {
    let y = include_str!("../input.txt")
        .trim()
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .unwrap()
        .1
        .split_once("..")
        .unwrap();
    let n = -y.0.parse::<i32>().unwrap().min(y.1.parse::<i32>().unwrap()) - 1;
    println!("{}", (n * (n + 1)) / 2);
}
