pub fn main() {
    let (base, rules) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut rules = rules
        .lines()
        .map(|l| {
            let (k, t) = l.split_once(" -> ").unwrap();
            let (k, t) = (k.as_bytes(), t.as_bytes());
            ([k[0], k[1]], [t[0], k[1]])
        })
        .collect::<Vec<([u8; 2], [u8; 2])>>();
    rules.sort_unstable_by_key(|r| r.0);

    let mut base = base.as_bytes().to_vec();
    let mut next = vec![base[0]];
    base.reserve(20_000);
    next.reserve(20_000);

    for _ in 0..10 {
        base.windows(2).for_each(|key| {
            next.extend_from_slice(&rules[rules.binary_search_by_key(&key, |r| &r.0).unwrap()].1);
        });
        std::mem::swap(&mut next, &mut base);
        next.truncate(1);
    }

    let mut counts = [0u32; (b'Z' - b'A') as usize];
    base.iter().for_each(|b| counts[(b - b'A') as usize] += 1);
    let (max, min) = counts
        .iter()
        .copied()
        .filter(|&b| b != 0)
        .fold((0, u32::MAX), |(min, max), b| (min.max(b), max.min(b)));
    println!("{}", max - min);
}
