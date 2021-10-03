pub const MAX: BitString<23648> = BitString::<23648>::new();

pub struct BitString<const C: usize> {
    data: [bool; C],
    len: usize,
}

impl<const C: usize> BitString<C> {
    pub const fn new() -> Self {
        BitString {
            data: [false; C],
            len: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn capacity(&self) -> usize {
        C
    }

    pub const fn get(&self, index: usize) -> bool {
        self.data[index]
    }
}

pub const fn push<const C: usize>(mut bs: BitString<C>, bit: bool) -> BitString<C> {
    bs.data[bs.len] = bit;
    bs.len += 1;
    bs
}

pub const fn push_u8<const C: usize>(mut bs: BitString<C>, bits: u8) -> BitString<C> {
    let mut shift = u8::BITS;

    while shift > 0 {
        shift -= 1;

        bs = push(bs, (bits >> shift) % 2 != 0);
    }

    bs
}

pub const fn push_bits<const C: usize>(
    mut bs: BitString<C>,
    bits: usize,
    len: usize,
) -> BitString<C> {
    let mut shift = len;

    while shift > 0 {
        shift -= 1;

        bs = push(bs, (bits >> shift) % 2 != 0);
    }

    bs
}

pub const fn push_slice<const C: usize>(mut bs: BitString<C>, slice: &[bool]) -> BitString<C> {
    let mut i = 0;

    while i < slice.len() {
        bs = push(bs, slice[i]);

        i += 1;
    }

    bs
}
