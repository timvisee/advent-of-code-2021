use std::{iter, mem};

#[rustfmt::skip]
const NEXT: [(isize, isize); 9] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1) ];
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
        let outside = run % 2 == 1 && key[0];
        (0..map.len()).for_each(|y| {
            (0..map[0].len()).clone().for_each(|x| {
                let i = NEXT.iter().fold(0, |abc, offset| {
                    *map.get(y.overflowing_add(offset.1 as usize).0)
                        .and_then(|r| r.get(x.overflowing_add(offset.0 as usize).0))
                        .unwrap_or(&outside) as usize
                        | (abc << 1)
                });
                other[y][x] = key[i];
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
