//! Contains all functions required to encode any string as a QRCode

#![deny(unsafe_code)]
#![warn(missing_docs)]

#[cfg(test)]
mod test;

use crate::runtime::qrcode::bitstring::BitString;
use crate::runtime::qrcode::ecl::ECL;
use crate::runtime::qrcode::hardcode;
use crate::runtime::qrcode::version::Version;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Enum for the 3 encoding mode
pub enum Mode {
    Numeric,
    Alphanumeric,
    Byte,
}

/// Encodes the string according the mode and version
pub fn encode(input: &[u8], ecl: ECL, mode: Mode, version: Version) -> BitString<2956> {
    let cci_bits = hardcode::cci_bits(version, mode);

    let mut bs = match mode {
        Mode::Numeric => encode_numeric(input, cci_bits),
        Mode::Alphanumeric => encode_alphanumeric(input, cci_bits),
        Mode::Byte => encode_byte(input, cci_bits),
    };

    let data_bits = hardcode::data_bits(version, ecl);

    add_terminator(&mut bs, data_bits);
    pad_to_8(&mut bs);
    bs.fill(data_bits);
    bs
}

/// Find the best encoding (Numeric -> Alnum -> Byte)
pub fn best_encoding(input: &[u8]) -> Mode {
    fn try_encode_numeric(input: &[u8], i: usize) -> Mode {
        for &c in input.iter().skip(i) {
            if !c.is_ascii_digit() {
                return try_encode_alphanumeric(input, i);
            }
        }
        Mode::Numeric
    }

    fn try_encode_alphanumeric(input: &[u8], i: usize) -> Mode {
        for &c in input.iter().skip(i) {
            if !is_qr_alphanumeric(c) {
                return Mode::Byte;
            }
        }
        Mode::Alphanumeric
    }

    try_encode_numeric(input, 0)
}

/// Encodes [numeric](https://www.thonky.com/qr-code-tutorial/numeric-mode-encoding) string
fn encode_numeric(input: &[u8], cci_bits: usize) -> BitString<2956> {
    fn encode_number(bs: &mut BitString<2956>, number: usize) {
        match number {
            0..=9 => bs.push_bits(number, 4),
            10..=99 => bs.push_bits(number, 7),
            /*100..=999*/ _ => bs.push_bits(number, 10),
        }
    }

    let mut bs = BitString::new();

    bs.push_bits(0b0001, 4);
    bs.push_bits(input.len(), cci_bits);

    {
        let mut i = 0;
        let len = input.len() - input.len() % 3;

        while i < len {
            let number = ascii_to_digit(input[i]) * 100
                + ascii_to_digit(input[i + 1]) * 10
                + ascii_to_digit(input[i + 2]);

            encode_number(&mut bs, number);
            i += 3;
        }

        if len != input.len() {
            let mut number = 0;

            while i < input.len() {
                number *= 10;
                number += ascii_to_digit(input[i]);
                i += 1;
            }

            encode_number(&mut bs, number);
        }
    }

    bs
}

/// Encodes [alphanumeric](https://www.thonky.com/qr-code-tutorial/alphanumeric-mode-encoding) string
fn encode_alphanumeric(input: &[u8], cci_bits: usize) -> BitString<2956> {
    let mut bs = BitString::new();

    bs.push_bits(0b0010, 4);
    bs.push_bits(input.len(), cci_bits);

    let even_size = input.len() - input.len() % 2;
    bs.push_u8_slice(&input[..even_size]);
    if even_size != input.len() {
        bs.push_bits(ascii_to_alphanumeric(*input.last().unwrap()), 6);
    }

    bs
}

/// Encodes [any](https://www.thonky.com/qr-code-tutorial/byte-mode-encoding) string
fn encode_byte(input: &[u8], cci_bits: usize) -> BitString<2956> {
    let mut bs = BitString::new();

    bs.push_bits(0b0100, 4);
    bs.push_bits(input.len(), cci_bits);
    bs.push_u8_slice(input);

    bs
}

/// Adds needed [terminator padding](https://www.thonky.com/qr-code-tutorial/data-encoding#add-a-terminator-of-0s-if-necessary)
/// as mentionned here
fn add_terminator(bs: &mut BitString<2956>, data_bits: usize) {
    let len = data_bits - bs.len();
    let len = std::cmp::min(len, 4);

    bs.push_bits(0, len)
}

/// Adds the [padding](https://www.thonky.com/qr-code-tutorial/data-encoding#add-more-0s-to-make-the-length-a-multiple-of-8)
/// to make the lenght a multiple of 8
fn pad_to_8(bs: &mut BitString<2956>) {
    let len = (8 - bs.len() % 8) % 8;
    bs.push_bits(0, len)
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
        _ => panic!(), // unreachable!()
    }
}

/// Checks if char is [alnum](https://www.thonky.com/qr-code-tutorial/alphanumeric-table)
const fn is_qr_alphanumeric(c: u8) -> bool {
    matches!(c,
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
        | b':')
}
