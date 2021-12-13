#![feature(stdio_locked)]
use std::{io::Write, iter::once};

const S: [usize; 2] = [40, 6];

pub fn main() {
    let (coords, folds) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let folds = folds
        .lines()
        .map(|l| l.trim_start_matches("fold along ").split_once('=').unwrap())
        .map(|(c, i)| (c.as_bytes()[0], i.parse::<i16>().unwrap()))
        .collect::<Vec<_>>();

    std::io::stdout_locked()
        .write_all(
            &coords
                .lines()
                .map(|l| l.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap()))
                .filter_map(|(mut x, mut y)| {
                    for &(c, i) in &folds {
                        match c {
                            b'x' if x == i => return None,
                            b'x' if x > i => x = i - (x - i),
                            b'y' if y == i => return None,
                            b'y' if y > i => y = i - (y - i),
                            _ => {}
                        }
                    }
                    Some((x, y))
                })
                .fold(vec![0u64; S[1]], |mut map, (x, y)| {
                    map[y as usize] |= 1 << x;
                    map
                })
                .iter()
                .flat_map(|row| {
                    (0..S[0])
                        .map(move |x| b' ' + ((row & 1 << x) >> x) as u8 * 3)
                        .chain(once(b'\n'))
                })
                .collect::<Vec<_>>(),
        )
        .unwrap();
}
