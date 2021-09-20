//! Contains how to encode KANJI data

use crate::bitstorage;
use crate::vecl;

/// Verifies that `c` is a `KANJI` char
pub fn verify_one(c: &char) -> bool {
    let value = *c;
    return (value >= '\u{8140}' && value <= '\u{9ffc}')
        || (value >= '\u{E040}' && value <= '\u{ebbf}');
}

/// Verifies that `to_encode` consists of `KANJI` chars
fn verify(to_encode: &String) -> bool {
    for c in to_encode.chars() {
        if !verify_one(&c) {
            return false;
        }
    }

    return true;
}

/// Character count needs to have diff length between versions
const fn format_character_count(version: usize) -> usize {
    return match version {
        1..=9 => 8,
        10..=26 => 10,
        27..=40 => 12,
        _ => 0,
    };
}
/**
 * Takes the string (UTF-8) and converts it to SHIFT-JIS encoding
 * Then does some tricks to get the value in 13-bits
 */
fn encode_data(from: &String, bitstorage: &mut bitstorage::BitStorage) {
    for c in from.chars() {
        println!("{:?}", c);
        println!("{:?}", c as u16);
    }
}

/// Uses all the information to encode `from`
pub fn encode(from: &String, version: usize, quality: vecl::ECL) -> Option<bitstorage::BitStorage> {
    if !verify(&from) {
        return None;
    }

    let mut new_res = bitstorage::BitStorage::new();

    new_res.push_last(0b1000u128, 4);
    new_res.push_last(from.len() as u128, format_character_count(version));

    // encode_data(bytes, &mut new_res);

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
