//! Contains all functions required to encode any string as a QRCode

#![deny(unsafe_code)]
#![warn(missing_docs)]

#[cfg(test)]
mod test;

use super::bitstring::{self, BitString};
use super::hardcode;
use super::{Version, ECL};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Enum for the 3 encoding mode
pub enum Mode {
    Numeric,
    Alphanumeric,
    Byte,
}

/// Encodes the string according the mode and version
pub const fn encode(input: &[u8], ecl: ECL, mode: Mode, version: Version) -> BitString<2956> {
    let cci_bits = hardcode::cci_bits(version, mode);

    let bs = match mode {
        Mode::Numeric => encode_numeric(input, cci_bits),
        Mode::Alphanumeric => encode_alphanumeric(input, cci_bits),
        Mode::Byte => encode_byte(input, cci_bits),
    };

    let data_bits = hardcode::data_bits(version, ecl);

    let bs = add_terminator(bs, data_bits);
    let bs = pad_to_8(bs);
    let bs = fill(bs, data_bits);

    bs
}

/// Find the best encoding (Numeric -> Alnum -> Byte)
pub const fn best_encoding(input: &[u8]) -> Mode {
    const fn try_encode_numeric(input: &[u8], mut i: usize) -> Mode {
        while i < input.len() {
            if !input[i].is_ascii_digit() {
                return try_encode_alphanumeric(input, i);
            }
            i += 1;
        }
        Mode::Numeric
    }

    const fn try_encode_alphanumeric(input: &[u8], mut i: usize) -> Mode {
        while i < input.len() {
            if !is_qr_alphanumeric(input[i]) {
                return Mode::Byte;
            }
            i += 1;
        }
        Mode::Alphanumeric
    }

    try_encode_numeric(input, 0)
}

/// Encodes [numeric](https://www.thonky.com/qr-code-tutorial/numeric-mode-encoding) string
const fn encode_numeric(input: &[u8], cci_bits: usize) -> BitString<2956> {
    const fn encode_number(bs: BitString<2956>, number: usize) -> BitString<2956> {
        match number {
            0..=9 => BitString::push_bits(bs, number, 4),
            10..=99 => BitString::push_bits(bs, number, 7),
            /*100..=999*/ _ => BitString::push_bits(bs, number, 10),
        }
    }

    let bs = BitString::new();
    let bs = BitString::push_bits(bs, 0b0001, 4);
    let mut bs = BitString::push_bits(bs, input.len(), cci_bits);

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

/// Encodes [alphanumeric](https://www.thonky.com/qr-code-tutorial/alphanumeric-mode-encoding) string
const fn encode_alphanumeric(input: &[u8], cci_bits: usize) -> BitString<2956> {
    let bs = BitString::new();

    let bs = BitString::push_bits(bs, 0b0010, 4);
    let mut bs = BitString::push_bits(bs, input.len(), cci_bits);

    {
        // dupe of BitString::push_bits
        let mut i = 0;
        let len = input.len() - input.len() % 2;

        while i < len {
            let number = ascii_to_alphanumeric(input[i]) * 45 + ascii_to_alphanumeric(input[i + 1]);
            {
                const LEN_BITS: usize = 11;
                let bits = number & bitstring::KEEP_LAST[LEN_BITS];

                let rem_space = (8 - bs.len() % 8) % 8;
                let first = bs.len() / 8;

                if rem_space > LEN_BITS {
                    bs.data[first] |= (bits << (rem_space - LEN_BITS)) as u8;
                    bs.len += LEN_BITS;
                    return bs;
                }

                if rem_space != 0 {
                    bs.data[first] |=
                        ((bits >> (LEN_BITS - rem_space)) & bitstring::KEEP_LAST[rem_space]) as u8;
                    bs.len += rem_space;
                }

                let mut j = LEN_BITS - rem_space;
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
            bs = BitString::push_bits(bs, ascii_to_alphanumeric(input[i]), 6);
        }
    }

    bs
}

/// Encodes [any](https://www.thonky.com/qr-code-tutorial/byte-mode-encoding) string
const fn encode_byte(input: &[u8], cci_bits: usize) -> BitString<2956> {
    let bs = BitString::new();
    let bs = BitString::push_bits(bs, 0b0100, 4);
    let bs = BitString::push_bits(bs, input.len(), cci_bits);

    return BitString::push_u8_slice(bs, input);
}

/// Adds needed [terminator padding](https://www.thonky.com/qr-code-tutorial/data-encoding#add-a-terminator-of-0s-if-necessary)
/// as mentionned here
const fn add_terminator(bs: BitString<2956>, data_bits: usize) -> BitString<2956> {
    let mut len = data_bits - bs.len();

    if len > 4 {
        len = 4;
    }

    BitString::push_bits(bs, 0, len)
}

/// Adds the [padding](https://www.thonky.com/qr-code-tutorial/data-encoding#add-more-0s-to-make-the-length-a-multiple-of-8)
/// to make the lenght a multiple of 8
const fn pad_to_8(bs: BitString<2956>) -> BitString<2956> {
    let len = (8 - bs.len() % 8) % 8;

    BitString::push_bits(bs, 0, len)
}

/// Adds the [filling](https://www.thonky.com/qr-code-tutorial/data-encoding#add-pad-bytes-if-the-string-is-still-too-short)
/// to make the BitString to it's full [specified len](https://www.thonky.com/qr-code-tutorial/error-correction-table)
const fn fill(mut bs: BitString<2956>, data_bits: usize) -> BitString<2956> {
    bs = BitString::fill(bs, data_bits);
    bs
}

/// Converts ascii number to it's value in usize
/// "5" -> 5
const fn ascii_to_digit(c: u8) -> usize {
    (c - b'0') as usize
}

/// Converts ascii alnum to it's value in usize following
/// [specifications](https://www.thonky.com/qr-code-tutorial/alphanumeric-table)
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
        b'A' => 10,
        b'B' => 11,
        b'C' => 12,
        b'D' => 13,
        b'E' => 14,
        b'F' => 15,
        b'G' => 16,
        b'H' => 17,
        b'I' => 18,
        b'J' => 19,
        b'K' => 20,
        b'L' => 21,
        b'M' => 22,
        b'N' => 23,
        b'O' => 24,
        b'P' => 25,
        b'Q' => 26,
        b'R' => 27,
        b'S' => 28,
        b'T' => 29,
        b'U' => 30,
        b'V' => 31,
        b'W' => 32,
        b'X' => 33,
        b'Y' => 34,
        b'Z' => 35,
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

/// Checks if char is [alnum](https://www.thonky.com/qr-code-tutorial/alphanumeric-table)
const fn is_qr_alphanumeric(c: u8) -> bool {
    match c {
        b'A'..=b'Z'
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
