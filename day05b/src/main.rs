use std::iter;

pub fn main() {
    let map = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            let ((x, y), (xx, yy)) = (from.split_once(',').unwrap(), to.split_once(',').unwrap());
            (
                x.parse().unwrap(),
                y.parse().unwrap(),
                xx.parse().unwrap(),
                yy.parse().unwrap(),
            )
        })
        .fold(vec![vec![0u8; 1000]; 1000], |mut map, (x, y, xx, yy)| {
            let range =
                |a: isize, b: isize| iter::successors(Some(a), move |n| Some(n + (b - a).signum()));
            range(x, xx)
                .cycle()
                .zip(range(y, yy).cycle())
                .take((x - xx).abs().max((y - yy).abs()) as usize + 1)
                .for_each(|(x, y)| map[y as usize][x as usize] += 1);
            map
        });

    println!("{}", map.into_iter().flatten().filter(|c| *c >= 2).count());
}
