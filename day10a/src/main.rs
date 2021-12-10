#[allow(clippy::unit_cmp)]

pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .filter_map(|seq| seq
                .bytes()
                .scan(Vec::with_capacity(110), |s, c| Some(match c {
                    c if matches!(c, b'(' | b'[' | b'{' | b'<') => (s.push(c) != ()).then(|| b' '),
                    b')' => (s.pop().unwrap() != b'(').then(|| b')'),
                    c => (s.pop().unwrap() != c - 2).then(|| c),
                }))
                .skip_while(Option::is_none)
                .map(|c| match c.unwrap() {
                    b')' => 3,
                    b']' => 57,
                    b'}' => 1197,
                    b'>' => 25137,
                    _ => unreachable!(),
                })
                .next())
            .sum::<usize>()
    );
}
