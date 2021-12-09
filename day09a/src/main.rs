pub fn main() {
    let input = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();
    let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut sum = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if neighbors.iter().all(|&(xx, yy)| {
                input
                    .get(y.overflowing_add(yy as usize).0)
                    .and_then(|l| l.get(x.overflowing_add(xx as usize).0))
                    .map(|n| cell < n)
                    .unwrap_or(true)
            }) {
                sum += (cell - b'0') as usize + 1;
            }
        }
    }
    println!("{sum}");
}
