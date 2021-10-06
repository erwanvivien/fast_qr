//! Matrix helpers functions
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::bitstring;
use crate::bitstring::BitString;
use crate::vecl;

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
    version: usize,
    quality: vecl::ECL,
) -> BitString<5430> {
    let databits = vecl::ecc_to_databits(quality, version);

    let error_codes = vecl::ecc_to_ect(quality, version);

    let [(g1_count, _), (g2_count, _)] = vecl::ecc_to_groups(quality, version);
    let groups_count_total = g1_count + g2_count;

    let max = (databits / 8) as usize + error_codes * groups_count_total;
    return BitString::from(binary, max + vecl::MISSING_BITS[version] as usize);
}
