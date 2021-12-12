#[allow(clippy::unit_cmp)]

pub fn main() {
    let mut scores = include_str!("../input.txt")
        .lines()
        .filter_map(|seq| {
            let mut s = Vec::with_capacity(64);
            seq.bytes()
                .all(|c| match c {
                    c if matches!(c, b'(' | b'[' | b'{' | b'<') => s.push(c) == (),
                    b')' => s.pop().unwrap() == b'(',
                    c => s.pop().unwrap() == c - 2,
                })
                .then(|| s)
        })
        .map(|s| {
            s.iter().rev().fold(0usize, |acc, &c| {
                acc * 5 + [1, 4, 2, 3][c as usize / 30 - 1]
            })
        })
        .collect::<Vec<_>>();

    let mid = scores.len() / 2;
    println!("{}", scores.select_nth_unstable(mid).1);
}
