pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|seq| seq
                .bytes()
                .scan(Vec::with_capacity(110), |stack, c| {
                    match c {
                        c if matches!(c, b'(' | b'[' | b'{' | b'<') => stack.push(c),
                        c if matches!(c, b')' | b']' | b'}' | b'>') => {
                            if match stack.pop().unwrap() {
                                b'(' => b')' != c,
                                b => b + 2 != c,
                            } {
                                return Some(Some(match c {
                                    b')' => 3,
                                    b']' => 57,
                                    b'}' => 1197,
                                    b'>' => 25137,
                                    _ => unreachable!(),
                                }));
                            }
                        }
                        _ => return None,
                    }
                    Some(None)
                })
                .skip_while(Option::is_none)
                .map(Option::unwrap)
                .next()
                .unwrap_or(0))
            .sum::<usize>()
    );
}
