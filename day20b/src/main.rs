use std::{iter, mem};

const RUNS: usize = 50;

pub fn main() {
    let input = include_str!("../input.txt").trim();
    let (key, map) = input.split_once("\n\n").unwrap();
    let size = map.bytes().position(|b| b == b'\n').unwrap();
    let key = key.bytes().map(&|b| b == b'#').collect::<Vec<_>>();

    let mut map = iter::repeat(vec![false; size + RUNS * 2])
        .take(RUNS)
        .chain(map.lines().map(|l| {
            iter::repeat(b'.')
                .take(RUNS)
                .chain(l.bytes())
                .chain(iter::repeat(b'.').take(RUNS))
                .map(&|b| b == b'#')
                .collect::<Vec<_>>()
        }))
        .chain(iter::repeat(vec![false; size + RUNS * 2]).take(RUNS))
        .collect::<Vec<Vec<bool>>>();
    let mut other = map.clone();

    for run in 0..RUNS {
        let out = run % 2 == 1 && key[0];
        (0..map.len()).for_each(|y| {
            let rows = [
                map.get((y as isize - 1) as usize),
                map.get(y),
                map.get(y + 1),
            ];
            let mut i = rows.iter().fold(0, |i, row| {
                row.map(|row| {
                    (-1isize..=1)
                        .map(|x| *row.get(x as usize).unwrap_or(&out) as usize)
                        .fold(0usize, |i, c| i << 1 | c)
                })
                .unwrap_or(out as usize * 7)
                    | i << 3
            });

            (0..map[0].len()).for_each(|x| {
                let new = (*rows[0].and_then(|r| r.get(x + 1)).unwrap_or(&out) as usize) << 6
                    | (*rows[1].and_then(|r| r.get(x + 1)).unwrap_or(&out) as usize) << 3
                    | (*rows[2].and_then(|r| r.get(x + 1)).unwrap_or(&out) as usize);
                i = i & 0b110110110 | new;
                other[y][x] = key[i];
                i <<= 1;
            });
        });
        mem::swap(&mut map, &mut other);
    }

    println!(
        "{}",
        map.iter()
            .map(|r| r.iter().filter(|b| **b).count())
            .sum::<usize>()
    );
}
