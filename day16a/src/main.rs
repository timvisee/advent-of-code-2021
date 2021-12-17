use bytes::BytesMut;

pub fn main() {
    let mut bits =
        include_str!("../input.txt")
            .trim()
            .chars()
            .fold(BytesMut::new(), |mut bits, c| {
                let b = u8::from_str_radix(&c.to_string(), 16).unwrap();
                bits.extend_from_slice(&[
                    (b & (1 << 3)) >> 3,
                    (b & (1 << 2)) >> 2,
                    (b & (1 << 1)) >> 1,
                    b & 1,
                ]);
                bits
            });
    println!("{}", packet(&mut bits));
}

fn packet(bits: &mut BytesMut) -> u16 {
    let mut ver = num(&bits.split_to(3));
    if num(&bits.split_to(3)) == 4 {
        (0..).take_while(|_| bits.split_to(5)[0] == 1).count();
    } else {
        if bits.split_to(1)[0] == 0 {
            let len = num(&bits.split_to(15)) as usize;
            let mut bits = bits.split_to(len);
            while !bits.is_empty() {
                ver += &packet(&mut bits);
            }
        } else {
            ver += (0..num(&bits.split_to(11)))
                .map(|_| packet(bits))
                .sum::<u16>();
        }
    }
    ver
}

fn num(b: &[u8]) -> u16 {
    b.iter()
        .enumerate()
        .map(|(i, &x)| (x as u16) << (b.len() - i - 1))
        .sum()
}
