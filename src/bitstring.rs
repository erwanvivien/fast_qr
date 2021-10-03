pub struct BitString {
    len: usize,
    data: [u8; 2956],
}

impl BitString {
    pub const fn new() -> Self {
        BitString {
            len: 0,
            data: [0; 2956],
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
    let right = bs.len % 8;
    let first_idx = bs.len / 8;

    if right == 0 {
        bs.data[first_idx] = bits;
    } else {
        let left = 8 - right;
        bs.data[first_idx] |= bits >> right;
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
