pub fn main() {
    let mut boxes = Vec::new();
    let tot = include_str!("../input.txt")
        .lines()
        .map(|line| -> (bool, [(i32, i32); 3]) {
            let (on, coords) = line.split_once(' ').unwrap();
            let coords = coords
                .splitn(3, ',')
                .map(|c| {
                    let (a, b) = c[2..].split_once("..").unwrap();
                    (a.parse().unwrap(), b.parse().unwrap())
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            (on == "on", coords)
        })
        .filter(|(_, c)| c.iter().all(|c| (c.0 <= 50 && c.1 >= -50)))
        .map(|(on, c)| -> (bool, [(i32, i32); 3]) {
            (
                on,
                c.iter()
                    .map(|c| (c.0.max(-50), c.1.min(50)))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .fold(0, |mut tot, (on, c)| {
            let cuboid = [[c[0].0, c[1].0, c[2].0], [c[0].1, c[1].1, c[2].1]];
            if !boxes.is_empty() {
                let mut new = vec![];
                for (boxx, intersect) in boxes
                    .iter_mut()
                    .filter_map(|b| intersect(&cuboid, b).map(|i| (b, i)))
                {
                    cut(boxx, &intersect, &mut new);
                    *boxx = [[0, 0, 0], [i32::MIN, i32::MIN, i32::MIN]];
                    tot -= (0..3)
                        .map(|c| intersect[1][c] as i64 - intersect[0][c] as i64 + 1)
                        .product::<i64>() as u64;
                }
                boxes.append(&mut new);
            }
            if on {
                boxes.push(cuboid);
                tot += (0..3)
                    .map(|c| cuboid[1][c] as i64 - cuboid[0][c] as i64 + 1)
                    .product::<i64>() as u64 as u64;
            }
            tot
        });
    println!("{}", tot);
}

fn intersect(a: &[[i32; 3]; 2], b: &[[i32; 3]; 2]) -> Option<[[i32; 3]; 2]> {
    Some([
        [
            a[0][0].max(b[0][0]),
            a[0][1].max(b[0][1]),
            a[0][2].max(b[0][2]),
        ],
        [
            a[1][0].min(b[1][0]),
            a[1][1].min(b[1][1]),
            a[1][2].min(b[1][2]),
        ],
    ])
    .filter(|c| c[0].iter().zip(c[1].iter()).all(|(a, b)| a <= b))
}

fn cut(boxx: &[[i32; 3]; 2], cut: &[[i32; 3]; 2], new: &mut Vec<[[i32; 3]; 2]>) {
    if cut[0][0] > boxx[0][0] {
        new.push([boxx[0], [cut[0][0] - 1, boxx[1][1], boxx[1][2]]]);
    }
    if cut[1][0] < boxx[1][0] {
        new.push([[cut[1][0] + 1, boxx[0][1], boxx[0][2]], boxx[1]]);
    }
    if cut[1][1] < boxx[1][1] {
        new.push([
            [cut[0][0], cut[1][1] + 1, boxx[0][2]],
            [cut[1][0], boxx[1][1], boxx[1][2]],
        ]);
    }
    if cut[0][1] > boxx[0][1] {
        new.push([
            [cut[0][0], boxx[0][1], boxx[0][2]],
            [cut[1][0], cut[0][1] - 1, boxx[1][2]],
        ]);
    }
    if cut[0][2] > boxx[0][2] {
        new.push([
            [cut[0][0], cut[0][1], boxx[0][2]],
            [cut[1][0], cut[1][1], cut[0][2] - 1],
        ]);
    }
    if cut[1][2] < boxx[1][2] {
        new.push([
            [cut[0][0], cut[0][1], cut[1][2] + 1],
            [cut[1][0], cut[1][1], boxx[1][2]],
        ]);
    }
}
