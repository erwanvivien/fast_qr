//! Contains how to encode KANJI data

use crate::bitstorage;
use crate::vecl;

pub fn verify_one(_: &char) -> bool {
    return false;
}

/// Verifies that `to_encode` consists of `ALPHANUMS` chars
fn verify(_: &String) -> bool {
    return false;
}

pub fn encode(from: &String, version: usize, quality: vecl::ECL) -> Option<bitstorage::BitStorage> {
    return None;
}
