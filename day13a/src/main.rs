pub fn main() {
    let (coords, folds) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let fold = folds.split_once('\n').unwrap().0;
    let (c, i) = fold[11..].split_once('=').unwrap();
    let (c, i) = (c.as_bytes()[0], i.parse().unwrap());

    let mut coords = coords
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap()))
        .filter_map(|(x, y)| match c {
            b'x' if x == i => None,
            b'x' if x > i => Some((i - (x - i), y)),
            b'y' if y == i => None,
            b'y' if y > i => Some((x, i - (y - i))),
            _ => Some((x, y)),
        })
        .collect::<Vec<_>>();
    coords.sort_unstable();
    coords.dedup();

    println!("{}", coords.len());
}
