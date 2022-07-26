//! Struct containing an u8-array of C size to store bitwise boolean values

#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::Version;

#[rustfmt::skip]
#[cfg(not(target_arch = "wasm32"))]
/// Values to keep last X bits of a u8
/// `KEEP_LAST[i]` equates `(1 << i) - 1`
/// 
/// # Example
/// ```rust
/// # pub const KEEP_LAST: [usize; 65] = [
/// #     0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383,
/// #     32767, 65535, 131071, 262143, 524287, 1048575, 2097151, 4194303, 8388607,
/// #     16777215, 33554431, 67108863, 134217727, 268435455, 536870911,  1073741823,
/// #     2147483647, 4294967295, 8589934591, 17179869183, 34359738367, 68719476735,
/// #     137438953471, 274877906943, 549755813887, 1099511627775, 2199023255551,
/// #     4398046511103, 8796093022207, 17592186044415, 35184372088831,
/// #     70368744177663, 140737488355327, 281474976710655, 562949953421311,
/// #     1125899906842623, 2251799813685247, 4503599627370495, 9007199254740991,
/// #     18014398509481983, 36028797018963967, 72057594037927935,
/// #     144115188075855871, 288230376151711743, 576460752303423487,
/// #     1152921504606846975, 2305843009213693951, 4611686018427387903,
/// #     9223372036854775807, 18446744073709551615,
/// # ];
/// let mut b = 0b1010_1010;
/// assert_eq!(b & KEEP_LAST[3], 0b010)
/// ```
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

#[rustfmt::skip]
#[cfg(target_arch = "wasm32")]
/// Values to keep last X bits of a u8
/// `KEEP_LAST[i]` equates `(1 << i) - 1`
pub const KEEP_LAST: [usize; 33] = [
    0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383,
    32767, 65535, 131071, 262143, 524287, 1048575, 2097151, 4194303, 8388607,
    16777215, 33554431, 67108863, 134217727, 268435455, 536870911,  1073741823,
    2147483647, 4294967295,
];

/// `CompactQR` is a struct that contains a `Vec<u8>` to store boolean values as bits.
pub struct CompactQR {
    pub len: usize,
    pub data: Vec<u8>,
}

impl CompactQR {
    #[allow(dead_code)]
    /// Instantiates a new CompactQR, should not be used, reduces performance.
    pub const fn new() -> Self {
        CompactQR {
            len: 0,
            data: Vec::new(),
        }
    }

    pub fn from_version(version: Version) -> Self {
        let len = version.max_bytes();
        let data = vec![0; len * 8];

        CompactQR { len: 0, data }
    }

    /// Instantiates a new CompactQR, with a given length, expects the length to be a multiple of 8.
    pub fn with_len(data_length: usize) -> Self {
        let length = data_length / 8 + (data_length % 8 != 0) as usize;
        CompactQR {
            len: 0,
            data: vec![0; length],
        }
    }

    /// Increase the length of data to specified length.
    pub fn increase_len(&mut self, data_length: usize) {
        if data_length / 8 >= self.data.len() {
            self.data.resize(data_length / 8, 0);
        }
    }

    /// Instantiates a new CompactQR from an already created array
    pub fn from_array(data: &[u8], len: usize) -> Self {
        CompactQR {
            len,
            data: data.to_vec(),
        }
    }

    /// Returns `len`, length is the current number of bits / boolean values stored in the array.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `data`, the array of bits.
    pub const fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    #[allow(dead_code)]
    /// Returns a string visualization of the CompactQR. \
    /// `CompactQR { len: 4, data: [0b1111_1010] }.to_string()` => `"1010"`
    pub fn to_string(&self) -> String {
        let mut res = String::with_capacity(self.len);

        for i in 0..(self.data.capacity() / 8) {
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
    /// Pushes eight values in the CompactQR, if the array is not big enough, it will be resized.
    pub fn push_u8(&mut self, bits: u8) {
        self.increase_len(self.len + 8);

        let right = self.len % 8;
        let first_idx = self.len / 8;

        if right == 0 {
            self.data[first_idx] = bits;
        } else {
            let left = 8 - right;
            self.data[first_idx] |= (bits >> right) & (KEEP_LAST[left]) as u8;
            self.data[first_idx + 1] |= (bits & KEEP_LAST[right] as u8) << left;
        }

        self.len += 8;
    }

    #[inline(always)]
    /// Pushes the u8 array in the CompactQR, using the `push_u8` function. \
    /// If the array is not big enough, it will be resized.
    pub fn push_u8_slice(&mut self, slice: &[u8]) {
        self.increase_len(self.len + 8 * slice.len());

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
    /// Pushes `len` values to the CompactQR. \
    /// If the array is not big enough, it will be resized.
    pub fn push_bits(&mut self, bits: usize, len: usize) {
        self.increase_len(self.len + len);

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
    /// Fills the CompactQR's remaining space with `[236, 17]`.
    /// Expects the CompactQR `len` to be a multiple of 8.
    pub fn fill(&mut self) {
        #[cfg(debug_assertions)]
        assert_eq!(self.len % 8, 0);

        const PAD_BYTES: [u8; 2] = [0b11101100, 0b00010001]; //[236, 17]

        for (i, _) in (self.len..self.data.len()).step_by(8).enumerate() {
            let bits = PAD_BYTES[i % 2];
            self.push_u8(bits);
        }
    }
}
