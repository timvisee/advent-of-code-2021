#[allow(clippy::unit_cmp)]

pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .filter_map(|seq| seq
                .bytes()
                .scan(Vec::with_capacity(64), |s, c| Some(match c {
                    c if matches!(c, b'(' | b'[' | b'{' | b'<') => (s.push(c) != ()).then(|| b' '),
                    b')' => (s.pop().unwrap() != b'(').then(|| b')'),
                    c => (s.pop().unwrap() != c - 2).then(|| c),
                }))
                .skip_while(Option::is_none)
                .map(|c| [3, 25137, 57, 1197][c.unwrap() as usize / 30 - 1])
                .next())
            .sum::<usize>()
    );
}
