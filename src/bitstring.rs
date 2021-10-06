pub struct BitString<const C: usize> {
    pub len: usize,
    pub data: [u8; C],
}

impl<const C: usize> BitString<C> {
    pub const fn new() -> Self {
        BitString {
            len: 0,
            data: [0; C],
        }
    }

    pub const fn from(from_: [u8; C], len_: usize) -> Self {
        BitString {
            len: len_,
            data: from_,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn get_data(&self) -> [u8; C] {
        self.data
    }

    #[cfg(test)]
    pub fn as_string(&self) -> String {
        let mut res = String::new();

        for i in 0..(C / 8) {
            let nb = self.data[i];
            for j in 0..8 {
                if i * 8 + j >= self.len {
                    return res;
                }

                let j = 7 - j;
                let c = if nb & (1 << j) != 0 { '1' } else { '0' };
                res.push(c);
            }
        }

        return res;
    }
}

#[inline(always)]
pub const fn push<const C: usize>(mut bs: BitString<C>, bit: bool) -> BitString<C> {
    bs.data[bs.len / 8] |= (bit as u8) << (7 - bs.len % 8);
    bs.len += 1;
    bs
}

#[inline(always)]
pub const fn push_u8<const C: usize>(mut bs: BitString<C>, bits: u8) -> BitString<C> {
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

#[inline(always)]
pub const fn push_u8_slice<const C: usize>(mut bs: BitString<C>, slice: &[u8]) -> BitString<C> {
    let mut i = 0;
    while i < slice.len() {
        let bits = slice[i];

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
        i += 1;
    }

    return bs;
}

// #[inline(always)]
// pub const fn push_bits<const C: usize>(
//     mut bs: BitString<C>,
//     bits: usize,
//     len: usize,
// ) -> BitString<C> {
//     let mut shift = len;

//     while shift > 0 {
//         shift -= 1;

//         bs = push(bs, (bits >> shift) % 2 != 0);
//     }

//     bs
// }

pub const KEEP_LAST: [usize; 9] = [0, 1, 3, 7, 15, 31, 63, 127, 255];

#[inline(always)]
pub const fn push_bits<const C: usize>(
    mut bs: BitString<C>,
    bits: usize,
    len: usize,
) -> BitString<C> {
    let bits = bits & ((1 << len) - 1);

    let rem_space = (8 - bs.len % 8) % 8;
    let first = bs.len / 8;

    if rem_space > len {
        bs.data[first] |= (bits >> (rem_space - len)) as u8;
        bs.len += len;
        return bs;
    }

    // println!("GOO");

    if rem_space != 0 {
        // println!("rem_space: {}", rem_space);
        bs.data[first] |= ((bits >> (len - rem_space)) & KEEP_LAST[rem_space]) as u8;
        bs.len += rem_space;
    }

    let mut i = len - rem_space;
    while i >= 8 {
        // println!("i: {}", i);

        bs.data[bs.len / 8] = (bits >> (i - 8)) as u8;
        bs.len += 8;
        i -= 8;
    }

    if i != 0 {
        // println!("remaining: {}", i);
        bs.data[bs.len / 8] = ((bits & KEEP_LAST[i]) as u8) << (8 - i);
        bs.len += i;
    }

    bs
}
