pub mod alphanum;
pub mod numeric;

/// Character count needs to have diff length between versions
const fn format_character_count(version: usize) -> usize {
    return match version {
        1..=9 => 9,
        10..=26 => 11,
        27..=40 => 13,
        _ => 0,
    };
}

/// Returns the required 0 to pad the string
fn terminator_count(len: usize, max_len: usize) -> usize {
    return std::cmp::min(max_len - len, 4);
}
