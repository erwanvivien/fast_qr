//! Matrix helpers functions

use crate::vecl;

/// Used to print a ` `
const EMPTY: &str = "\x1b[1;47m  ";
/// Used to print a `█`
const BLOCK: &str = "\x1b[0;30m██";

/// Prints a matrix
pub fn _print_matrix(mat: &Vec<Vec<bool>>) {
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
pub fn print_matrix_with_margin(mat: &Vec<Vec<bool>>) {
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
pub fn binary_to_binarystring_version(binary: Vec<u8>, version: usize) -> String {
    let mut result: String = String::new();
    for nb in binary {
        for i in (0..8).rev() {
            if nb & (1 << i) == 0 {
                result.push('0');
            } else {
                result.push('1');
            }
        }
    }

    for _ in 0..vecl::MISSING_BITS[version] {
        result.push('0');
    }
    return result;
}
