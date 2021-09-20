//! Contains all encoding for
//! `[ALNUM, NUMERIC, KANJI, BYTE]` modes

pub mod alphanum;
pub mod byte;
pub mod kanji;
pub mod numeric;

use crate::bitstorage;
use crate::vecl;

/// Returns the required 0 to pad the string
fn terminator_count(len: usize, max_len: usize) -> usize {
    return std::cmp::min(max_len - len, 4);
}

/// All possible verification 'algorithm'
const POSSIBLE_VERIFY: [for<'r> fn(&'r char) -> bool; 4] = [
    numeric::verify_one,
    alphanum::verify_one,
    kanji::verify_one,
    byte::verify_one,
];

/// The associated encoding
pub const POSSIBLE_ENCODINGS: [for<'r> fn(
    &'r String,
    usize,
    vecl::ECL,
) -> Option<bitstorage::BitStorage>; 4] = [
    numeric::encode,
    alphanum::encode,
    kanji::encode,
    byte::encode,
];

/// Loops only once in the string to figure out what is the best encoding
pub fn best_encoding(s: &String) -> usize {
    let mut best = 0;
    for c in s.chars() {
        while !POSSIBLE_VERIFY[best](&c) {
            best += 1;
        }
    }

    return best;
}
