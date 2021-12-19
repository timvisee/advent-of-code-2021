#![feature(array_chunks)]

mod bits;

use bits::Bits;

pub fn main() {
    let bytes = include_str!("../input.txt")
        .trim()
        .as_bytes()
        .array_chunks()
        .map(|&[a, b]| {
            (((a as char).to_digit(16).unwrap() as u8) << 4)
                | (b as char).to_digit(16).unwrap() as u8
        })
        .collect::<Vec<_>>();
    println!("{}", packet(&mut Bits::new(&bytes))[0]);
}

fn packet(bits: &mut Bits) -> Vec<usize> {
    bits.skip(3);
    let typ = bits.take(3);
    if typ == 4 {
        return vec![bits.take_literal()];
    }

    let mut nums = vec![];
    if bits.take(1) == 0 {
        let len = bits.take(15);
        let mut payload = bits.split(len);
        while !payload.is_empty() {
            nums.extend_from_slice(&packet(&mut payload));
        }
    } else {
        (0..bits.take(11)).for_each(|_| nums.extend_from_slice(&packet(bits)));
    }

    match typ {
        0 => vec![nums.iter().sum()],
        1 => vec![nums.iter().product()],
        2 => vec![*nums.iter().min().unwrap()],
        3 => vec![*nums.iter().max().unwrap()],
        5 => vec![(nums[0] > nums[1]) as usize],
        6 => vec![(nums[0] < nums[1]) as usize],
        7 => vec![(nums[0] == nums[1]) as usize],
        _ => unreachable!(),
    }
}
