//! Contains all encoding for
//! `[ALNUM, NUMERIC, KANJI, BYTE]` modes

pub mod alphanum;
pub mod byte;
pub mod numeric;

use crate::bitstorage;
use crate::vecl;

/// Returns the required 0 to pad the string
fn terminator_count(len: usize, max_len: usize) -> usize {
    return std::cmp::min(max_len - len, 4);
}

pub fn encode(from: &String, version: usize, quality: vecl::ECL) -> Option<bitstorage::BitStorage> {
    numeric::encode(from, version, quality)
}
