pub fn main() {
    let (f, d, _) = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0, 0), |(f, d, a), (k, v)| {
            match (k, v.parse::<i32>().unwrap()) {
                ("forward", v) => (f + v, d + a * v, a),
                ("down", v) => (f, d, a + v),
                ("up", v) => (f, d, a - v),
                _ => unreachable!(),
            }
        });

    println!("{}", f * d);
}
