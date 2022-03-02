//! Struct containing an u8-array of C size to store bitwise boolean values

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

/// Struct containing an u8-array of C size to store bitwise boolean values
pub struct BitString<const C: usize> {
    pub len: usize,
    pub data: [u8; C],
}

impl<const C: usize> BitString<C> {
    /// Instantiates a new BitString
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

    /// Instantiates a new BitString from an already created array
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
        let mut res = String::with_capacity(self.len);

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

        res
    }

    #[inline(always)]
    #[allow(dead_code)]
    /// Pushes height values in the BitString
    ///
    /// # Example
    /// ```txt
    /// let mut bs = BitString::<50>::new();
    /// bs.push_u8(0b1001_1001);
    /// assert_eq!(bs.as_string(), "10011001");
    /// ```
    pub fn push_u8(&mut self, bits: u8) {
        let right = self.len % 8;
        let first_idx = self.len / 8;

        if right == 0 {
            self.data[first_idx] = bits;
        } else {
            let left = 8 - right;
            self.data[first_idx] |= (bits & (KEEP_LAST[left] << right) as u8) >> right;
            self.data[first_idx + 1] |= (bits & KEEP_LAST[right] as u8) << left;
        }

        self.len += 8;
    }

    #[inline(always)]
    /// Pushes the u8 array in the BitString
    ///
    /// # Example
    /// ```txt
    /// let mut bs = BitString::<50>::new();
    /// bs.push_u8_slice(&[0b1001_1001, 0b0110_0110]);
    /// assert_eq!(bs.as_string(), "1001100101100110");
    /// ```
    pub fn push_u8_slice(&mut self, slice: &[u8]) {
        for &u in slice {
            self.push_u8(u);
        }
    }

    // pub const fn tmp(&self, nb_bits: usize) {
    //     let rem_space = (8 - self.len % 8) % 8;
    //     let rem_space = rem_space as u8;
    //     let mut t= [(0u8, 0u8); 8];
    //     t[0] = std::cmp::min(rem_space, nb_bits as u8);
    // }

    #[inline(always)]
    /// Pushes `len` values to the BitString
    ///
    /// # Example
    /// ```txt
    /// let mut bs = BitString::<50>::new();
    /// bs.push_bits(0b1000_1000_0110_1110, 16);
    /// assert_eq!(bs.as_string(), "1000100001101110");
    /// ```
    pub fn push_bits(&mut self, bits: usize, len: usize) {
        // Caps to max usize bits
        let bits = bits & KEEP_LAST[len];

        let rem_space = (8 - self.len % 8) % 8;
        let first = self.len / 8;

        if rem_space > len {
            self.data[first] |= (bits << (rem_space - len)) as u8;
            self.len += len;
            return;
        }

        if rem_space != 0 {
            self.data[first] |= ((bits >> (len - rem_space)) & KEEP_LAST[rem_space]) as u8;
            self.len += rem_space;
        }

        for i in (8..=len - rem_space).rev().step_by(8) {
            self.push_u8((bits >> (i - 8)) as u8);
        }

        let remaining = (len - rem_space) % 8;
        if remaining == 0 {
            return;
        }

        self.data[self.len / 8] += ((bits & KEEP_LAST[remaining]) as u8) << (8 - remaining);
        self.len += remaining;
    }

    #[inline(always)]
    /// Fills the bitstring to it's length `data_bits`  with `236` and `17`
    /// which is very useful for QRCodes padding
    ///
    /// # Example
    /// ```txt
    /// let mut bs = BitString::<50>::new();
    /// bs.fill(bs, 4 * 8);
    /// assert_eq!(bs.as_string(), "11101100000100011110110000010001");
    /// ```
    pub fn fill(&mut self, data_bits: usize) {
        const PAD_BYTES: [u8; 2] = [0b11101100, 0b00010001]; //[236, 17]

        for (i, _) in (self.len..data_bits).step_by(8).enumerate() {
            let bits = PAD_BYTES[i % 2];
            self.push_u8(bits);
        }
    }
}
