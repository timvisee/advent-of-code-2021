pub fn main() {
    let (x, y) = include_str!("../input.txt")
        .trim()
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .unwrap();
    let (x, y) = (x.split_once("..").unwrap(), y.split_once("..").unwrap());
    let target: (_, i32, _, _) = (
        x.0.parse().unwrap(),
        y.0.parse().unwrap(),
        x.1.parse().unwrap(),
        y.1.parse().unwrap(),
    );

    println!(
        "{}",
        (1..=target.2)
            .flat_map(|vx| {
                let range = target.1.abs();
                (-range..=range).filter(move |&vy| fire(target, (vx, vy)))
            })
            .count()
    );
}

fn fire(target: (i32, i32, i32, i32), mut v: (i32, i32)) -> bool {
    let mut p = (0, 0);
    if v.1 > 1 {
        for _ in 0..v.1 * 2 + 1 {
            p.0 += v.0;
            v.0 -= 1;
            if p.0 > target.2 {
                return false;
            } else if v.0 == 0 {
                break;
            }
        }
        v.1 = -v.1 - 1;
    }
    for (x, y, vx, _) in path(p, v) {
        if vx == 0 && x < target.0 || x > target.2 || y < target.1 {
            return false;
        } else if x >= target.0 && x <= target.2 && y >= target.1 && y <= target.3 {
            return true;
        }
    }
    unreachable!();
}

fn path(p: (i32, i32), v: (i32, i32)) -> impl Iterator<Item = (i32, i32, i32, i32)> {
    std::iter::successors(Some((p.0, p.1, v.0, v.1)), |p| {
        Some((p.0 + p.2, p.1 + p.3, (p.2 - 1).max(0), p.3 - 1))
    })
}
