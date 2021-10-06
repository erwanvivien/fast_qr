use crate::bitstring::{self, BitString};
use crate::vecl::ECL;
use crate::version::Version;

#[derive(Clone, Copy)]
pub enum Mode {
    Numeric,
    Alphanumeric,
    Byte,
}

pub const fn encode(input: &[u8], ecl: ECL, mode: Mode, version: Version) -> BitString<2956> {
    let cci_bits = version.cci_bits(mode);

    let bs = match mode {
        Mode::Numeric => encode_numeric(input, cci_bits),
        Mode::Alphanumeric => encode_alphanumeric(input, cci_bits),
        Mode::Byte => encode_byte(input, cci_bits),
    };

    let data_bits = version.data_bits(ecl);

    let bs = add_terminator(bs, data_bits);
    let bs = pad_to_8(bs);
    let bs = fill(bs, data_bits);

    bs
}

pub const fn best_encoding(input: &[u8]) -> Mode {
    const fn try_encode_numeric(input: &[u8], mut i: usize) -> Mode {
        loop {
            if i == input.len() {
                break;
            }
            if !input[i].is_ascii_digit() {
                return try_encode_alphanumeric(input, i);
            }
            i += 1;
        }
        Mode::Numeric
    }

    const fn try_encode_alphanumeric(input: &[u8], mut i: usize) -> Mode {
        loop {
            if i == input.len() {
                break;
            }
            if !is_qr_alphanumeric(input[i]) {
                return Mode::Byte;
            }
            i += 1;
        }
        Mode::Alphanumeric
    }

    try_encode_numeric(input, 0)
}

const fn encode_numeric(input: &[u8], cci_bits: usize) -> BitString<2956> {
    const fn encode_number(bs: BitString<2956>, number: usize) -> BitString<2956> {
        match number {
            0..=9 => bitstring::push_bits(bs, number, 4),
            10..=99 => bitstring::push_bits(bs, number, 7),
            /*100..=999*/ _ => bitstring::push_bits(bs, number, 10),
        }
    }

    let bs = BitString::new();
    let bs = bitstring::push_bits(bs, 0b0001, 4);
    let mut bs = bitstring::push_bits(bs, input.len(), cci_bits);

    {
        let mut i = 0;
        let len = input.len() - input.len() % 3;

        while i < len {
            let number = ascii_to_digit(input[i]) * 100
                + ascii_to_digit(input[i + 1]) * 10
                + ascii_to_digit(input[i + 2]);

            bs = encode_number(bs, number);

            i += 3;
        }

        if len != input.len() {
            let mut number = 0;

            while i < input.len() {
                number *= 10;

                number += ascii_to_digit(input[i]);

                i += 1;
            }

            bs = encode_number(bs, number);
        }
    }

    bs
}

const fn encode_alphanumeric(input: &[u8], cci_bits: usize) -> BitString<2956> {
    let bs = BitString::new();

    let bs = bitstring::push_bits(bs, 0b0010, 4);
    let mut bs = bitstring::push_bits(bs, input.len(), cci_bits);

    {
        let mut i = 0;
        let len = input.len() - input.len() % 2;

        while i < len {
            let number = ascii_to_alphanumeric(input[i]) * 45 + ascii_to_alphanumeric(input[i + 1]);

            {
                let bits = number & ((1 << 11) - 1);

                let rem_space = (8 - bs.len() % 8) % 8;
                let first = bs.len() / 8;

                if rem_space > 11 {
                    bs.data[first] |= (bits >> (rem_space - 11)) as u8;
                    bs.len += 11;
                    return bs;
                }

                if rem_space != 0 {
                    bs.data[first] |=
                        ((bits >> (11 - rem_space)) & bitstring::KEEP_LAST[rem_space]) as u8;
                    bs.len += rem_space;
                }

                let mut j = 11 - rem_space;
                while j >= 8 {
                    bs.data[bs.len() / 8] = (bits >> (j - 8)) as u8;
                    bs.len += 8;
                    j -= 8;
                }

                if j != 0 {
                    bs.data[bs.len() / 8] = ((bits & bitstring::KEEP_LAST[j]) as u8) << (8 - j);
                    bs.len += j;
                }
            }

            i += 2;
        }

        if len != input.len() {
            bs = bitstring::push_bits(bs, ascii_to_alphanumeric(input[i]), 6);
        }
    }

    bs
}

const fn encode_byte(input: &[u8], cci_bits: usize) -> BitString<2956> {
    let bs = BitString::new();
    let bs = bitstring::push_bits(bs, 0b0100, 4);
    let bs = bitstring::push_bits(bs, input.len(), cci_bits);

    return bitstring::push_u8_slice(bs, input);
}

const fn add_terminator(bs: BitString<2956>, data_bits: usize) -> BitString<2956> {
    let mut len = data_bits - bs.len();

    if len > 4 {
        len = 4;
    }

    bitstring::push_bits(bs, 0, len)
}

const fn pad_to_8(bs: BitString<2956>) -> BitString<2956> {
    let len = (8 - bs.len() % 8) % 8;

    bitstring::push_bits(bs, 0, len)
}

const fn fill(mut bs: BitString<2956>, data_bits: usize) -> BitString<2956> {
    let pad_bytes = [0b11101100, 0b00010001];
    // let pad_bytes = [236, 17];

    let mut byte = false;

    while bs.len() < data_bits {
        bs = bitstring::push_u8(bs, pad_bytes[byte as usize]);
        byte = !byte;
    }

    bs
}

const fn ascii_to_digit(c: u8) -> usize {
    (c - b'0') as usize
}

const fn ascii_to_alphanumeric(c: u8) -> usize {
    match c {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'A' | b'a' => 10,
        b'B' | b'b' => 11,
        b'C' | b'c' => 12,
        b'D' | b'd' => 13,
        b'E' | b'e' => 14,
        b'F' | b'f' => 15,
        b'G' | b'g' => 16,
        b'H' | b'h' => 17,
        b'I' | b'i' => 18,
        b'J' | b'j' => 19,
        b'K' | b'k' => 20,
        b'L' | b'l' => 21,
        b'M' | b'm' => 22,
        b'N' | b'n' => 23,
        b'O' | b'o' => 24,
        b'P' | b'p' => 25,
        b'Q' | b'q' => 26,
        b'R' | b'r' => 27,
        b'S' | b's' => 28,
        b'T' | b't' => 29,
        b'U' | b'u' => 30,
        b'V' | b'v' => 31,
        b'W' | b'w' => 32,
        b'X' | b'x' => 33,
        b'Y' | b'y' => 34,
        b'Z' | b'z' => 35,
        b' ' => 36,
        b'$' => 37,
        b'%' => 38,
        b'*' => 39,
        b'+' => 40,
        b'-' => 41,
        b'.' => 42,
        b'/' => 43,
        b':' => 44,
        _ => usize::MAX, // unreachable!()
    }
}

const fn is_qr_alphanumeric(c: u8) -> bool {
    match c {
        b'A'..=b'Z'
        | b'a'..=b'z'
        | b'0'..=b'9'
        | b' '
        | b'$'
        | b'%'
        | b'*'
        | b'+'
        | b'-'
        | b'.'
        | b'/'
        | b':' => true,
        _ => false,
    }
}
