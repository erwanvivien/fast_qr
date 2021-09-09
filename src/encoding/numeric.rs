//! Contains how to encode NUMERIC data

use crate::bitstorage;
use crate::vecl;

const NUMERIC: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Verifies that `to_encode` consists of `ALPHANUMS` chars
fn verify(to_encode: &String) -> bool {
    for c in to_encode.chars() {
        if !NUMERIC.contains(&c) {
            return false;
        }
    }

    return true;
}

fn encode_data(from: &[u8], bitstorage: &mut bitstorage::BitStorage) {
    for block in from.chunks(3) {
        let mut nb_zero = 0;
        while block[nb_zero] == b'0' {
            nb_zero += 1;
        }

        let mut value: u128 = 0;
        for b in block {
            value *= 10;
            value += (b - 48) as u128;
        }

        let bits = match (nb_zero, block.len()) {
            (0, 2) | (1, 3) => 7,
            (1, 2) | (2, 3) | (0, 1) => 4,
            (_, _) => 10,
        };

        bitstorage.push_last(value, bits);
    }
}

pub fn encode(from: String, version: usize, quality: vecl::ECL) -> Option<bitstorage::BitStorage> {
    if !verify(&from) {
        return None;
    }

    let bytes = from.as_bytes();
    let mut new_res = bitstorage::BitStorage::new();

    new_res.push_last(0b0010u128, 4);
    new_res.push_last(bytes.len() as u128, super::format_character_count(version));

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
