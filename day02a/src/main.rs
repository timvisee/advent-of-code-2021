pub fn main() {
    let (f, d) = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(f, d), (k, v)| {
            match (k, v.parse::<i32>().unwrap()) {
                ("forward", v) => (f + v, d),
                ("down", v) => (f, d + v),
                ("up", v) => (f, d - v),
                _ => unreachable!(),
            }
        });

    println!("{}", f * d);
}
