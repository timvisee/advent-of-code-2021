pub fn main() {
    let bits = include_str!("../input.txt")
        .trim()
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap();
            (0..4).map(move |i| (1 & n >> (3 - i)) as u8)
        })
        .collect::<Vec<u8>>();
    println!("{}", packet(&bits).0[0]);
}

fn packet(mut bits: &[u8]) -> (Vec<usize>, &[u8]) {
    let typ = num(&bits[3..6]);
    bits = &bits[6..];
    if typ == 4 {
        let mut num_bits = vec![];
        (0..)
            .take_while(|_| {
                let cont = bits[0] == 1;
                num_bits.extend(&bits[1..5]);
                bits = &bits[5..];
                cont
            })
            .count();
        return (vec![num(&num_bits)], bits);
    }

    let mut nums = vec![];
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
            nums.extend_from_slice(&packet.0);
        }
    } else {
        let split = bits.split_at(11);
        bits = split.1;
        (0..num(split.0)).for_each(|_| {
            let packet = packet(bits);
            bits = packet.1;
            nums.extend_from_slice(&packet.0);
        });
    }

    match typ {
        0 => (vec![nums.iter().sum()], bits),
        1 => (vec![nums.iter().product()], bits),
        2 => (vec![*nums.iter().min().unwrap()], bits),
        3 => (vec![*nums.iter().max().unwrap()], bits),
        5 => (vec![(nums[0] > nums[1]) as usize], bits),
        6 => (vec![(nums[0] < nums[1]) as usize], bits),
        7 => (vec![(nums[0] == nums[1]) as usize], bits),
        _ => unreachable!(),
    }
}

fn num(b: &[u8]) -> usize {
    b.iter()
        .enumerate()
        .map(|(i, &x)| (x as usize) << (b.len() - i - 1))
        .sum()
}
