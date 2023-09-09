//! Matrix helpers functions
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::module::Module;
use crate::QRCode;

/// Used to print a ` ` (space)
const EMPTY: char = ' ';
/// Used to print a `█`
const BLOCK: char = '█';
/// Used to print a `▀`
const TOP: char = '▀';
/// Used to print a `▄`
const BOTTOM: char = '▄';

/// Helper to print two lines at the same time
fn print_line(line1: &[Module], line2: &[Module], size: usize) -> String {
    let mut line = String::with_capacity(size);
    for i in 0..size {
        match (line1[i].value(), line2[i].value()) {
            (true, true) => line.push(EMPTY),
            (true, false) => line.push(BOTTOM),
            (false, true) => line.push(TOP),
            (false, false) => line.push(BLOCK),
        }
    }
    line
}

/// Prints a matrix with margins
pub fn print_matrix_with_margin(qr: &QRCode) -> String {
    let mut out = String::new();

    let line = print_line(
        &[Module::empty(true); 177],
        &[Module::empty(false); 177],
        qr.size,
    );

    out.push(BOTTOM);
    out.push_str(&line);
    out.push_str(&format!("{BOTTOM}\n"));

    // Black background
    for i in (0..qr.size - 1).step_by(2) {
        let line = print_line(&qr[i], &qr[i + 1], qr.size);
        out.push(BLOCK);
        out.push_str(&line);
        out.push_str(&format!("{BLOCK}\n"));
    }

    let line = print_line(&qr[qr.size - 1], &[Module::empty(false); 177], qr.size);
    out.push(BLOCK);
    out.push_str(&line);
    out.push(BLOCK);

    out
}

#[cfg(test)]
use crate::{compact::CompactQR, Version};

/// Convert a vector of u8 to it's representation in bits
///
/// If bits are required by the QR code (referring to 8.6 of the spec), they are added to the end of the vector.
///
/// ## Example
/// { 101 } => "01100101"
#[cfg(test)]
pub fn binary_to_binarystring_version(binary: [u8; 5430], version: Version) -> CompactQR {
    let max = version.max_bytes() * 8;
    CompactQR::from_array(&binary, max + version.missing_bits())
}
