#![feature(drain_filter, btree_drain_filter)]

use std::cell::UnsafeCell;
use std::collections::BTreeMap;

pub fn main() {
    let data = include_str!("../input.txt");
    let mut id = data
        .lines()
        .flat_map(|l| l.splitn(2, '-'))
        .collect::<Vec<_>>();
    id.sort_unstable();
    id.dedup();

    let start = id.binary_search(&"start").unwrap() as u8;
    let end = id.binary_search(&"end").unwrap() as u8;
    let mut edge: UnsafeCell<BTreeMap<_, Vec<_>>> =
        UnsafeCell::new(data.lines().fold(BTreeMap::new(), |mut m, l| {
            let (a, b) = l.split_once('-').unwrap();
            let a = (id.binary_search(&a).unwrap() as u8, a.as_bytes()[0] <= b'Z');
            let b = (id.binary_search(&b).unwrap() as u8, b.as_bytes()[0] <= b'Z');
            m.entry(a.0).or_default().push(b);
            m.entry(b.0).or_default().push(a);
            m
        }));

    // Eliminate large caves from input, map to all possible small caves
    (0..id.len() as u8)
        .filter(|e| id[*e as usize].as_bytes()[0] > b'Z')
        .for_each(|e| {
            edge.get_mut()
                .get_mut(&e)
                .unwrap()
                .drain_filter(|next| next.1)
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(rm, _)| unsafe {
                    // Safe because we fetch/update different values
                    (&mut *edge.get())
                        .get_mut(&e)
                        .unwrap()
                        .extend((&*edge.get()).get(&rm).unwrap())
                });
        });
    edge.get_mut()
        .drain_filter(|&c, _| id[c as usize].as_bytes()[0] <= b'Z');

    let mut vis = Vec::from([end]);
    vis.reserve(10);
    println!("{}", path(&edge.into_inner(), start, end, &mut vis));
}

fn path(m: &BTreeMap<u8, Vec<(u8, bool)>>, start: u8, cur: u8, vis: &mut Vec<u8>) -> usize {
    m.get(&cur).unwrap().iter().fold(0, |acc, &(b, u)| match b {
        next if next == start => acc + 1,
        next if !u && vis.contains(&next) => acc,
        next => {
            let len = vis.len();
            vis.push(next);
            let paths = path(m, start, next, vis);
            vis.truncate(len);
            acc + paths
        }
    })
}
