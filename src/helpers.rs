//! Matrix helpers functions
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::bitstring::BitString;
use crate::vecl::ECL;
use crate::version;

/// Used to print a ` ` (space)
const EMPTY: char = ' ';
/// Used to print a `█`
const BLOCK: char = '█';
/// Used to print a `▀`
const TOP: char = '▀';
/// Used to print a `█`
const BOTTOM: char = '▄';

/// Helper to print two lines at the same time
fn print_line<const N: usize>(line1: &[bool; N], line2: &[bool; N]) {
    for i in 0..N {
        match (line1[i], line2[i]) {
            (true, true) => print!("{}", EMPTY),
            (true, false) => print!("{}", BOTTOM),
            (false, true) => print!("{}", TOP),
            (false, false) => print!("{}", BLOCK),
        }
    }
}

/// Prints a matrix with margins
pub fn print_matrix_with_margin<const N: usize>(mat: &[[bool; N]; N]) {
    print!("{}", BOTTOM);
    print_line(&[true; N], &[false; N]);
    println!("{}", BOTTOM);

    // Black background
    print!("\x1b[40m");
    for i in (0..N - 1).step_by(2) {
        print!("{}", BLOCK);
        print_line(&mat[i + 0], &mat[i + 1]);
        println!("{}", BLOCK);
    }

    print!("{}", BLOCK);
    print_line(&mat[N - 1], &[false; N]);
    println!("{}", BLOCK);

    // Resets colors
    print!("\x1b[0m");
}

/**
 * Convert a vector of u8 to it's representation in bits
 *
 * If bits are required by the QR code (see [Missing bits](https://www.thonky.com/qr-code-tutorial/structure-final-message#list-of-versions-and-required-remainder-bits))
 * It adds them.
 *
 * Example: { 101 } => "01100101"
 */
pub const fn binary_to_binarystring_version(
    binary: [u8; 5430],
    version: version::Version,
    _quality: ECL,
) -> BitString<5430> {
    let max = version.max_bytes() * 8;
    return BitString::from(binary, max + version.missing_bits());
}
