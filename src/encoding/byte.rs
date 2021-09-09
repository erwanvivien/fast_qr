//! Contains how to encode BYTE data

use crate::bitstorage;
use crate::vecl;

/// Verifies that `to_encode` consists of `ALPHANUMS` chars
fn verify(to_encode: &String) -> bool {
    for c in to_encode.chars() {
        if !c.is_ascii() {
            return false;
        }
    }

    return true;
}

fn encode_data(from: &[u8], bitstorage: &mut bitstorage::BitStorage) {
    for &e in from {
        bitstorage.push_u8(e);
    }
}

pub fn encode(from: &String, version: usize, quality: vecl::ECL) -> Option<bitstorage::BitStorage> {
    if !verify(&from) {
        return None;
    }

    let bytes = from.as_bytes();
    let mut new_res = bitstorage::BitStorage::new();

    new_res.push_last(0b0100u128, 4);
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
