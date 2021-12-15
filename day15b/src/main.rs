use pathfinding::directed::dijkstra;

const NEXT: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn main() {
    let map: Vec<Vec<_>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect();
    let s = map.len();
    let goal = (s as i32 * 5 - 1, s as i32 * 5 - 1);

    println!(
        "{}",
        dijkstra::dijkstra(
            &(0, 0),
            |&(x, y)| {
                NEXT.iter()
                    .map(|&(xx, yy)| ((x + xx) as usize, (y + yy) as usize))
                    .filter(|(x, y)| (x / 5 < s && y / 5 < s))
                    .map(|(x, y)| {
                        map.get(y % s).and_then(|r| r.get(x % s)).map(|c| {
                            (
                                (x as i32, y as i32),
                                ((*c as usize + (x / s) + (y / s) - 1) % 9 + 1) as u32,
                            )
                        })
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            },
            |&p| p == goal,
        )
        .unwrap()
        .1,
    );
}
