use atoi::atoi;
use nom::*;

pub fn main() {
    let map = include_bytes!("../input.txt")
        .split(|b| *b == b'\n')
        .map(|entry| {
            let ((x, y), (xx, yy)) = line(entry).unwrap().1;
            (x.min(xx), y.min(yy), x.max(xx), y.max(yy))
        })
        .filter(|(x, y, xx, yy)| x == xx || y == yy)
        .fold(vec![0u8; 1000 * 1000], |mut map, (x, y, xx, yy)| {
            if x == xx {
                (y..=yy).for_each(|y| map[x + y * 1000] += 1);
            } else {
                (x..=xx).for_each(|x| map[x + y * 1000] += 1);
            }
            map
        });

    println!("{}", map.into_iter().filter(|c| *c >= 2).count());
}

named!(usize<&[u8], usize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (usize, usize)>, separated_pair!(usize, char!(','), usize));
named!(line<&[u8], ((usize, usize), (usize, usize))>, separated_pair!(coord, tag!(" -> "), coord));
