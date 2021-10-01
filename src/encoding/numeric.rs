//! Contains how to encode NUMERIC data

use crate::bitstorage;
use crate::vecl;

/// Verifies that the input contains only digits
const fn verify(input: &[u8]) -> bool {
    let mut i = 0;

    while i < input.len() {
        if !input[i].is_ascii_digit() {
            return false;
        }

        i += 1;
    }

    true
}

/// Character count needs to have diff length between versions
const fn format_character_count(version: usize) -> usize {
    match version {
        1..=9 => 10,
        10..=26 => 12,
        27..=40 => 14,
        _ => 0,
    }
}

/**
 * Takes integers in group of 3 and takes them as is:
 * `1458` => '145' and '8' => [145, 8]
 * 3 digits takes 10 bits
 * 2 digits takes  7 bits
 * 1 digit  takes  4 bits
 */
fn encode_data(from: &[u8], bitstorage: &mut bitstorage::BitStorage) {
    let mut i = 0;
    while i < from.len() {
        let block = &from[i..std::cmp::min(from.len() - i, 3)];

        let mut nb_zero = 0;
        while block[nb_zero] == b'0' {
            nb_zero += 1;
        }

        let mut value: u128 = 0;

        {
            let mut i = 0;

            while i < block.len() {
                value *= 10;
                value += (block[i] - 48) as u128;
                i += 1;
            }
        }

        let bits = match (nb_zero, block.len()) {
            (1, 2) | (2, 3) | (0, 1) => 4,
            (0, 2) | (1, 3) => 7,
            (_, _) => 10,
        };

        bitstorage.push_last(value, bits);

        i += 1;
    }
}

/// Uses all the information to encode `from`
pub fn encode(from: &String, version: usize, quality: vecl::ECL) -> Option<bitstorage::BitStorage> {
    if !verify(&from.as_bytes()) {
        return None;
    }

    let bytes = from.as_bytes();
    let mut new_res = bitstorage::BitStorage::new();

    new_res.push_last(0b0001u128, 4);
    new_res.push_last(bytes.len() as u128, format_character_count(version));

    encode_data(bytes, &mut new_res);

    let max_bits = vecl::ecc_to_databits(quality, version) as usize;
    new_res.push_last(0u128, super::terminator_count(new_res.len(), max_bits));

    let padding_to_8 = (8 - (new_res.len() % 8)) % 8;
    new_res.push_last(0u128, padding_to_8);

    const PADDING_TO_MAX_VALUES: [u8; 2] = [0b11101100, 0b00010001];
    let padding_to_max = (max_bits - new_res.len()) / 8;
    for i in 0..padding_to_max {
        new_res.push_u8(PADDING_TO_MAX_VALUES[i % 2]);
    }

    return Some(new_res);
}
