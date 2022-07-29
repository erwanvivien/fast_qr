//! Creates the default empty QRCodes (no data)
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::datamasking::Mask;
use crate::module::Module;
use crate::version::Version;
use crate::{hardcode, QRCode, ECL};

/// Size of FIP (Finder Patterns)
const POSITION_SIZE: usize = 7;

pub fn transpose(qr: &QRCode) -> QRCode {
    let mut tranpose = qr.clone();

    for i in 0..qr.size {
        for j in i + 1..qr.size {
            tranpose[i][j] = qr[j][i];
            tranpose[j][i] = qr[i][j];
        }
    }

    tranpose
}

pub fn create_matrix(version: Version) -> QRCode {
    let size = version.size();
    let mut qr = QRCode::default(size);

    create_matrix_pattern(&mut qr);
    create_matrix_timing(&mut qr);
    create_matrix_dark_module(&mut qr);
    create_matrix_alignments(&mut qr, version);
    create_matrix_version_info(&mut qr, version);
    create_matrix_empty(&mut qr);

    let n: usize = qr.size;

    // Format information is not placed on the matrix yet
    // But we fill it anyway with garbage data to make it easier for placement
    {
        if (version as usize) < (Version::V01 as usize) {
            return qr;
        }

        for i in 0..=5 {
            // Top left
            qr[8][i] = Module::format(Module::LIGHT);
            qr[i][8] = Module::format(Module::LIGHT);

            // Top right
            qr[8][n - 1 - i] = Module::format(Module::LIGHT);

            // Bottom left
            qr[n - 1 - i][8] = Module::format(Module::LIGHT);
        }

        // Top left
        qr[8][7] = Module::format(Module::LIGHT);
        qr[8][8] = Module::format(Module::LIGHT);
        qr[7][8] = Module::format(Module::LIGHT);

        // Top right
        qr[8][n - 1 - 6] = Module::format(Module::LIGHT);
        qr[8][n - 1 - 7] = Module::format(Module::LIGHT);

        // Bottom left
        qr[n - 1 - 6][8] = Module::format(Module::LIGHT);
    }

    qr
}

/// Adds the 3 needed squares
pub fn create_matrix_pattern(qr: &mut QRCode) {
    let length = qr.size;
    let offsets = [
        (0, 0),
        (length - POSITION_SIZE, 0),
        (0, length - POSITION_SIZE),
    ];

    // Required pattern (4.1 Positions)
    for (y, x) in offsets {
        // Border
        for j in 0..=6 {
            qr[y][j + x] = Module::finder_pattern(Module::DARK);
            qr[6 + y][j + x] = Module::finder_pattern(Module::DARK);

            qr[j + y][x] = Module::finder_pattern(Module::DARK);
            qr[j + y][6 + x] = Module::finder_pattern(Module::DARK);
        }

        for j in 1..=5 {
            qr[y + 1][j + x] = Module::finder_pattern(Module::LIGHT);
            qr[5 + y][j + x] = Module::finder_pattern(Module::LIGHT);

            qr[j + y][x + 1] = Module::finder_pattern(Module::LIGHT);
            qr[j + y][5 + x] = Module::finder_pattern(Module::LIGHT);
        }

        for j in 2..=4 {
            qr[j + y][2 + x] = Module::finder_pattern(Module::DARK);
            qr[j + y][3 + x] = Module::finder_pattern(Module::DARK);
            qr[j + y][4 + x] = Module::finder_pattern(Module::DARK);
        }
    }
}

/// Adds the two lines of Timing patterns
pub fn create_matrix_timing(qr: &mut QRCode) {
    let length = qr.size;
    // Required pattern (4.3 Timing)
    for i in POSITION_SIZE + 1..length - POSITION_SIZE {
        let value = if (POSITION_SIZE + 1) % 2 == i % 2 {
            Module::DARK
        } else {
            Module::LIGHT
        };

        qr[POSITION_SIZE - 1][i] = Module::timing(value);
        qr[i][POSITION_SIZE - 1] = Module::timing(value);
    }
}

/// Adds the forever present pixel
pub fn create_matrix_dark_module(qr: &mut QRCode) {
    // Dark module
    let n: usize = qr.size;
    qr[n - 8][8] = Module::dark(Module::DARK);
}

/// Adds the smaller squares if needed
pub fn create_matrix_alignments(qr: &mut QRCode, version: Version) {
    if let Version::V01 = version {
        return;
    }

    // Alignments (smaller cubes)
    let alignment_patterns = version.alignment_patterns_grid();
    let max = alignment_patterns.len() - 1;

    for (i, &alignment_y) in alignment_patterns.iter().enumerate() {
        for (j, &alignment_x) in alignment_patterns.iter().enumerate() {
            if i == 0 && (j == max || j == 0) || (i == max && j == 0) {
                continue;
            }

            let y = alignment_y - 2;
            let x = alignment_x - 2;

            for offset in 0..=4 {
                qr[y][x + offset] = Module::alignment(Module::DARK);
                qr[y + 4][x + offset] = Module::alignment(Module::DARK);

                qr[y + offset][x] = Module::alignment(Module::DARK);
                qr[y + offset][x + 4] = Module::alignment(Module::DARK);
            }

            let y = alignment_y - 1;
            let x = alignment_x - 1;

            for offset in 0..=2 {
                qr[y][x + offset] = Module::alignment(Module::LIGHT);
                qr[y + 2][x + offset] = Module::alignment(Module::LIGHT);

                qr[y + offset][x] = Module::alignment(Module::LIGHT);
                qr[y + offset][x + 2] = Module::alignment(Module::LIGHT);
            }

            qr[alignment_y][alignment_x] = Module::alignment(Module::DARK);
        }
    }
}

/// Adds the version information if needed
pub fn create_matrix_version_info(qr: &mut QRCode, version: Version) {
    if (version as usize) < (Version::V07 as usize) {
        return;
    }

    let version_info = version.information();

    let n: usize = qr.size;

    for i in 0..=2 {
        for j in 0..=5 {
            let shift_i = 2 - i;
            let shift_j = 5 - j;
            let shift: u32 = 1 << ((5 - shift_j) * 3 + (2 - shift_i));

            let value = (version_info & shift) != 0;
            qr[j][n - 11 + i] = Module::version(value);
            qr[n - 11 + i][j] = Module::version(value);
        }
    }
}

/// Adds the format information if needed
pub fn create_matrix_format_info(qr: &mut QRCode, quality: ECL, mask: Mask) {
    let format_info = hardcode::ecm_to_format_information(quality, mask);

    let n: usize = qr.size;

    for i in (0..=5).rev() {
        let shift = 1 << (i + 9);
        let value = (format_info & shift) != 0;
        qr[8][5 - i] = Module::format(value);
        qr[n - 6 + i][8] = Module::format(value);
    }

    for i in 0..=5 {
        let shift = 1 << i;
        let value = (format_info & shift) != 0;
        qr[i][8] = Module::format(value);
        qr[8][n - i - 1] = Module::format(value);
    }

    {
        let shift = 1 << 8;
        let value = (format_info & shift) != 0;
        // Six on left
        qr[8][7] = Module::format(value);
        // Six on bottom
        qr[n - 7][8] = Module::format(value);
    }
    {
        let shift = 1 << 7;
        let value = (format_info & shift) != 0;
        // Seven on left
        qr[8][8] = Module::format(value);
        // Seven on right
        qr[8][n - 8] = Module::format(value);
    }
    {
        let shift = 1 << 6;
        let value = (format_info & shift) != 0;
        // Height on left
        qr[7][8] = Module::format(value);
        // Height on right
        qr[8][n - 7] = Module::format(value);
    }
}

/// Adds the space between finder patterns and data
fn create_matrix_empty(qr: &mut QRCode) {
    let n: usize = qr.size;

    for i in 0..=7 {
        // Top left
        qr[i][7] = Module::empty(Module::LIGHT);
        qr[7][i] = Module::empty(Module::LIGHT);

        // Bottom left
        qr[n - 8 + i][7] = Module::empty(Module::LIGHT);
        qr[n - 8][i] = Module::empty(Module::LIGHT);

        // Top right
        qr[i][n - 8] = Module::empty(Module::LIGHT);
        qr[7][n - 8 + i] = Module::empty(Module::LIGHT);
    }
}

#[cfg(test)]
pub fn create_mat_from_bool<const N: usize>(bool_mat: &[[bool; N]; N]) -> QRCode {
    let mut mat = create_matrix(Version::from_n(N));

    for (i, row) in bool_mat.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            mat[i][j].set(value);
        }
    }

    mat
}
