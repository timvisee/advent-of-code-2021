#![feature(drain_filter)]

pub fn main() {
    let (nums, boards) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut boards: Vec<Vec<Option<u8>>> = boards
        .split("\n\n")
        .map(|b| b.split_whitespace().map(|n| n.parse().ok()).collect())
        .collect();

    let (board, num) = nums
        .split(',')
        .map(|n| n.parse().unwrap())
        .filter_map(|n| {
            boards
                .drain_filter(|board| {
                    board.iter_mut().for_each(|c| *c = c.filter(|v| *v != n));
                    (0..5).any(|r| board[r * 5..][..5].iter().all(Option::is_none))
                        || (0..5).any(|c| board.iter().skip(c).step_by(5).all(Option::is_none))
                })
                .map(|b| (b, n))
                .next()
        })
        .last()
        .unwrap();

    println!(
        "{}",
        board.iter().map(|n| n.unwrap_or(0) as u32).sum::<u32>() * num as u32,
    );
}
