// Used strategy from: https://is.gd/j9dCKv

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use nalgebra::{Matrix3, Rotation3, Vector3};

type Fp = (i32, i32);

pub fn main() {
    let mut rots: HashSet<_> = [Matrix3::identity()].into_iter().collect();
    [Vector3::x_axis(), Vector3::y_axis(), Vector3::z_axis()]
        .iter()
        .for_each(|axis| {
            (0..4)
                .map(|n| std::f32::consts::FRAC_PI_2 * n as f32)
                .for_each(|angle| {
                    let r = Matrix3::from_iterator(
                        Rotation3::from_axis_angle(axis, angle)
                            .matrix()
                            .iter()
                            .map(|n| n.round() as i32),
                    );
                    rots.extend(rots.clone().into_iter().map(|m| r * m));
                });
        });

    let scans = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|l| {
                    let mut c = l.splitn(3, ',').map(|c| c.parse().unwrap());
                    [c.next().unwrap(), c.next().unwrap(), c.next().unwrap()].into()
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    let mut scans = scans
        .iter()
        .map(|scan| {
            (
                scan,
                scan.iter()
                    .tuple_combinations()
                    .map(|(a, b)| (fp(a, b), [a, b]))
                    .collect(),
            )
        })
        .collect::<Vec<(_, Vec<_>)>>();

    let first = scans.remove(0);
    let mut scans_pos = vec![[0, 0, 0].into()];
    let mut beacons = first.0.iter().cloned().collect();
    let mut fps = HashMap::new();
    add_scan_fps(&mut fps, first.0);

    while !scans.is_empty() {
        let (i, (pos, scan)) = scans
            .iter()
            .enumerate()
            .flat_map(|(i, (s, s_fps))| matching(&beacons, &fps, &rots, s, s_fps).map(|m| (i, m)))
            .next()
            .unwrap();
        scans.remove(i);
        add_scan_fps(&mut fps, &scan);
        beacons.extend(scan.into_iter());
        scans_pos.push(pos);
    }

    println!(
        "{}",
        scans_pos
            .into_iter()
            .tuple_combinations()
            .map(|(a, b)| (a - b).iter().map(|n| n.abs()).sum::<i32>())
            .max()
            .unwrap()
    );
}

fn fp(a: &Vector3<i32>, b: &Vector3<i32>) -> Fp {
    (
        (a - b).iter().map(|n| n.abs()).sum(),
        (a - b).iter().map(|n| n.abs()).max().unwrap(),
    )
}

fn add_scan_fps(fps: &mut HashMap<Fp, Vec<[Vector3<i32>; 2]>>, scan: &[Vector3<i32>]) {
    scan.iter().tuple_combinations().for_each(|(a, b)| {
        fps.entry(fp(a, b)).or_default().push([*a, *b]);
    })
}

fn matching(
    beacons: &HashSet<Vector3<i32>>,
    fps: &HashMap<Fp, Vec<[Vector3<i32>; 2]>>,
    rots: &HashSet<Matrix3<i32>>,
    scan: &[Vector3<i32>],
    scan_fps: &[(Fp, [&Vector3<i32>; 2])],
) -> Option<(Vector3<i32>, Vec<Vector3<i32>>)> {
    match scan_fps
        .iter()
        .filter(|(f, _)| fps.contains_key(f))
        .take(12)
        .collect::<Vec<_>>()
    {
        match_fps if match_fps.len() < 12 => None,
        match_fps => match_fps
            .into_iter()
            .flat_map(|(fp, [ma, mb])| {
                fps[fp]
                    .iter()
                    .flat_map(|[a, b]| {
                        rots.iter().find(|&m| a - m * *ma == b - m * *mb).map(|r| {
                            let t = a - r * *ma;
                            (t, scan.iter().map(|p| r * p + t).collect::<Vec<_>>())
                        })
                    })
                    .filter(|(_, s)| s.iter().filter(|b| beacons.contains(b)).take(2).count() >= 2)
            })
            .next(),
    }
}
