use pathfinding::directed::dijkstra;

const NEXT: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn main() {
    let map: Vec<Vec<_>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect();
    let goal = (map[0].len() as i32 - 1, map.len() as i32 - 1);

    println!(
        "{}",
        dijkstra::dijkstra(
            &(0, 0),
            |(x, y)| {
                NEXT.iter()
                    .map(|(xx, yy)| {
                        map.get((y + yy) as usize)
                            .and_then(|r| r.get((x + xx) as usize))
                            .map(|c| ((x + xx, y + yy), *c as u32))
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
