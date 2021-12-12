#[rustfmt::skip]
const NEXT: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
const SIZE: usize = 10;

pub fn main() {
    let mut m = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..100).fold(0, |acc, _| {
            m.iter_mut()
                .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));
            acc + (0..SIZE)
                .flat_map(|y| (0..SIZE).map(move |x| (x, y)))
                .fold(0, |acc, (x, y)| {
                    acc + (m[y][x] > b'9').then(|| flash(&mut m, x, y)).unwrap_or(0)
                })
        }),
    );
}

fn flash(map: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    map[y][x] = b'0';
    NEXT.iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            match map.get_mut(y).and_then(|l| l.get_mut(x)) {
                Some(cell) if *cell > b'0' => {
                    *cell += 1;
                    acc + (*cell > b'9').then(|| flash(map, x, y)).unwrap_or(0)
                }
                _ => acc,
            }
        })
}
