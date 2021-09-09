//! Contains how to encode ALNUM data

use super::vecl;

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
fn format_character_count(b: u16, version: usize) -> String {
    return match version {
        1..=9 => format!("{:09b}", b),
        10..=26 => format!("{:011b}", b),
        27..=40 => format!("{:013b}", b),
        _ => String::new(),
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
fn encode_data(from: &[u8]) -> String {
    let mut res = String::new();

    let tmp = from
        .chunks_exact(2)
        .map(|a| REVERSE_ALPHANUMS[a[0] as usize] * 45 + REVERSE_ALPHANUMS[a[1] as usize]);
    for slice in tmp {
        res.push_str(&format!("{:011b}", slice));
    }

    if from.len() % 2 != 0 {
        res.push_str(&format!(
            "{:06b}",
            REVERSE_ALPHANUMS[*from.last().unwrap() as usize],
        ));
    }

    return res;
}

/// Returns the required 0 to pad the string
fn terminator_count(len: usize, max_len: usize) -> usize {
    return std::cmp::min(max_len - len, 4);
}

/// Uses all the information to encode `from`
pub fn encode_alphanum(from: String, version: usize, quality: vecl::ECL) -> String {
    if !verify(&from) {
        return String::new();
    }

    let bytes = from.as_bytes();
    let mut res = String::new();

    res.push_str("0010");
    res.push_str(&format_character_count(bytes.len() as u16, version));
    res.push_str(&encode_data(bytes));

    let max_bits = vecl::ecc_to_databits(quality, version) as usize;
    for _ in 0..terminator_count(res.len(), max_bits) {
        res.push('0');
    }

    let padding_to_8 = (8 - (res.len() % 8)) % 8;

    for _ in 0..padding_to_8 {
        res.push('0');
    }

    const PADDING_TO_MAX_VALUES: [&str; 2] = ["11101100", "00010001"];
    let padding_to_max = (max_bits - res.len()) / 8;
    for i in 0..padding_to_max {
        res.push_str(PADDING_TO_MAX_VALUES[i % 2]);
    }

    return res;
}
