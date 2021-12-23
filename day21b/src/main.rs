const PROB: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

pub fn main() {
    let pos = include_str!("../input.txt")
        .lines()
        .map(|l| l[l.len() - 2..l.len()].trim().parse().unwrap())
        .collect::<Vec<_>>();

    let p = [scores(pos[0]), scores(pos[1])];
    println!(
        "{}",
        (1..11)
            .map(|t| p[0].0[t] * p[1].1[t - 1])
            .sum::<usize>()
            .max((1..11).map(|t| p[1].0[t - 1] * p[0].1[t - 1]).sum())
    );
}

fn scores(pos: usize) -> ([usize; 11], [usize; 11]) {
    let mut tab = [[[0; 22]; 11]; 11];
    tab[0][pos][0] = 1;
    (1..11).for_each(|t| {
        (1..11).for_each(|p| {
            (0..21).for_each(|s| {
                PROB.iter().enumerate().for_each(|(i, w)| {
                    let q = ((p + i + 2) % 10) + 1;
                    let v = (q + s).min(21);
                    tab[t][q][v] += w * tab[t - 1][p][s];
                });
            });
        });
    });

    let mut out = ([0; 11], [0; 11]);
    tab.iter().enumerate().for_each(|(t, tab)| {
        tab[1..].iter().for_each(|tab| {
            tab[..21].iter().for_each(|tab| out.1[t] += tab);
            out.0[t] += tab[21];
        });
    });
    out
}
