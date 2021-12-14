use std::mem;

pub fn main() {
    let (base, rules) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let base = base.as_bytes().to_vec();
    let mut rules = rules
        .lines()
        .map(|l| {
            let (k, t) = l.split_once(" -> ").unwrap();
            let (k, t) = (k.as_bytes(), t.as_bytes()[0]);
            ([k[0], k[1]], [k[0], t])
        })
        .collect::<Vec<_>>();
    rules.sort_unstable_by_key(|r| r.0);
    let rule = rules
        .iter()
        .map(|r| {
            (
                r.0,
                rules.binary_search_by_key(&r.1, |r| r.0).unwrap(),
                rules
                    .binary_search_by_key(&[r.1[1], r.0[1]], |r| r.0)
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let (mut num, mut next) = (vec![0; rule.len()], vec![0; rule.len()]);
    base.windows(2)
        .for_each(|key| num[rule.binary_search_by_key(&key, |r| &r.0).unwrap()] += 1);

    (0..40).for_each(|_| {
        num.iter_mut().zip(&rule).for_each(|(n, r)| {
            next[r.1] += *n;
            next[r.2] += *n;
            *n = 0;
        });
        mem::swap(&mut num, &mut next);
    });

    let mut occur = [0; (b'Z' - b'A') as usize];
    occur[(base.last().unwrap() - b'A') as usize] += 1;
    rule.iter()
        .zip(num)
        .for_each(|(r, n)| occur[(r.0[0] - b'A') as usize] += n);

    let (min, max) = occur
        .iter()
        .filter(|&&n| n != 0)
        .fold((u64::MAX, 0), |(min, max), &n| (min.min(n), max.max(n)));
    println!("{}", max - min);
}
