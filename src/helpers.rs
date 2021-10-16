//! Matrix helpers functions
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::bitstring::BitString;
use crate::vecl;
use crate::version;

/// Used to print a ` `
const EMPTY: &str = "\x1b[1;47m  ";
/// Used to print a `█`
const BLOCK: &str = "\x1b[0;30m██";

/// Prints a matrix
pub fn _print_matrix<const N: usize>(mat: &[[bool; N]; N]) {
    for line in mat {
        for &cell in line {
            if cell == true {
                print!("{0}", BLOCK);
            } else {
                print!("{0}", EMPTY);
            }
        }
        println!("\x1b[0m");
    }
}

/// Prints a matrix with margins
pub fn print_matrix_with_margin<const N: usize>(mat: &[[bool; N]; N]) {
    for _ in 0..4 {
        for _ in 0..mat.len() + 8 {
            print!("{}", EMPTY);
        }
        println!("\x1b[0m");
    }

    for line in mat {
        print!("\x1b[0m{0}{0}{0}{0}", EMPTY);
        for &cell in line {
            if cell == true {
                print!("{0}", BLOCK);
            } else {
                print!("{0}", EMPTY);
            }
        }
        println!("{0}{0}{0}{0}\x1b[0m", EMPTY);
    }

    for _ in 0..4 {
        for _ in 0..mat.len() + 8 {
            print!("{}", EMPTY);
        }
        println!("\x1b[0m");
    }
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
    _quality: vecl::ECL,
) -> BitString<5430> {
    let max = version.max_bytes() * 8;
    return BitString::from(binary, max + version.missing_bits());
}
