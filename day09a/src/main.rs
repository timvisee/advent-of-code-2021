pub fn main() {
    let map = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();
    let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut sum = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if neighbors.iter().all(|&(xx, yy)| {
                map.get((y as isize + yy) as usize)
                    .and_then(|l| l.get((x as isize + xx) as usize))
                    .map(|n| cell < n)
                    .unwrap_or(true)
            }) {
                sum += (cell - b'0') as usize + 1;
            }
        }
    }
    println!("{sum}");
}
