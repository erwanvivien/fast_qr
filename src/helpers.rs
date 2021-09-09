//! Matrix helpers functions

/// Used to print a ` `
const EMPTY: &str = "\x1b[1;47m  ";
/// Used to print a `█`
const BLOCK: &str = "\x1b[0;30m██";

/// Prints a matrix
pub fn _print_matrix(mat: &Vec<Vec<bool>>) {
    for line in mat {
        for &cell in line {
            if cell == true {
                print!("{0}{0}", BLOCK);
            } else {
                print!("{0}{0}", EMPTY);
            }
        }
        println!();
    }
}

/// Prints a matrix with margins
pub fn print_matrix_with_margin(mat: &Vec<Vec<bool>>) {
    for _ in 0..2 {
        println!();
    }

    for line in mat {
        print!("\x1b[0m>    ");
        for &cell in line {
            if cell == true {
                print!("{0}", BLOCK);
            } else {
                print!("{0}", EMPTY);
            }
        }
        println!("\x1b[0m    <");
    }

    for _ in 0..2 {
        println!();
    }
}

/**
 * Convert a string that represents binary (i.e. "01100101") to it's equivalent in decimal value
 *
 * Mut be multiple of 8 bytes.
 *
 * Example: "01100101" => { 101 }
 */
pub fn binarystring_to_binary(alnum: &str) -> Vec<u8> {
    if alnum.len() % 8 != 0 {
        panic!(
            "`alnum` parameter should have a length multiple of 8. Current: {}",
            alnum.len()
        );
    }

    let alnum_string = String::from(alnum);
    let alnum_bytes = alnum_string.as_bytes();
    let mut vec = Vec::new();

    for i in (0..alnum.len()).step_by(8) {
        let mut tmp: u8 = 0;
        for j in 0..8 {
            let byte = alnum_bytes[i + j];
            if byte < 48 || byte > 49 {
                panic!("`alnum` parameter should not contains anything other than '0' or '1'. Current: {}", alnum);
            }

            tmp <<= 1;
            tmp += byte & 1 as u8;
        }
        vec.push(tmp);
    }

    return vec;
}

/**
 * Convert a vector of u8 to it's representation in bits
 *
 * Example: { 101 } => "01100101"
 */
pub fn binary_to_binarystring(binary: Vec<u8>) -> String {
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
    return result;
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

    for _ in 0..crate::vecl::MISSING_BITS[version] {
        result.push('0');
    }
    return result;
}
