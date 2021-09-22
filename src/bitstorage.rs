//! New type: Contains bits as a vector of u64
#![deny(unsafe_code)]
#![warn(missing_docs)]

/// u64 has 64 bits.
const TYPE_SIZE: usize = 64;

#[cfg(test)]
fn verify(binstr: &str) -> bool {
    for c in binstr.chars() {
        if c != '0' && c != '1' {
            return false;
        }
    }
    return true;
}

/// BitStorage has a len & capacity attributes
pub struct BitStorage {
    bits: Vec<u64>,
    len: usize,
    capacity: usize,
}

impl BitStorage {
    /// Can push a bool, will extend len by 1
    pub fn push_one(&mut self, value: bool) {
        if self.len + 1 > self.capacity {
            self.bits.push(0);
            self.capacity += TYPE_SIZE;
        }

        let to_add = if value { 1 } else { 0 };
        self.bits[self.len / TYPE_SIZE] |= to_add << (63 - self.len % TYPE_SIZE);
        self.len += 1;
    }

    /// Can push a u8, will extend len by 8
    pub fn push_u8(&mut self, value: u8) {
        const CURRENT_TYPE_SIZE: usize = 8;

        let size = self.len;
        let capacity = self.capacity;

        let value_as_u64 = value as u64;
        if size + CURRENT_TYPE_SIZE > capacity {
            for i in (0..8).rev() {
                let is_set = value & (1 << i);
                self.push_one(if is_set != 0 { true } else { false });
            }
        } else {
            self.bits[size / TYPE_SIZE] |= value_as_u64 << (64 - 8 - (size % TYPE_SIZE));
            self.len += CURRENT_TYPE_SIZE;
        }
    }

    /// Can push a u16, will extend len by 16
    pub fn push_u16(&mut self, value: u16) {
        self.push_u8((value >> 8) as u8);
        self.push_u8((value >> 0) as u8);
    }

    /// Can push a u32, will extend len by 32
    pub fn push_u32(&mut self, value: u32) {
        self.push_u16((value >> 16) as u16);
        self.push_u16((value >> 00) as u16);
    }

    /// Can push a u64, will extend len by 64
    pub fn push_u64(&mut self, value: u64) {
        self.push_u32((value >> 32) as u32);
        self.push_u32((value >> 00) as u32);
    }

    /// Can push a u128, will extend len by 128
    pub fn push_u128(&mut self, value: u128) {
        self.push_u64((value >> 64) as u64);
        self.push_u64((value >> 00) as u64);
    }

    /// Can push any integer, will extend len by `last`
    pub fn push_last(&mut self, value: u128, last: usize) {
        const POSSIBLE: [usize; 6] = [128, 64, 32, 16, 8, 1];
        let mut size = last;

        for i in 0..POSSIBLE.len() {
            let p = POSSIBLE[i];
            if p > size {
                continue;
            }

            while size >= p {
                let keep = if p == 128 { u128::MAX } else { (1 << p) - 1 };
                let tmp = (value >> (size - p)) & keep;

                match p {
                    128 => self.push_u128(tmp as u128),
                    64 => self.push_u64(tmp as u64),
                    32 => self.push_u32(tmp as u32),
                    16 => self.push_u16(tmp as u16),
                    8 => self.push_u8(tmp as u8),
                    1 => self.push_one(if tmp != 0 { true } else { false }),
                    _ => panic!("push_last: impossible"),
                }
                size -= p;
            }
        }
    }

    #[cfg(test)]
    pub fn push_binstr(&mut self, value: &str) {
        let is_ok = verify(value);
        if !is_ok {
            panic!(
                "Couldn't push a binary string that contains an element different that '0' or '1'"
            );
        }

        for c in value.chars() {
            self.push_one(c == '1');
        }
    }

    /// Returns the length
    pub fn len(&self) -> usize {
        return self.len;
    }

    /// Returns the length
    #[allow(dead_code)]
    pub fn capactiy(&self) -> usize {
        return self.capacity;
    }

    /// Converts the u64 vec to a u8 vec
    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        for i in (0..self.len).step_by(8) {
            let bit = self.bits[i / TYPE_SIZE];
            let shift = 64 - 8 - (i % TYPE_SIZE);

            let keep = bit >> shift;
            vec.push(keep as u8)
        }

        return vec;
    }

    /// Basic function to init the BitStorage
    pub const fn new() -> BitStorage {
        return BitStorage {
            bits: Vec::new(),
            len: 0,
            capacity: 0,
        };
    }
}

impl std::fmt::Binary for BitStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();

        for i in 0..self.bits.len() {
            let bit = self.bits[i];
            let max = if i + 1 == self.bits.len() {
                self.len % TYPE_SIZE
            } else {
                TYPE_SIZE
            };

            for i in 0..max {
                let bit_active = bit & (1 << (63 - i));
                result.push(if bit_active != 0 { '1' } else { '0' });
            }
        }

        if result.len() % 8 != 0 {
            write!(
                f,
                "{} {}",
                &result[..result.len() % 8],
                result[result.len() % 8..]
                    .chars()
                    .enumerate()
                    .flat_map(|(i, c)| {
                        if i != 0 && i % 8 == 0 {
                            Some(' ')
                        } else {
                            None
                        }
                        .into_iter()
                        .chain(std::iter::once(c))
                    })
                    .collect::<String>()
            )
        } else {
            write!(f, "{}", result)
        }
    }
}

impl std::fmt::Display for BitStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = self
            .bits
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        write!(f, "[{}]", c.join(", "))
    }
}

impl std::fmt::Debug for BitStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = self
            .bits
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        write!(f, "[{}], size: {}", c.join(", "), self.len)
    }
}

impl std::ops::Index<usize> for BitStorage {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        static T: bool = true;
        static F: bool = false;

        if index >= self.len {
            panic!("Index {} is out of bounds", index);
        }

        let bit_pos = index / TYPE_SIZE;
        let bit = self.bits[bit_pos];

        let shift = 1 << (63 - (index % TYPE_SIZE));
        return if (bit & shift) != 0 { &T } else { &F };
    }
}

impl IntoIterator for BitStorage {
    type Item = bool;
    type IntoIter = BitStorageIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitStorageIterator {
            bitstorage: self,
            index: 0,
        }
    }
}

/// The structure to convert BitStorage to an Iterator
pub struct BitStorageIterator {
    bitstorage: BitStorage,
    index: usize,
}

impl Iterator for BitStorageIterator {
    type Item = bool;
    /// Access different part of the u64 vector
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.bitstorage.len {
            return None;
        }

        let result = self.bitstorage[self.index];
        self.index += 1;
        return Some(result);
    }
}
