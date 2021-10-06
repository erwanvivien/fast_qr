//! Is used to compute ECC (Error Correction Coding)

#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::polynomials;
use crate::vecl;

// use parking_lot::const_mutex;
// use parking_lot::Mutex;
// use std::sync::atomic::{AtomicUsize, Ordering};

/// Used in the ring, convert a^x using LOG[x%255] to it's decimal Gallois-Field value
const LOG: [u8; 256] = [
    1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117,
    234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181,
    119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161,
    95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187,
    107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136,
    13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197,
    151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168,
    77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198,
    145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149,
    55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167,
    83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72,
    144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207,
    131, 27, 54, 108, 216, 173, 71, 142, 1,
];

/// Reverses a ring value, converts decimal value x using ANTILOG[x%255] to it's alpha power value
const ANTILOG: [u8; 256] = [
    175, 0, 1, 25, 2, 50, 26, 198, 3, 223, 51, 238, 27, 104, 199, 75, 4, 100, 224, 14, 52, 141,
    239, 129, 28, 193, 105, 248, 200, 8, 76, 113, 5, 138, 101, 47, 225, 36, 15, 33, 53, 147, 142,
    218, 240, 18, 130, 69, 29, 181, 194, 125, 106, 39, 249, 185, 201, 154, 9, 120, 77, 228, 114,
    166, 6, 191, 139, 98, 102, 221, 48, 253, 226, 152, 37, 179, 16, 145, 34, 136, 54, 208, 148,
    206, 143, 150, 219, 189, 241, 210, 19, 92, 131, 56, 70, 64, 30, 66, 182, 163, 195, 72, 126,
    110, 107, 58, 40, 84, 250, 133, 186, 61, 202, 94, 155, 159, 10, 21, 121, 43, 78, 212, 229, 172,
    115, 243, 167, 87, 7, 112, 192, 247, 140, 128, 99, 13, 103, 74, 222, 237, 49, 197, 254, 24,
    227, 165, 153, 119, 38, 184, 180, 124, 17, 68, 146, 217, 35, 32, 137, 46, 55, 63, 209, 91, 149,
    188, 207, 205, 144, 135, 151, 178, 220, 252, 190, 97, 242, 86, 211, 171, 20, 42, 93, 158, 132,
    60, 57, 83, 71, 109, 65, 162, 31, 45, 67, 216, 183, 123, 164, 118, 196, 23, 73, 236, 127, 12,
    111, 246, 108, 161, 59, 82, 41, 157, 85, 170, 251, 96, 134, 177, 187, 204, 62, 90, 203, 89, 95,
    176, 156, 169, 160, 81, 11, 245, 22, 235, 122, 117, 44, 215, 79, 174, 213, 233, 230, 231, 173,
    232, 116, 214, 244, 234, 168, 80, 88, 175,
];

/**
 * Return a string of human readable polynomial (ex: below)
 *
 * [0, 75, 249, 78, 6] => "α0x4 + α75x3 + α249x2 + α78x + α6"
 *
 * [Polynomial generator tool](https://www.thonky.com/qr-code-tutorial/generator-polynomial-tool)
*/
#[cfg(test)]
pub fn generated_to_string(poly: &[u8]) -> String {
    let mut s = String::new();
    let length = poly.len();

    for (i, item) in poly.iter().enumerate() {
        s.push_str(&format!(
            "α{}{}",
            item,
            &match length - i - 1 {
                0 => String::new(),
                1 => String::from("x + "),
                n => format!("x{} + ", n),
            },
        ));
    }

    return s;
}

/// Takes an array and divides it by the other in a Gallois Field (256)
/// ```
/// from: [ 32,  91,  11, 120, 209, 114, 220,  77,  67,  64, 236,
///         17, 236,  17, 236,  17] (integer)
/// by  :                          [  0, 251,  67,  46,  61, 118,
///         70,  64,  94,  32,  45] (alpha)
/// ```
///
/// `from` should be of length `from.len() + by.len()`, so we pad zeroes, like so:
/// ```
/// from: [ 32,  91,  11, 120, 209, 114, 220,  77,  67,  64, 236,
///         17, 236,  17, 236,  17,   0, ..height..,   0] (integer)
/// ```
///
/// Then the actual division takes place
/// We convert `from` from INTEGER to ALPHA
pub fn division(from: &[u8], by: &[u8]) -> Vec<u8> {
    let mut from_mut = from.to_vec();

    from_mut.extend_from_slice(&vec![0; by.len() - 1]);

    let mut i = 0;

    while i < from.len() {
        // println!("{:?}", &from_mut[i..]);
        if from_mut[i] == 0 {
            i += 1;
            continue;
        }

        let alpha = ANTILOG[from_mut[i] as usize];

        let mut j = 0;

        while j < by.len() {
            let tmp = by[j] as usize + alpha as usize;
            from_mut[i + j] ^= LOG[tmp % 255];
            j += 1;
        }

        i += 1;
    }

    // println!("{:?}", &from_mut[from.len()..]);
    return from_mut[from.len()..].to_vec();
}

/// Uses the data and error(generator polynomail) to compute the divisions
/// for each block.
pub fn structure(data: &[u8], error: &[u8], quality: vecl::ECL, version: usize) -> Vec<u8> {
    let error_codes = vecl::ecc_to_ect(quality, version);

    let [(g1_count, g1_size), (g2_count, g2_size)] = vecl::ecc_to_groups(quality, version);
    let groups_count_total = g1_count + g2_count;

    let mut interleaved_data: Vec<u8> = Vec::new();
    let mut interleaved_error: Vec<u8> = vec![0; error_codes * groups_count_total];

    // println!("{}\n", error_codes);
    for i in 0..g1_count {
        let start_idx = i * g1_size;
        let division = polynomials::division(&data[start_idx..start_idx + g1_size], &error);
        // println!("{:?}", &data[start_idx..start_idx + g1_size]);
        // println!("{:?}\n", &division);
        for j in 0..division.len() {
            interleaved_error[j * groups_count_total + i] = division[j];
        }
    }
    for i in 0..g2_count {
        let start_idx = g1_size * g1_count + i * g2_size;
        let division = polynomials::division(&data[start_idx..start_idx + g2_size], &error);
        // println!("{:?}", &data[start_idx..start_idx + g2_size]);
        // println!("{:?}\n", &division);

        for j in 0..division.len() {
            interleaved_error[j * groups_count_total + i + g1_count] = division[j];
        }
    }

    for i in 0..std::cmp::max(g1_size, g2_size) {
        if i < g1_size {
            for j in 0..g1_count {
                let idx = j * g1_size + i;
                interleaved_data.push(data[idx]);
            }
        }
        if i < g2_size {
            for j in 0..g2_count {
                let idx = j * g2_size + i + g1_size * g1_count;
                interleaved_data.push(data[idx]);
            }
        }
    }

    interleaved_data.append(&mut interleaved_error);
    return interleaved_data;
}
