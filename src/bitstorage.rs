const TYPE_SIZE: usize = 64;

fn verify(binstr: &str) -> bool {
    for c in binstr.chars() {
        if c != '0' && c != '1' {
            return false;
        }
    }
    return true;
}

pub struct BitStorage {
    bits: Vec<u64>,
    len: usize,
    capacity: usize,
}

const KEEP_LAST_X: [u64; 65] = [
    0,
    1,
    3,
    7,
    15,
    31,
    63,
    127,
    255,
    511,
    1023,
    2047,
    4095,
    8191,
    16383,
    32767,
    65535,
    131071,
    262143,
    524287,
    1048575,
    2097151,
    4194303,
    8388607,
    16777215,
    33554431,
    67108863,
    134217727,
    268435455,
    536870911,
    1073741823,
    2147483647,
    4294967295,
    8589934591,
    17179869183,
    34359738367,
    68719476735,
    137438953471,
    274877906943,
    549755813887,
    1099511627775,
    2199023255551,
    4398046511103,
    8796093022207,
    17592186044415,
    35184372088831,
    70368744177663,
    140737488355327,
    281474976710655,
    562949953421311,
    1125899906842623,
    2251799813685247,
    4503599627370495,
    9007199254740991,
    18014398509481983,
    36028797018963967,
    72057594037927935,
    144115188075855871,
    288230376151711743,
    576460752303423487,
    1152921504606846975,
    2305843009213693951,
    4611686018427387903,
    9223372036854775807,
    18446744073709551615,
];

impl BitStorage {
    pub fn push_one(&mut self, value: bool) {
        if self.len + 1 > self.capacity {
            self.bits.push(0);
            self.capacity += TYPE_SIZE;
        }

        let to_add = if value { 1 } else { 0 };

        self.bits[self.len / TYPE_SIZE] |= to_add << (63 - self.len % TYPE_SIZE);
        self.len += 1;
    }

    pub fn push_u8(&mut self, value: u8) {
        const CURRENT_TYPE_SIZE: usize = 8;

        let value_as_u64 = value as u64;

        let size = self.len;
        if size + CURRENT_TYPE_SIZE > self.capacity {
            self.bits.push(0);
            self.capacity += TYPE_SIZE;
        }

        let capacity = self.capacity;

        if size + CURRENT_TYPE_SIZE > capacity {
            let right_size = size + CURRENT_TYPE_SIZE - capacity;
            let mut left = value_as_u64 & (KEEP_LAST_X[capacity - size] << (right_size));
            left >>= right_size;
            let right = value_as_u64 & KEEP_LAST_X[right_size];

            self.bits[size / TYPE_SIZE] |= left;
            self.bits[size / TYPE_SIZE + 1] |= right << (63 - right_size);

            self.len += CURRENT_TYPE_SIZE;
        } else {
            self.bits[size / TYPE_SIZE] |= value_as_u64 << (64 - 8 - (size % TYPE_SIZE));
            self.len += CURRENT_TYPE_SIZE;
        }
    }

    pub fn push_u16(&mut self, value: u16) {
        self.push_u8((value >> 8) as u8);
        self.push_u8((value >> 0) as u8);
    }

    pub fn push_u32(&mut self, value: u32) {
        self.push_u16((value >> 16) as u16);
        self.push_u16((value >> 00) as u16);
    }

    pub fn push_u64(&mut self, value: u64) {
        self.push_u32((value >> 32) as u32);
        self.push_u32((value >> 00) as u32);
    }

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

    pub fn push_u128(&mut self, value: u128) {
        self.push_u64((value >> 64) as u64);
        self.push_u64((value >> 00) as u64);
    }

    pub fn push_binstr(&mut self, value: &str) {
        let is_ok = verify(value);
        if !is_ok {
            panic!(
                "Couldn't push a binary string that contains an element different that '0' or '1'"
            );
        }

        for c in value.chars() {
            self.push_one(if c == '1' { true } else { false });
        }
    }

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

fn main() {
    let mut b = BitStorage::new();

    b.push_last(0b110001, 6);
    b.push_last(0b110001, 6);
    println!("{:b}", b);
}
