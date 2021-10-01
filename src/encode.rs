use crate::bitstack::{self};
use crate::vecl::ECL;
use crate::version::Version;

type BitStack = bitstack::BitStack<23648>;

pub const fn encode(input: &[u8], ecl: ECL) -> Option<BitStack> {
    try_encode_numeric(input, ecl)
}

const fn try_encode_numeric(input: &[u8], ecl: ECL) -> Option<BitStack> {
    let mut i = 0;

    loop {
        if i == input.len() {
            break;
        }

        if !input[i].is_ascii_digit() {
            return try_encode_alphanumeric(input, ecl, i);
        }

        i += 1;
    }

    encode_numeric(input, ecl)
}

const fn try_encode_alphanumeric(input: &[u8], ecl: ECL, mut i: usize) -> Option<BitStack> {
    loop {
        if i == input.len() {
            break;
        }

        if !is_qr_alphanumeric(input[i]) {
            return encode_byte(input, ecl);
        }

        i += 1;
    }

    encode_alphanumeric(input, ecl)
}

const fn encode_numeric(input: &[u8], ecl: ECL) -> Option<BitStack> {
    const fn encode_number(bs: BitStack, number: usize) -> BitStack {
        match number {
            0..=9 => bitstack::push_bits(bs, number, 4),
            10..=99 => bitstack::push_bits(bs, number, 7),
            /*100..=999*/ _ => bitstack::push_bits(bs, number, 10),
        }
    }

    let version = match Version::from_len_numeric(input.len(), ecl) {
        Some(version) => version,
        None => return None,
    };

    let bs = BitStack::new();

    let bs = bitstack::push_slice(bs, &[false, false, false, true]);

    let mut bs = bitstack::push_bits(bs, input.len(), version.cci_size_numeric());

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

    None
}

const fn encode_alphanumeric(input: &[u8], ecl: ECL) -> Option<BitStack> {
    let version = match Version::from_len_alphanumeric(input.len(), ecl) {
        Some(version) => version,
        None => return None,
    };

    let bs = BitStack::new();

    let bs = bitstack::push_slice(bs, &[false, false, true, false]);

    let mut bs = bitstack::push_bits(bs, input.len(), version.cci_size_alphanumeric());

    {
        let mut i = 0;
        let len = input.len() - input.len() % 2;

        while i < len {
            let number = ascii_to_alphanumeric(input[i]) * 45 + ascii_to_alphanumeric(input[i + 1]);

            bs = bitstack::push_bits(bs, number, 11);

            i += 2;
        }

        if len != input.len() {
            bs = bitstack::push_bits(bs, ascii_to_alphanumeric(input[i]), 6);
        }
    }

    None
}

const fn encode_byte(input: &[u8], ecl: ECL) -> Option<BitStack> {
    let version = match Version::from_len_byte(input.len(), ecl) {
        Some(version) => version,
        None => return None,
    };

    let bs = BitStack::new();

    let bs = bitstack::push_slice(bs, &[false, true, false, false]);

    let mut bs = bitstack::push_bits(bs, input.len(), version.cci_size_byte());

    {
        let mut i = 0;

        while i < input.len() {
            bs = bitstack::push_u8(bs, input[i]);

            i += 1;
        }
    }

    None
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
