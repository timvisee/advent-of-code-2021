pub struct Bits<'a> {
    b: &'a [u8],
    pos: usize,
    len: usize,
}

impl<'a> Bits<'a> {
    pub fn new(b: &'a [u8]) -> Self {
        Self {
            pos: 0,
            len: b.len() * 8,
            b,
        }
    }

    pub fn take(&mut self, mut count: usize) -> usize {
        let mut out = 0;
        while count > 0 {
            let left = 8 - (self.pos % 8);
            let bits = left.min(count);
            out = out << bits
                | ((self.b[self.pos / 8] & u8::MAX >> (self.pos % 8)) >> (left - bits)) as usize;
            count -= bits;
            self.skip(bits);
        }
        out
    }

    pub fn take_literal(&mut self) -> usize {
        let mut out = 0;
        loop {
            let num = self.take(5);
            out = out << 4 | num & 0b1111;
            if num & 0b10000 == 0 {
                return out;
            }
        }
    }

    pub fn split(&mut self, count: usize) -> Bits {
        debug_assert!(count <= self.len - self.pos);
        self.pos += count;
        Bits {
            b: self.b,
            pos: self.pos - count,
            len: self.pos,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.len
    }

    pub fn skip(&mut self, count: usize) {
        debug_assert!(self.pos <= self.len);
        self.pos += count;
    }
}
