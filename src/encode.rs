//! Contains all functions required to encode any string as a `QRCode`

#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::compact::CompactQR;
use crate::ecl::ECL;
use crate::hardcode;
use crate::version::Version;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Enum for the 3 encoding mode
pub enum Mode {
    Numeric,
    Alphanumeric,
    Byte,
}

/// Encodes the string according the mode and version
pub fn encode(input: &[u8], ecl: ECL, mode: Mode, version: Version) -> CompactQR {
    let cci_bits = hardcode::cci_bits(version, mode);

    let mut compact = CompactQR::from_version(version);

    match mode {
        Mode::Numeric => encode_numeric(&mut compact, input, cci_bits),
        Mode::Alphanumeric => encode_alphanumeric(&mut compact, input, cci_bits),
        Mode::Byte => encode_byte(&mut compact, input, cci_bits),
    };

    let data_bits = hardcode::data_bits(version, ecl);

    add_terminator(&mut compact, data_bits);
    pad_to_8(&mut compact);
    compact.fill();

    compact
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

/// Encodes numeric strings (i.e. "123456789"), referring to 8.4.2 of the spec.
pub(crate) fn encode_numeric(compact: &mut CompactQR, input: &[u8], cci_bits: usize) {
    enum NumericEncoding {
        Single,
        Double,
        Triple,
    }

    fn encode_number(compact: &mut CompactQR, number: usize, encoding: NumericEncoding) {
        match encoding {
            NumericEncoding::Single => compact.push_bits(number, 4),
            NumericEncoding::Double => compact.push_bits(number, 7),
            NumericEncoding::Triple => compact.push_bits(number, 10),
        }
    }

    compact.push_bits(0b0001, 4);
    compact.push_bits(input.len(), cci_bits);

    let mut i = 0;
    let len = input.len() - input.len() % 3;

    while i < len {
        let number = ascii_to_digit(input[i]) * 100
            + ascii_to_digit(input[i + 1]) * 10
            + ascii_to_digit(input[i + 2]);

        encode_number(compact, number, NumericEncoding::Triple);
        i += 3;
    }

    // If the length is a multiple of 3, we are done
    if len == input.len() {
        return;
    }

    let mut number = 0;
    while i < input.len() {
        number *= 10;
        number += ascii_to_digit(input[i]);
        i += 1;
    }

    let encoding = match i % 3 {
        1 => NumericEncoding::Single,
        2 => NumericEncoding::Double,
        _ => unreachable!("i % 3 can only be 1 or 2"),
    };

    encode_number(compact, number, encoding);
}

/// Encodes alphanumeric strings (i.e. "FAST-QR123"), referring to 8.4.3 of the spec.
pub(crate) fn encode_alphanumeric(compact: &mut CompactQR, input: &[u8], cci_bits: usize) {
    compact.push_bits(0b0010, 4);
    compact.push_bits(input.len(), cci_bits);

    let even_size = input.len() - input.len() % 2;
    for chunk in input.chunks_exact(2) {
        let a = ascii_to_alphanumeric(chunk[0]);
        let b = ascii_to_alphanumeric(chunk[1]);
        compact.push_bits(a * 45 + b, 11);
    }
    if even_size != input.len() {
        compact.push_bits(ascii_to_alphanumeric(*input.last().unwrap()), 6);
    }
}

/// Encodes any string (i.e. "<https://fast-qr.com/🚀>"), referring to 8.4.4 of the spec.
pub(crate) fn encode_byte(compact: &mut CompactQR, input: &[u8], cci_bits: usize) {
    compact.push_bits(0b0100, 4);
    compact.push_bits(input.len(), cci_bits);
    compact.push_u8_slice(input);
}

/// Adds needed terminator padding, terminating the data `BitString`, referring to 8.4.8 of the spec.
fn add_terminator(compact: &mut CompactQR, data_bits: usize) {
    let len = data_bits - compact.len();
    let len = core::cmp::min(len, 4);

    compact.push_bits(0, len);
}

/// Adds the padding to make the length of the `BitString` a multiple of 8, referring to 8.4.9 of the spec.
fn pad_to_8(compact: &mut CompactQR) {
    let len = (8 - compact.len() % 8) % 8;
    compact.push_bits(0, len);
}

/// Converts ascii number to it's value in usize
/// "5" -> 5
const fn ascii_to_digit(c: u8) -> usize {
    (c - b'0') as usize
}

/// Converts ascii alnum to it's numeric value, characters included in `AlphaNumeric` are: \
/// 0-9, A-Z, $%*./:+-?.= [space] \
/// referring to 7.1 of the spec.
pub(crate) fn ascii_to_alphanumeric(c: u8) -> usize {
    match c {
        b'0'..=b'9' => (c - b'0') as usize,
        b'A'..=b'Z' => (c - b'A') as usize + 10,
        b' ' => 36,
        b'$' => 37,
        b'%' => 38,
        b'*' => 39,
        b'+' => 40,
        b'-' => 41,
        b'.' => 42,
        b'/' => 43,
        b':' => 44,
        _ => panic!("Character '{}' should not occur", c as char), // unreachable!()
    }
}

/// Checks if character c is alphanumeric: 0-9, A-Z, $%*./:+-?.= [space] \
/// referring to 7.1 of the spec.
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
