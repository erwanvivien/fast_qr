//! Creates the default empty QRCodes (no data)
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::module::{Matrix, Module};
use crate::version::Version;
use crate::{hardcode, ECL};

/// Size of FIP (Finder Patterns)
const POSITION_SIZE: usize = 7;

pub fn create_matrix<const N: usize>() -> Matrix<N> {
    let version: Version = Version::from_n::<N>();

    let default_cell = Module::data(Module::LIGHT);
    let mut mat = [[default_cell; N]; N];

    create_matrix_pattern(&mut mat);
    create_matrix_timing(&mut mat);
    create_matrix_dark_module(&mut mat);
    create_matrix_alignments(&mut mat, version);
    create_matrix_version_info(&mut mat, version);
    create_matrix_empty(&mut mat);

    // Format information is not placed on the matrix yet
    // But we fill it anyway with garbage data to make it easier for placement
    {
        if (version as usize) < (Version::V01 as usize) {
            return mat;
        }

        for i in 0..=5 {
            // Top left
            mat[8][i] = Module::format(Module::LIGHT);
            mat[i][8] = Module::format(Module::LIGHT);

            // Top right
            mat[8][N - 1 - i] = Module::format(Module::LIGHT);

            // Bottom left
            mat[N - 1 - i][8] = Module::format(Module::LIGHT);
        }

        // Top left
        mat[8][7] = Module::format(Module::LIGHT);
        mat[8][8] = Module::format(Module::LIGHT);
        mat[7][8] = Module::format(Module::LIGHT);

        // Top right
        mat[8][N - 1 - 6] = Module::format(Module::LIGHT);
        mat[8][N - 1 - 7] = Module::format(Module::LIGHT);

        // Bottom left
        mat[N - 1 - 6][8] = Module::format(Module::LIGHT);
    }

    mat
}

/// Adds the 3 needed squares
pub fn create_matrix_pattern<const N: usize>(mat: &mut Matrix<N>) {
    let length = mat.len();
    let offsets = [
        (0, 0),
        (length - POSITION_SIZE, 0),
        (0, length - POSITION_SIZE),
    ];

    // Required pattern (4.1 Positions)
    for (y, x) in offsets {
        // Border
        for j in 0..=6 {
            mat[y][j + x] = Module::finder_pattern(Module::DARK);
            mat[6 + y][j + x] = Module::finder_pattern(Module::DARK);

            mat[j + y][x] = Module::finder_pattern(Module::DARK);
            mat[j + y][6 + x] = Module::finder_pattern(Module::DARK);
        }

        for j in 1..=5 {
            mat[y + 1][j + x] = Module::finder_pattern(Module::LIGHT);
            mat[5 + y][j + x] = Module::finder_pattern(Module::LIGHT);

            mat[j + y][x + 1] = Module::finder_pattern(Module::LIGHT);
            mat[j + y][5 + x] = Module::finder_pattern(Module::LIGHT);
        }

        for j in 2..=4 {
            mat[j + y][2 + x] = Module::finder_pattern(Module::DARK);
            mat[j + y][3 + x] = Module::finder_pattern(Module::DARK);
            mat[j + y][4 + x] = Module::finder_pattern(Module::DARK);
        }
    }
}

/// Adds the two lines of Timing patterns
pub fn create_matrix_timing<const N: usize>(mat: &mut Matrix<N>) {
    let length = mat.len();
    // Required pattern (4.3 Timing)
    for i in POSITION_SIZE + 1..length - POSITION_SIZE {
        let value = if (POSITION_SIZE + 1) % 2 == i % 2 {
            Module::DARK
        } else {
            Module::LIGHT
        };

        mat[POSITION_SIZE - 1][i] = Module::timing(value);
        mat[i][POSITION_SIZE - 1] = Module::timing(value);
    }
}

/// Adds the forever present pixel
pub fn create_matrix_dark_module<const N: usize>(mat: &mut Matrix<N>) {
    // https://www.thonky.com/qr-code-tutorial/format-version-information
    // Dark module
    mat[N - 8][8] = Module::dark(Module::DARK);
}

/// Adds the smaller squares if needed
pub fn create_matrix_alignments<const N: usize>(mat: &mut Matrix<N>, version: Version) {
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
                mat[y][x + offset] = Module::alignment(Module::DARK);
                mat[y + 4][x + offset] = Module::alignment(Module::DARK);

                mat[y + offset][x] = Module::alignment(Module::DARK);
                mat[y + offset][x + 4] = Module::alignment(Module::DARK);
            }

            let y = alignment_y - 1;
            let x = alignment_x - 1;

            for offset in 0..=2 {
                mat[y][x + offset] = Module::alignment(Module::LIGHT);
                mat[y + 2][x + offset] = Module::alignment(Module::LIGHT);

                mat[y + offset][x] = Module::alignment(Module::LIGHT);
                mat[y + offset][x + 2] = Module::alignment(Module::LIGHT);
            }

            mat[alignment_y][alignment_x] = Module::alignment(Module::DARK);
        }
    }
}

/// Adds the version information if needed
pub fn create_matrix_version_info<const N: usize>(mat: &mut Matrix<N>, version: Version) {
    if (version as usize) < (Version::V07 as usize) {
        return;
    }

    let version_info = version.information();

    for i in 0..=2 {
        for j in 0..=5 {
            let shift_i = 2 - i;
            let shift_j = 5 - j;
            let shift: u32 = 1 << ((5 - shift_j) * 3 + (2 - shift_i));

            let value = (version_info & shift) != 0;
            mat[j][N - 11 + i] = Module::version(value);
            mat[N - 11 + i][j] = Module::version(value);
        }
    }
}

/// Adds the format information if needed
pub fn create_matrix_format_info<const N: usize>(
    mat: &mut Matrix<N>,
    quality: ECL,
    mask_nb: usize,
) {
    let version = Version::from_n::<N>();
    if (version as usize) < (Version::V01 as usize) {
        return;
    }

    let format_info = hardcode::ecm_to_format_information(quality, mask_nb);

    for i in (0..=5).rev() {
        let shift = 1 << (i + 9);
        let value = (format_info & shift) != 0;
        mat[8][5 - i] = Module::format(value);
        mat[N - 6 + i][8] = Module::format(value);
    }

    for i in 0..=5 {
        let shift = 1 << i;
        let value = (format_info & shift) != 0;
        mat[i][8] = Module::format(value);
        mat[8][N - i - 1] = Module::format(value);
    }

    {
        let shift = 1 << 8;
        let value = (format_info & shift) != 0;
        // Six on left
        mat[8][7] = Module::format(value);
        // Six on bottom
        mat[N - 7][8] = Module::format(value);
    }
    {
        let shift = 1 << 7;
        let value = (format_info & shift) != 0;
        // Seven on left
        mat[8][8] = Module::format(value);
        // Seven on right
        mat[8][N - 8] = Module::format(value);
    }
    {
        let shift = 1 << 6;
        let value = (format_info & shift) != 0;
        // Height on left
        mat[7][8] = Module::format(value);
        // Height on right
        mat[8][N - 7] = Module::format(value);
    }
}

/// Adds the space between finder patterns and data
fn create_matrix_empty<const N: usize>(mat: &mut Matrix<N>) {
    for i in 0..=7 {
        // Top left
        mat[i][7] = Module::empty(Module::LIGHT);
        mat[7][i] = Module::empty(Module::LIGHT);

        // Bottom left
        mat[N - 8 + i][7] = Module::empty(Module::LIGHT);
        mat[N - 8][i] = Module::empty(Module::LIGHT);

        // Top right
        mat[i][N - 8] = Module::empty(Module::LIGHT);
        mat[7][N - 8 + i] = Module::empty(Module::LIGHT);
    }
}

#[cfg(test)]
pub fn create_mat_from_bool<const N: usize>(bool_mat: &[[bool; N]; N]) -> [[Module; N]; N] {
    let mut mat = create_matrix();

    for (i, row) in bool_mat.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            mat[i][j].set(value);
        }
    }

    mat
}
