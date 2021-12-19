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
    println!("{}", packet(&mut Bits::new(&bytes)));
}

fn packet(bits: &mut Bits) -> usize {
    let mut ver = bits.take(3);
    if bits.take(3) == 4 {
        bits.take_literal();
        return ver;
    }

    if bits.take(1) == 0 {
        let len = bits.take(15);
        let mut payload = bits.split(len);
        while !payload.is_empty() {
            ver += packet(&mut payload);
        }
    } else {
        (0..bits.take(11)).for_each(|_| ver += packet(bits));
    }
    ver
}
