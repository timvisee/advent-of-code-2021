use std::collections::BTreeMap;

const EDGES: usize = 12;

pub fn main() {
    let mut uc = vec![];
    let mut id: BTreeMap<&str, u8> = BTreeMap::from([("start", 1), ("end", 0)]);
    let mut map: BTreeMap<u8, Vec<u8>> = BTreeMap::new();

    include_str!("../input.txt").lines().for_each(|l| {
        let mut idx = |a| {
            let idx = id.len() as u8;
            *id.entry(a).or_insert_with(|| {
                (a.as_bytes()[0] <= b'Z').then(|| uc.push(idx));
                idx
            })
        };
        let mut branch = |a, b| {
            let entry = map.entry(a).or_insert_with(|| Vec::with_capacity(6));
            (b != 0).then(|| entry.push(b));
        };
        let (a, b) = l.split_once('-').unwrap();
        let (a, b) = (idx(a), idx(b));
        branch(a, b);
        branch(b, a);
    });

    let map = map
        .keys()
        .filter(|b| !uc.contains(b))
        .map(|&b| {
            (
                b,
                map[&b].iter().fold([0; EDGES], |mut chld, b| {
                    if uc.contains(b) {
                        map[b].iter().for_each(|b| chld[*b as usize] += 1);
                    } else {
                        chld[*b as usize] += 1;
                    }
                    chld
                }),
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut todo: Vec<(u8, u8, bool, usize)> = vec![(0, 1, true, 1)];
    let (mut to, mut count) = ([1; EDGES], 0);
    while let Some((a, b, t, s)) = todo.pop() {
        to[b as usize] = a;
        count += map[&a]
            .iter()
            .enumerate()
            .filter(|&(_, routes)| *routes > 0)
            .fold(0, |acc, (c, _)| match c {
                1 => acc + s * map[&a][c],
                v => {
                    let visit = v != 0 && to[2..=b as usize].contains(&(v as u8));
                    (t || !visit).then(|| todo.push((v as u8, b + 1, t && !visit, s * map[&a][v])));
                    acc
                }
            });
    }

    println!("{}", count);
}
