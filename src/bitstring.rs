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

    pub const fn convert(&self) -> [u8; 2956] {
        let mut buffer = [0; 2956];
        let mut i = 0;

        while i < C {
            let index = i / 8;

            buffer[index] = (self.data[i] as u8) << 7
                | (self.data[i + 1] as u8) << 6
                | (self.data[i + 2] as u8) << 5
                | (self.data[i + 3] as u8) << 4
                | (self.data[i + 4] as u8) << 3
                | (self.data[i + 5] as u8) << 2
                | (self.data[i + 6] as u8) << 1
                | (self.data[i + 7] as u8);

            i += 8;
        }

        buffer
    }

    pub fn to_vec(&self) -> Vec<bool> {
        self.data.to_vec()
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
