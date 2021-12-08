pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let (ex, digits) = line.split_once('|').unwrap();
                let ex = ex.split_ascii_whitespace().collect::<Vec<_>>();
                let one = ex.iter().find(|d| d.len() == 2).unwrap();
                let four = ex.iter().find(|d| d.len() == 4).unwrap();
                digits
                    .split_ascii_whitespace()
                    .map(|d| match d.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        7 => 8,
                        len => match (
                            len,
                            d.bytes().filter(|&b| one.contains(b as char)).count(),
                            d.bytes().filter(|&b| four.contains(b as char)).count(),
                        ) {
                            (5, 1, 3) => 5,
                            (5, 2, 3) => 3,
                            (5, _, 2) => 2,
                            (6, 1, _) => 6,
                            (6, _, 3) => 0,
                            (6, _, 4) => 9,
                            _ => unreachable!(),
                        },
                    })
                    .enumerate()
                    .fold(0, |sum, (i, n)| sum + n * 10_u32.pow(3 - i as u32))
            })
            .sum::<u32>()
    );
}
