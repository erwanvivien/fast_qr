//! Places data on a matrix
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::bitstring::BitString;
use crate::encode::Mode;
use crate::module::{Matrix, ModuleType};
use crate::{datamasking, default, encode, helpers, polynomials, score};
use crate::{Version, ECL};
use std::iter::Rev;
use std::ops::Range;

pub enum BiRange {
    Forward(Range<i32>),
    Backwards(Rev<Range<i32>>),
}

impl Iterator for BiRange {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        match self {
            BiRange::Forward(range) => range.next(),
            BiRange::Backwards(range) => range.next(),
        }
    }
}

#[cfg(test)]
pub fn test_place_on_matrix_data<const N: usize>(
    mat: &mut Matrix<N>,
    structure_as_binarystring: &BitString<5430>,
) {
    place_on_matrix_data(mat, structure_as_binarystring);
}

/// Places the data on the matrix
pub fn place_on_matrix_data<const N: usize>(
    mat: &mut Matrix<N>,
    structure_as_binarystring: &BitString<5430>,
) {
    let structure_bytes_tmp = structure_as_binarystring.get_data();

    let mut rev = true;
    let mut idx = 0;

    // 0, 2, 4, 7, 9, .., N (skipping 6)
    for x in (0..6).chain(7..N).rev().step_by(2) {
        let x = x as usize;

        let y_range = if rev {
            BiRange::Backwards((0..N as i32).rev())
        } else {
            BiRange::Forward(0..N as i32)
        };

        for y in y_range {
            let y = y as usize;

            if mat[y][x].module_type() == ModuleType::Data {
                let c = structure_bytes_tmp[idx / 8] & (1 << (7 - idx % 8));
                idx += 1;
                mat[y][x].set(c != 0);
            }
            if mat[y][x - 1].module_type() == ModuleType::Data {
                let c = structure_bytes_tmp[idx / 8] & (1 << (7 - idx % 8));
                idx += 1;
                mat[y][x - 1].set(c != 0);
            }
        }

        rev = !rev;
    }

    #[cfg(debug_assertions)]
    {
        let version = Version::from_matrix::<N>();
        assert_eq!(idx - version.missing_bits(), version.max_bytes() * 8);
    }
}

/// Main function to place everything in the QRCode, returns a valid matrix
pub fn place_on_matrix<const N: usize>(
    structure_as_binarystring: &BitString<5430>,
    version: Version,
    quality: ECL,
    mask: Option<usize>,
) -> Matrix<N> {
    let mut best_score = u32::MAX;
    let mut best_mask = usize::MAX;

    let mut mat = default::create_matrix(version);

    place_on_matrix_data(&mut mat, structure_as_binarystring);

    let mut mask_nb = 0usize;

    while mask.is_none() && mask_nb < 8 {
        let mut copy = mat;
        datamasking::mask(&mut copy, mask_nb);
        let matrix_score = score::matrix_score(&copy);
        if matrix_score < best_score {
            best_score = matrix_score;
            best_mask = mask_nb;
        }

        mask_nb += 1;
    }

    best_mask = mask.unwrap_or(best_mask);

    default::create_matrix_format_info(&mut mat, quality, best_mask);
    datamasking::mask(&mut mat, best_mask);

    mat
}

/// Generate the whole matrix
pub fn create_matrix<const N: usize>(
    input: &[u8],
    ecl: ECL,
    mode: Mode,
    version: Version,
    mask_nb: Option<usize>,
) -> Matrix<N> {
    let data_codewords = encode::encode(input, ecl, mode, version);
    let structure = polynomials::structure(&data_codewords.get_data(), ecl, version);
    let structure_binstring = helpers::binary_to_binarystring_version(structure, version, ecl);

    place_on_matrix(&structure_binstring, version, ecl, mask_nb)
}
