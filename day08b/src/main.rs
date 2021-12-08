// seg:
//    0000
//   1    2
//   1    2
//    3333
//   4    5
//   4    5
//    6666

pub fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let (hints, digits) = line.split_once(" | ").unwrap();
                let mut hints = hints
                    .split_ascii_whitespace()
                    .map(|d| d.bytes().map(|b| b - b'a').collect())
                    .collect::<Vec<Vec<u8>>>();
                hints.select_nth_unstable_by_key(1, Vec::len);

                let mut seg = [255u8; 7];
                let seg_count = hints.iter().fold([0u8; 7], |mut map, digit| {
                    digit.iter().for_each(|&b| map[b as usize] += 1);
                    map
                });

                seg[0] = *hints[1].iter().find(|b| !hints[0].contains(b)).unwrap();
                seg[1] = seg_count.iter().position(|&c| c == 6).unwrap() as u8;
                seg[5] = seg_count.iter().position(|&c| c == 9).unwrap() as u8;
                seg[2] = *hints[1].iter().find(|b| !seg.contains(b)).unwrap();
                seg[3] = *hints[2].iter().find(|c| !seg.contains(c)).unwrap();

                digits
                    .split_ascii_whitespace()
                    .map(|d| d.bytes().map(|b| b - b'a').collect::<Vec<_>>())
                    .map(|d| match d.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        7 => 8,
                        6 if !d.contains(&seg[3]) => 0,
                        6 if !d.contains(&seg[2]) => 6,
                        6 => 9,
                        _ if !d.contains(&seg[2]) => 5,
                        _ if d.contains(&seg[5]) => 3,
                        _ => 2,
                    })
                    .enumerate()
                    .fold(0, |sum, (i, n)| sum + n * 10_u32.pow(3 - i as u32))
            })
            .sum::<u32>()
    );
}
