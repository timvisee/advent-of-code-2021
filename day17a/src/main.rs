pub fn main() {
    let (x, y) = include_str!("../input.txt")
        .trim()
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .unwrap();
    let (x, y) = (x.split_once("..").unwrap(), y.split_once("..").unwrap());
    let target: (isize, isize, isize, isize) = (
        x.0.parse().unwrap(),
        y.0.parse().unwrap(),
        x.1.parse().unwrap(),
        y.1.parse().unwrap(),
    );

    println!(
        "{}",
        (1..=target.2)
            .flat_map(|vx| (1..=target.1.abs()).flat_map(move |vy| fire(target, (vx, vy))))
            .max()
            .unwrap()
    );
}

fn fire(target: (isize, isize, isize, isize), v: (isize, isize)) -> Option<isize> {
    let mut high = 0;
    for (x, y, _, _) in path(v) {
        if x > target.2 || y < target.1 {
            return None;
        } else if x >= target.0 && x <= target.2 && y >= target.1 && y <= target.3 {
            break;
        }
        high = high.max(y);
    }
    Some(high)
}

fn path(v: (isize, isize)) -> impl Iterator<Item = (isize, isize, isize, isize)> {
    std::iter::successors(Some((0, 0, v.0, v.1)), |p| {
        Some((p.0 + p.2, p.1 + p.3, (p.2 - 1).max(0), p.3 - 1))
    })
}
