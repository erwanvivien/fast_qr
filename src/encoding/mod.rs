pub mod alphanum;
pub mod byte;
pub mod numeric;

/// Returns the required 0 to pad the string
fn terminator_count(len: usize, max_len: usize) -> usize {
    return std::cmp::min(max_len - len, 4);
}
