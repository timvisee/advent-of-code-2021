use std::collections::HashMap;

pub fn main() {
    let data = include_str!("../input.txt");
    let mut id = data
        .lines()
        .flat_map(|l| l.splitn(2, '-'))
        .collect::<Vec<_>>();
    id.sort_unstable();
    id.dedup();

    let start = id.binary_search(&"start").unwrap() as u8;
    let end = id.binary_search(&"end").unwrap() as u8;
    let branches: HashMap<u8, Vec<(u8, bool)>> = data.lines().fold(HashMap::new(), |mut m, l| {
        let (a, b) = l.split_once('-').unwrap();
        let a = (id.binary_search(&a).unwrap() as u8, a.as_bytes()[0] <= b'Z');
        let b = (id.binary_search(&b).unwrap() as u8, b.as_bytes()[0] <= b'Z');
        m.entry(a.0).or_default().push(b);
        m.entry(b.0).or_default().push(a);
        m
    });

    let mut vis = Vec::from([end]);
    vis.reserve(15);
    println!("{}", path(&branches, start, end, end, &mut vis, true));
}

fn path(
    m: &HashMap<u8, Vec<(u8, bool)>>,
    start: u8,
    end: u8,
    cur: u8,
    vis: &mut Vec<u8>,
    mul: bool,
) -> usize {
    m.get(&cur).unwrap().iter().fold(0, |acc, &(b, u)| match b {
        next if next == start => acc + 1,
        next if next == end => acc,
        next => {
            let seen = vis.contains(&next);
            if !u && !mul && seen {
                return acc;
            }

            let len = vis.len();
            vis.push(next);
            let paths = path(m, start, end, next, vis, mul && (u || !seen));
            vis.truncate(len);
            acc + paths
        }
    })
}
