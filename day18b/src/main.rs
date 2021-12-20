/// Snailfish sequence: `[(depth, n)]`
type Num = Vec<(u8, u8)>;

pub fn main() {
    let nums = include_str!("../input.txt")
        .trim()
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|b| {
            b.iter()
                .fold((0, Vec::with_capacity(b.len() / 2)), |(mut d, mut n), b| {
                    match b {
                        b'[' => d += 1,
                        b']' => d -= 1,
                        b'0'..=b'9' => n.push((d, b - b'0')),
                        _ => {}
                    }
                    (d, n)
                })
                .1
        })
        .collect::<Vec<_>>();

    let mut max = 0;
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            let (a, b) = (&nums[i], &nums[j]);
            let (mut aa, mut bb) = (a.clone(), b.clone());
            add(&mut aa, b);
            reduce(&mut aa, 0);
            max = mag(&mut 0, 1, &aa).max(max);
            add(&mut bb, a);
            reduce(&mut bb, 0);
            max = mag(&mut 0, 1, &bb).max(max);
        }
    }

    println!("{}", max);
}

fn add(nums: &mut Num, other: &Num) {
    nums.extend_from_slice(other);
    nums.iter_mut().for_each(|(d, _)| *d += 1);
}

fn reduce(nums: &mut Num, i: usize) {
    for i in i..nums.len() - 1 {
        if nums[i].0 == 5 {
            let (l, r) = (nums[i].1, nums[i + 1].1);
            nums[i] = (4, 0);
            nums.remove(i + 1);
            let _ = nums.get_mut(i.overflowing_sub(1).0).map(|n| n.1 += l);
            let _ = nums.get_mut(i + 1).map(|n| n.1 += r);
            return reduce(nums, i);
        }
    }
    for i in 0..nums.len() {
        let (d, n) = nums[i];
        if n >= 10 {
            nums[i] = (d + 1, n / 2);
            nums.insert(i + 1, (d + 1, (n + 1) / 2));
            return reduce(nums, i);
        }
    }
}

#[inline]
fn mag(i: &mut usize, depth: u8, sf: &Num) -> u16 {
    3 * if sf[*i].0 == depth {
        *i += 1;
        sf[*i - 1].1 as u16
    } else {
        mag(i, depth + 1, sf)
    } + 2 * if sf[*i].0 == depth {
        *i += 1;
        sf[*i - 1].1 as u16
    } else {
        mag(i, depth + 1, sf)
    }
}
