#![feature(int_abs_diff)]

use atoi::atoi;
use nom::*;
use std::iter;

pub fn main() {
    let (mut map, mut overlaps) = (vec![0u8; 1000 * 1000], 0);
    include_bytes!("../input.txt")
        .split(|b| *b == b'\n')
        .for_each(|entry| {
            let ((x, y), (xx, yy)) = line(entry).unwrap().1;
            let range =
                |a: isize, b: isize| iter::successors(Some(a), move |n| Some(n + (b - a).signum()));
            range(x, xx)
                .zip(range(y, yy))
                .take(x.abs_diff(xx).max(y.abs_diff(yy)) + 1)
                .for_each(|(x, y)| {
                    if map[(x + y * 1000) as usize] == 1 {
                        overlaps += 1;
                    }
                    map[(x + y * 1000) as usize] += 1;
                });
        });

    println!("{}", overlaps);
}

named!(isize<&[u8], isize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (isize, isize)>, separated_pair!(isize, char!(','), isize));
named!(line<&[u8], ((isize, isize), (isize, isize))>, separated_pair!(coord, tag!(" -> "), coord));
