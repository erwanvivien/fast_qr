//! Struct containing an u8-array of C size to store bitwisely boolean values

#![deny(unsafe_code)]
#![warn(missing_docs)]

/// Values to keep last X bits of a u8
/// `KEEP_LAST[i]` equates `(1 << i) - 1`
pub const KEEP_LAST: [usize; 9] = [0, 1, 3, 7, 15, 31, 63, 127, 255];

/// Struct containing an u8-array of C size to store bitwisely boolean values
pub struct BitString<const C: usize> {
    pub len: usize,
    pub data: [u8; C],
}

impl<const C: usize> BitString<C> {
    /// Instanciates a new BitString
    ///
    /// # Example
    /// ```
    /// let bs = BitString::<50>::new();
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
    /// ```
    /// let mut array = [0; 50];
    /// array[0] = 0b11010000;
    /// let bs = BitString::<50>::from(array, 4);
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

    #[cfg(test)]
    /// Returns a string visualization of the BitString
    ///
    /// # Example
    /// ```
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
    /// Pushes one value in the BitString
    ///
    /// # Example
    /// ```
    /// let mut bs = BitString::<50>::new();
    /// bs = BitString::push(bs, true);
    /// assert_eq!(bs.as_string(), "1");
    /// ```
    pub const fn push(mut bs: BitString<C>, bit: bool) -> BitString<C> {
        bs.data[bs.len / 8] |= (bit as u8) << (7 - bs.len % 8);
        bs.len += 1;
        bs
    }

    #[inline(always)]
    /// Pushes height values in the BitString
    ///
    /// # Example
    /// ```
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
            bs.data[first_idx] |= bits >> right;
            bs.data[first_idx + 1] |= (bits & ((1 << left) - 1)) << right;
        }

        bs.len += 8;
        bs
    }

    #[inline(always)]
    /// Pushes the u8 array in the BitString
    ///
    /// # Example
    /// ```
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
                bs.data[first_idx] |= bits >> right;
                bs.data[first_idx + 1] |= (bits & ((1 << left) - 1)) << right;
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
    /// ```
    /// let mut bs = BitString::<50>::new();
    /// bs = BitString::push_bits(bs, 0b1000_1000_0110_1110, 16);
    /// assert_eq!(bs.as_string(), "1000100001101110");
    /// ```
    pub const fn push_bits(mut bs: BitString<C>, bits: usize, len: usize) -> BitString<C> {
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
}
