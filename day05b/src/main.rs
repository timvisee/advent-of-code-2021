use atoi::atoi;
use nom::*;
use std::iter;

pub fn main() {
    let map = include_bytes!("../input.txt").split(|b| *b == b'\n').fold(
        vec![0u8; 1000 * 1000],
        |mut map, entry| {
            let ((x, y), (xx, yy)) = line(entry).unwrap().1;
            let range =
                |a: isize, b: isize| iter::successors(Some(a), move |n| Some(n + (b - a).signum()));
            range(x, xx)
                .cycle()
                .zip(range(y, yy).cycle())
                .take((x - xx).abs().max((y - yy).abs()) as usize + 1)
                .for_each(|(x, y)| map[(x + y * 1000) as usize] += 1);
            map
        },
    );

    println!("{}", map.into_iter().filter(|c| *c >= 2).count());
}

named!(isize<&[u8], isize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (isize, isize)>, separated_pair!(isize, char!(','), isize));
named!(line<&[u8], ((isize, isize), (isize, isize))>, separated_pair!(coord, tag!(" -> "), coord));
