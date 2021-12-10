#[allow(clippy::unit_cmp)]

pub fn main() {
    let mut scores = include_str!("../input.txt")
        .lines()
        .filter_map(|seq| {
            let mut stack = Vec::with_capacity(110);
            seq.bytes()
                .all(|c| match c {
                    c if matches!(c, b'(' | b'[' | b'{' | b'<') => stack.push(c) == (),
                    b')' => stack.pop().unwrap() == b'(',
                    c => stack.pop().unwrap() == c - 2,
                })
                .then(|| stack)
        })
        .map(|s| {
            s.iter()
                .rev()
                .map(|c| match c {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!(),
                })
                .fold(0usize, |acc, v| acc * 5 + v)
        })
        .collect::<Vec<_>>();

    let mid = scores.len() / 2;
    println!("{}", scores.select_nth_unstable(mid).1);
}
