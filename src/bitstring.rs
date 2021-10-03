pub struct BitString {
    data: [u8; 2956],
    len: usize,
}

impl BitString {
    pub const fn new() -> Self {
        BitString {
            data: [0; 2956],
            len: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn get_data(&self) -> [u8; 2956] {
        self.data
    }
}

pub const fn push(mut bs: BitString, bit: bool) -> BitString {
    bs.data[bs.len / 8] |= (bit as u8) << (7 - bs.len % 8);
    bs.len += 1;
    bs
}

pub const fn push_u8(mut bs: BitString, bits: u8) -> BitString {
    let left = 8 - bs.len % 8;
    let right = 8 - left;

    let first_idx = bs.len / 8;
    bs.data[first_idx] |= bits >> right;
    if right != 0 {
        bs.data[first_idx + 1] |= (bits & ((1 << left) - 1)) << right;
    }

    bs.len += 8;
    bs
}

pub const fn push_bits(mut bs: BitString, bits: usize, len: usize) -> BitString {
    let mut shift = len;

    while shift > 0 {
        shift -= 1;

        bs = push(bs, (bits >> shift) % 2 != 0);
    }

    bs
}
