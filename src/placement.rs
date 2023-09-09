//! Places data on a matrix
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::compact::CompactQR;
use crate::datamasking::Mask;
use crate::encode::Mode;

use crate::module::ModuleType;
use crate::{datamasking, default, encode, polynomials, score, QRCode};
use crate::{Version, ECL};
use core::iter::Rev;
use core::ops::Range;

pub enum BiRange {
    Forward(Range<usize>),
    Backwards(Rev<Range<usize>>),
}

impl Iterator for BiRange {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self {
            BiRange::Forward(range) => range.next(),
            BiRange::Backwards(range) => range.next(),
        }
    }
}

#[cfg(test)]
pub fn test_place_on_matrix_data(qr: &mut QRCode, structure_as_binarystring: &CompactQR) {
    place_on_matrix_data(qr, structure_as_binarystring);
}

/// Places the data on the matrix
pub fn place_on_matrix_data(qr: &mut QRCode, structure_as_binarystring: &CompactQR) {
    let structure_bytes_tmp = structure_as_binarystring.get_data();

    let mut rev = true;
    let mut idx = 0;

    // 0, 2, 4, 7, 9, .., N (skipping 6)
    for x in (0..6).chain(7..qr.size).rev().step_by(2) {
        let y_range = if rev {
            BiRange::Backwards((0..qr.size).rev())
        } else {
            BiRange::Forward(0..qr.size)
        };

        for y in y_range {
            if qr[y][x].module_type() == ModuleType::Data {
                let c = structure_bytes_tmp[idx / 8] & (1 << (7 - idx % 8));
                idx += 1;
                qr[y][x].set(c != 0);
            }
            if qr[y][x - 1].module_type() == ModuleType::Data {
                let c = structure_bytes_tmp[idx / 8] & (1 << (7 - idx % 8));
                idx += 1;
                qr[y][x - 1].set(c != 0);
            }
        }

        rev = !rev;
    }

    #[cfg(debug_assertions)]
    {
        let version = Version::from_n(qr.size);
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

/// Main function to place everything in the `QRCode`, returns a valid matrix
pub fn place_on_matrix(
    structure_as_binarystring: &CompactQR,
    quality: ECL,
    version: Version,
    mask: &mut Option<Mask>,
) -> QRCode {
    let mut best_score = u32::MAX;
    let mut best_mask = MASKS[0];

    let mut qr = default::create_matrix(version);
    place_on_matrix_data(&mut qr, structure_as_binarystring);

    let transpose = default::transpose(&qr);

    for mask in MASKS {
        let mut copy = qr.clone();
        let copy_transpose = transpose.clone();

        datamasking::mask(&mut copy, mask);
        let matrix_score = score::score(&copy, &copy_transpose);
        if matrix_score < best_score {
            best_score = matrix_score;
            best_mask = mask;
        }
    }

    best_mask = mask.unwrap_or(best_mask);
    *mask = Some(best_mask);

    default::create_matrix_format_info(&mut qr, quality, best_mask);
    datamasking::mask(&mut qr, best_mask);

    qr.mask = *mask;
    qr
}

/// Generate the whole matrix
pub fn create_matrix(
    input: &[u8],
    ecl: ECL,
    mode: Mode,
    version: Version,
    mask: &mut Option<Mask>,
) -> QRCode {
    let data_codewords = encode::encode(input, ecl, mode, version);
    let structure = polynomials::structure(data_codewords.get_data(), ecl, version);

    let max = version.max_bytes() * 8;
    let structure_binstring = CompactQR::from_array(&structure, max + version.missing_bits());

    QRCode {
        mode: Some(mode),
        ecl: Some(ecl),
        version: Some(version),
        ..place_on_matrix(&structure_binstring, ecl, version, mask)
    }
}
