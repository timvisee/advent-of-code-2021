pub fn main() {
    let map = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            let ((x, y), (xx, yy)) = (from.split_once(',').unwrap(), to.split_once(',').unwrap());
            let (x, y, xx, yy) = (
                x.parse::<usize>().unwrap(),
                y.parse::<usize>().unwrap(),
                xx.parse::<usize>().unwrap(),
                yy.parse::<usize>().unwrap(),
            );
            (x.min(xx), y.min(yy), x.max(xx), y.max(yy))
        })
        .filter(|(x, y, xx, yy)| x == xx || y == yy)
        .fold(vec![vec![0u8; 1000]; 1000], |mut map, (x, y, xx, yy)| {
            if x == xx {
                (y..=yy).for_each(|y| map[y][x] += 1);
            } else {
                (x..=xx).for_each(|x| map[y][x] += 1);
            }
            map
        });

    println!("{}", map.into_iter().flatten().filter(|c| *c >= 2).count());
}
