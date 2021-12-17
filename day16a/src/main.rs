pub fn main() {
    let bits = include_str!("../input.txt")
        .trim()
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap();
            (0..4).map(move |i| (1 & n >> (3 - i)) as u8)
        })
        .collect::<Vec<u8>>();
    println!("{}", packet(&bits).0);
}

fn packet(mut bits: &[u8]) -> (u16, &[u8]) {
    let mut ver = num(&bits[0..3]);
    let typ = num(&bits[3..6]);
    bits = &bits[6..];
    if typ == 4 {
        (0..)
            .take_while(|_| {
                let cont = bits[0] == 1;
                bits = &bits[5..];
                cont
            })
            .count();
        return (ver, bits);
    }

    let split = bits.split_at(1);
    bits = split.1;
    if split.0[0] == 0 {
        let len = num(&bits[0..15]) as usize;
        bits = &bits[15..];
        let (mut payload, extra) = bits.split_at(len);
        bits = extra;
        while !payload.is_empty() {
            let packet = packet(payload);
            payload = packet.1;
            ver += packet.0;
        }
    } else {
        let split = bits.split_at(11);
        bits = split.1;
        ver += (0..num(split.0))
            .map(|_| {
                let packet = packet(bits);
                bits = packet.1;
                packet.0
            })
            .sum::<u16>();
    }
    (ver, bits)
}

fn num(b: &[u8]) -> u16 {
    b.iter()
        .enumerate()
        .map(|(i, &x)| (x as u16) << (b.len() - i - 1))
        .sum()
}
