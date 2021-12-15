#![feature(int_abs_diff)]

use pathfinding::directed::dijkstra;

const NEXT: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn main() {
    let map: Vec<Vec<_>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect();
    let (w, h) = (map[0].len(), map.len());
    let goal = (w as i32 * 5 - 1, h as i32 * 5 - 1);

    println!(
        "{}",
        dijkstra::dijkstra(
            &(0, 0),
            |&(x, y)| -> Vec<((_, _), _)> {
                NEXT.iter()
                    .map(|&(xx, yy)| ((x + xx) as usize, (y + yy) as usize))
                    .filter(|(x, y)| (x / 5 < w && y / 5 < h))
                    .map(|(x, y)| -> Option<((i32, i32), u32)> {
                        map.get(y % h).and_then(|r| r.get(x % w)).map(|c| {
                            (
                                (x as i32, y as i32),
                                (((*c as usize + (x / w) + (y / h) - 1) % 9) + 1) as u32,
                            )
                        })
                    })
                    .flatten()
                    .collect()
            },
            |&p| p == goal,
        )
        .unwrap()
        .1
    );
}
