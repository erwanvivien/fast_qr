//! Places data on a matrix
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::compact::CompactQR;
use crate::datamasking::Mask;
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
    structure_as_binarystring: &CompactQR<5430>,
) {
    place_on_matrix_data(mat, structure_as_binarystring);
}

/// Places the data on the matrix
pub fn place_on_matrix_data<const N: usize>(
    mat: &mut Matrix<N>,
    structure_as_binarystring: &CompactQR<5430>,
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
        let version = Version::from_n::<N>();
        assert_eq!(idx - version.missing_bits(), version.max_bytes() * 8);
    }
}

const MASKS: [Mask; 8] = [
    Mask::Checkerboard,
    Mask::HorizontalLines,
    Mask::VerticalLines,
    Mask::DiagonalLines,
    Mask::LargeCheckerboard,
    Mask::Fields,
    Mask::Diamonds,
    Mask::Meadow,
];

/// Main function to place everything in the QRCode, returns a valid matrix
pub fn place_on_matrix<const N: usize>(
    structure_as_binarystring: &CompactQR<5430>,
    quality: ECL,
    mask: Option<Mask>,
) -> Matrix<N> {
    let mut best_score = u32::MAX;
    let mut best_mask = MASKS[0];

    let mut mat = default::create_matrix();

    place_on_matrix_data(&mut mat, structure_as_binarystring);

    for mask in MASKS {
        let mut copy = mat;
        datamasking::mask(&mut copy, mask);
        let matrix_score = score::matrix_score(&copy);
        if matrix_score < best_score {
            best_score = matrix_score;
            best_mask = mask;
        }
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
    mask: Option<Mask>,
) -> Matrix<N> {
    let data_codewords = encode::encode(input, ecl, mode, version);
    let structure = polynomials::structure(&data_codewords.get_data(), ecl, version);
    let structure_binstring = helpers::binary_to_binarystring_version(structure, version);

    place_on_matrix(&structure_binstring, ecl, mask)
}
