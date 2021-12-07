use atoi::atoi;
use nom::*;

pub fn main() {
    let (mut map, mut overlaps) = (vec![0u8; 1000 * 1000], 0);
    include_bytes!("../input.txt")
        .split(|b| *b == b'\n')
        .map(|entry| {
            let ((x, y), (xx, yy)) = line(entry).unwrap().1;
            (x.min(xx), y.min(yy), x.max(xx), y.max(yy))
        })
        .for_each(|(x, y, xx, yy)| {
            let mut mark = |x, y| {
                if map[(x + y * 1000) as usize] == 1 {
                    overlaps += 1;
                }
                map[(x + y * 1000) as usize] += 1;
            };
            if x == xx {
                (y..=yy).for_each(|y| mark(x, y));
            } else if y == yy {
                (x..=xx).for_each(|x| mark(x, y));
            }
        });

    println!("{}", overlaps);
}

named!(usize<&[u8], usize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (usize, usize)>, separated_pair!(usize, char!(','), usize));
named!(line<&[u8], ((usize, usize), (usize, usize))>, separated_pair!(coord, tag!(" -> "), coord));
