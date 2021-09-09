//! Contains how to encode ALNUM data

use crate::bitstorage;
use crate::vecl;

/// Authorized characters for `ALNUM` QR-Codes
const ALPHANUMS: [char; 45] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' ', '$',
    '%', '*', '+', '-', '.', '/', ':',
];

/// Reversed map of `ALPHANUMS` to get the index
const REVERSE_ALPHANUMS: [u16; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    36, 0, 0, 0, 37, 38, 0, 0, 0, 0, 39, 40, 0, 41, 42, 43, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 44, 0, 0,
    0, 0, 0, 0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// Verifies that `to_encode` consists of `ALPHANUMS` chars
fn verify(to_encode: &String) -> bool {
    for c in to_encode.chars() {
        if c.is_ascii() && !ALPHANUMS.contains(&c) {
            return false;
        }
    }

    return true;
}

/// Character count needs to have diff length between versions
const fn format_character_count(version: usize) -> usize {
    return match version {
        1..=9 => 9,
        10..=26 => 11,
        27..=40 => 13,
        _ => 0,
    };
}

/**
 * Takes a string and makes pairs, to add them up
 *
 * `"HELLO WORLD"` => `["HE", "LL", "O ", "WO", "RL", "D"]`
 *
 * Then we add them thanks to their values in `REVERSE_ALPHANUMS`
 * (i.e: "HE" => 17 * 45 + 14 = 779 = 0b011 0000 1011 encoded in 11 bits)
 *
 * If the string is odd, the last one is encoded on 6 bits
 */
fn encode_data(from: &[u8], bitstorage: &mut bitstorage::BitStorage) {
    // let mut res = String::new();

    let tmp = from
        .chunks_exact(2)
        .map(|a| REVERSE_ALPHANUMS[a[0] as usize] * 45 + REVERSE_ALPHANUMS[a[1] as usize]);
    for slice in tmp {
        bitstorage.push_last(slice as u128, 11);
        // res.push_str(&format!("{:011b}", slice));
    }

    if from.len() % 2 != 0 {
        // res.push_str(&format!(
        //     "{:06b}",
        //     REVERSE_ALPHANUMS[*from.last().unwrap() as usize],
        // ));
        bitstorage.push_last(REVERSE_ALPHANUMS[*from.last().unwrap() as usize] as u128, 6);
    }

    // return res;
}

/// Returns the required 0 to pad the string
fn terminator_count(len: usize, max_len: usize) -> usize {
    return std::cmp::min(max_len - len, 4);
}

/// Uses all the information to encode `from`
pub fn encode_alphanum(
    from: String,
    version: usize,
    quality: vecl::ECL,
) -> Option<bitstorage::BitStorage> {
    if !verify(&from) {
        return None;
    }

    let bytes = from.as_bytes();
    let mut new_res = bitstorage::BitStorage::new();

    new_res.push_last(0b0010u128, 4);
    new_res.push_last(bytes.len() as u128, format_character_count(version));

    encode_data(bytes, &mut new_res);

    let max_bits = vecl::ecc_to_databits(quality, version) as usize;
    new_res.push_last(0u128, terminator_count(new_res.len(), max_bits));

    let padding_to_8 = (8 - (new_res.len() % 8)) % 8;
    new_res.push_last(0u128, padding_to_8);

    const PADDING_TO_MAX_VALUES: [u8; 2] = [0b11101100, 0b00010001];
    let padding_to_max = (max_bits - new_res.len()) / 8;
    for i in 0..padding_to_max {
        new_res.push_u8(PADDING_TO_MAX_VALUES[i % 2]);
    }

    return Some(new_res);
}
