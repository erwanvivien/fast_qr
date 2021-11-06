//! Struct containing an u8-array of C size to store bitwisely boolean values

#![deny(unsafe_code)]
#![warn(missing_docs)]

#[cfg(test)]
mod test;

#[rustfmt::skip]
/// Values to keep last X bits of a u8
/// `KEEP_LAST[i]` equates `(1 << i) - 1`
pub const KEEP_LAST: [usize; 65] = [
    0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383,
    32767, 65535, 131071, 262143, 524287, 1048575, 2097151, 4194303, 8388607,
    16777215, 33554431, 67108863, 134217727, 268435455, 536870911,  1073741823,
    2147483647, 4294967295, 8589934591, 17179869183, 34359738367, 68719476735,
    137438953471, 274877906943, 549755813887, 1099511627775, 2199023255551,
    4398046511103, 8796093022207, 17592186044415, 35184372088831,
    70368744177663, 140737488355327, 281474976710655, 562949953421311,
    1125899906842623, 2251799813685247, 4503599627370495, 9007199254740991,
    18014398509481983, 36028797018963967, 72057594037927935,
    144115188075855871, 288230376151711743, 576460752303423487,
    1152921504606846975, 2305843009213693951, 4611686018427387903,
    9223372036854775807, 18446744073709551615,
];

/// Struct containing an u8-array of C size to store bitwisely boolean values
pub struct BitString<const C: usize> {
    pub len: usize,
    pub data: [u8; C],
}

impl<const C: usize> BitString<C> {
    /// Instanciates a new BitString
    ///
    /// # Example
    /// ```txt
    /// let bs = BitString::<50>::new();
    /// assert_eq!(bs.as_string(), "");
    /// ```
    pub const fn new() -> Self {
        BitString {
            len: 0,
            data: [0; C],
        }
    }

    /// Instanciates a new BitString from an already created array
    ///
    /// # Example
    /// ```txt
    /// let mut array = [0; 50];
    /// array[0] = 0b11010000;
    /// let bs = BitString::<50>::from(array, 4);
    /// assert_eq!(bs.as_string(), "1101");
    /// ```
    pub const fn from(data: [u8; C], len: usize) -> Self {
        BitString { len, data }
    }

    /// Returns `len`
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `data`
    pub const fn get_data(&self) -> [u8; C] {
        self.data
    }

    #[allow(dead_code)]
    /// Returns a string visualization of the BitString
    ///
    /// # Example
    /// ```txt
    /// let mut array = [0; 50];
    /// array[0] = 0b11010000;
    /// let bs = BitString::<50>::from(array, 4);
    /// assert_eq!(bs.as_string(), "1101");
    /// ```
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

    #[inline(always)]
    #[allow(dead_code)]
    /// Pushes height values in the BitString
    ///
    /// # Example
    /// ```txt
    /// let mut bs = BitString::<50>::new();
    /// bs = BitString::push_u8(bs, 0b1001_1001);
    /// assert_eq!(bs.as_string(), "10011001");
    /// ```
    pub const fn push_u8(mut bs: BitString<C>, bits: u8) -> BitString<C> {
        let right = bs.len % 8;
        let first_idx = bs.len / 8;

        if right == 0 {
            bs.data[first_idx] = bits;
        } else {
            let left = 8 - right;
            bs.data[first_idx] |= (bits & (KEEP_LAST[left] << right) as u8) >> right;
            bs.data[first_idx + 1] |= (bits & KEEP_LAST[right] as u8) << left;
        }

        bs.len += 8;
        bs
    }

    #[inline(always)]
    /// Pushes the u8 array in the BitString
    ///
    /// # Example
    /// ```txt
    /// let mut bs = BitString::<50>::new();
    /// bs = BitString::push_u8_slice(bs, &[0b1001_1001, 0b0110_0110]);
    /// assert_eq!(bs.as_string(), "1001100101100110");
    /// ```
    pub const fn push_u8_slice(mut bs: BitString<C>, slice: &[u8]) -> BitString<C> {
        let mut i = 0;
        while i < slice.len() {
            let bits = slice[i];

            let right = bs.len % 8;
            let first_idx = bs.len / 8;

            if right == 0 {
                bs.data[first_idx] = bits;
            } else {
                let left = 8 - right;
                bs.data[first_idx] |= (bits & (KEEP_LAST[left] << right) as u8) >> right;
                bs.data[first_idx + 1] |= (bits & KEEP_LAST[right] as u8) << left;
            }

            bs.len += 8;
            i += 1;
        }

        return bs;
    }

    #[inline(always)]
    /// Pushes `len` values to the BitString
    ///
    /// # Example
    /// ```txt
    /// let mut bs = BitString::<50>::new();
    /// bs = BitString::push_bits(bs, 0b1000_1000_0110_1110, 16);
    /// assert_eq!(bs.as_string(), "1000100001101110");
    /// ```
    pub const fn push_bits(mut bs: BitString<C>, bits: usize, len: usize) -> BitString<C> {
        // Caps to max usize bits
        let len = len & KEEP_LAST[usize::BITS as usize];
        let bits = bits & KEEP_LAST[len];

        let rem_space = (8 - bs.len % 8) % 8;
        let first = bs.len / 8;

        if rem_space > len {
            bs.data[first] |= (bits << (rem_space - len)) as u8;
            bs.len += len;
            return bs;
        }

        if rem_space != 0 {
            bs.data[first] |= ((bits >> (len - rem_space)) & KEEP_LAST[rem_space]) as u8;
            bs.len += rem_space;
        }

        let mut i = len - rem_space;
        while i >= 8 {
            bs.data[bs.len / 8] = (bits >> (i - 8)) as u8;
            bs.len += 8;
            i -= 8;
        }

        if i != 0 {
            bs.data[bs.len / 8] = ((bits & KEEP_LAST[i]) as u8) << (8 - i);
            bs.len += i;
        }

        bs
    }

    #[inline(always)]
    /// Fills the bitstring to it's length `data_bits`  with `236` and `17`
    /// which is very usefull for QRCodes padding
    ///
    /// # Example
    /// ```txt
    /// let mut bs = BitString::<50>::new();
    /// bs = BitString::fill(bs, 4*8);
    /// assert_eq!(bs.as_string(), "11101100000100011110110000010001");
    /// ```
    pub const fn fill(mut bs: BitString<C>, data_bits: usize) -> BitString<C> {
        const PAD_BYTES: [u8; 2] = [0b11101100, 0b00010001]; //[236, 17]

        let mut byte = false;

        while bs.len() < data_bits {
            let bits = PAD_BYTES[byte as usize];

            let first_idx = bs.len / 8;
            bs.data[first_idx] = bits;

            bs.len += 8;
            byte = !byte;
        }

        bs
    }
}
