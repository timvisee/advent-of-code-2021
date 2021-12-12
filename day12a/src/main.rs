use std::collections::HashMap;

pub fn main() {
    let branches = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .fold(HashMap::new(), |mut m, (a, b)| {
            m.entry(a).or_insert_with(|| Vec::with_capacity(6)).push(b);
            m.entry(b).or_insert_with(|| Vec::with_capacity(6)).push(a);
            m
        });

    let mut vis = Vec::from(["end"]);
    vis.reserve(11);
    println!("{}", path(&branches, "end", &mut vis));
}

fn path(m: &HashMap<&str, Vec<&'static str>>, cur: &str, vis: &mut Vec<&str>) -> usize {
    m.get(cur).unwrap().iter().fold(0, |acc, &b| match b {
        "start" => acc + 1,
        next if next.as_bytes()[0] > b'Z' && vis.contains(&next) => acc,
        next => {
            let len = vis.len();
            vis.push(next);
            let paths = path(m, next, vis);
            vis.truncate(len);
            acc + paths
        }
    })
}
