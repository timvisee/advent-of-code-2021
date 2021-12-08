// seg:
//    0000
//   1    2
//   1    2
//    3333
//   -    4
//   -    4
//    ----

pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let (ex, digits) = line.split_once('|').unwrap();
                let mut ex = ex.split_ascii_whitespace().collect::<Vec<_>>();
                ex.sort_unstable_by_key(|d| d.len());

                let mut seg = [0u8; 5];
                let seg_count = ex.iter().fold([0u8; 7], |mut map, digit| {
                    digit.bytes().for_each(|b| map[(b - b'a') as usize] += 1);
                    map
                });

                seg[0] = ex[1].bytes().find(|&b| !ex[0].contains(b as char)).unwrap();
                seg[1] = seg_count.iter().position(|&c| c == 6).unwrap() as u8 + b'a';
                seg[4] = seg_count.iter().position(|&c| c == 9).unwrap() as u8 + b'a';
                seg[2] = ex[1].bytes().find(|b| !seg.contains(b)).unwrap();
                seg[3] = ex[2].bytes().find(|c| !seg.contains(c)).unwrap();

                digits
                    .split_ascii_whitespace()
                    .map(|d| match d.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        7 => 8,
                        6 if !d.contains(seg[3] as char) => 0,
                        6 if !d.contains(seg[2] as char) => 6,
                        6 => 9,
                        _ if !d.contains(seg[2] as char) => 5,
                        _ if d.contains(seg[4] as char) => 3,
                        _ => 2,
                    })
                    .enumerate()
                    .fold(0, |sum, (i, n)| sum + n * 10_u32.pow(3 - i as u32))
            })
            .sum::<u32>()
    );
}
