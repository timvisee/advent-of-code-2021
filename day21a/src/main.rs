pub fn main() {
    let (mut pos, mut score): ([_; 2], _) = (
        include_str!("../input.txt")
            .lines()
            .map(|l| l[l.len() - 2..l.len()].trim().parse::<u16>().unwrap() - 1)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        [0; 2],
    );

    let mut die = 0..;
    let mut throw = || (die.by_ref().next().unwrap() * 3 + 2) * 3;

    for turn in 0.. {
        for p in 0..2 {
            pos[p] = (pos[p] + throw()) % 10;
            score[p] += pos[p] + 1;
            if score[p] >= 1000 {
                println!(
                    "{}",
                    score[(p + 1) % 2] as usize * (turn * 6 + ((p + 1) * 3))
                );
                return;
            }
        }
    }
}
