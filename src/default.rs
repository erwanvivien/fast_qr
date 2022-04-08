//! Creates the default empty QRCodes (no data)
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::version::Version;

/// Size of FIP (Finder Patterns)
const POSITION_SIZE: usize = 7;

/// Adds the 3 needed squares
pub fn create_matrix_pattern<const N: usize>(mat: &mut [[bool; N]; N]) {
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
            mat[y][j + x] = true;
            mat[6 + y][j + x] = true;

            mat[j + y][x] = true;
            mat[j + y][6 + x] = true;
        }

        for j in 2..=4 {
            mat[j + y][2 + x] = true;
            mat[j + y][3 + x] = true;
            mat[j + y][4 + x] = true;
        }
    }
}

/// Adds the two lines of Timing patterns
pub fn create_matrix_timing<const N: usize>(mat: &mut [[bool; N]; N]) {
    let length = mat.len();
    // Required pattern (4.3 Timing)
    for i in POSITION_SIZE + 1..length - POSITION_SIZE {
        mat[POSITION_SIZE - 1][i] = true;
        mat[i][POSITION_SIZE - 1] = true;
    }
}

/// Adds the forever present pixel
pub fn create_matrix_black_module<const N: usize>(mat: &mut [[bool; N]; N]) {
    // https://www.thonky.com/qr-code-tutorial/format-version-information
    // Dark module
    mat[N - 8][8] = true;
}

/// Adds the smaller squares if needed
pub fn create_matrix_alignments<const N: usize>(mat: &mut [[bool; N]; N], version: Version) {
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

            mat[alignment_y][alignment_x] = true;

            mat[alignment_y - 2][alignment_x - 2] = true;
            mat[alignment_y - 2][alignment_x - 1] = true;
            mat[alignment_y - 2][alignment_x] = true;
            mat[alignment_y - 2][alignment_x + 1] = true;
            mat[alignment_y - 2][alignment_x + 2] = true;
            mat[alignment_y - 1][alignment_x - 2] = true;
            mat[alignment_y - 1][alignment_x + 2] = true;
            mat[alignment_y][alignment_x - 2] = true;
            mat[alignment_y][alignment_x + 2] = true;
            mat[alignment_y + 1][alignment_x - 2] = true;
            mat[alignment_y + 1][alignment_x + 2] = true;
            mat[alignment_y + 2][alignment_x - 2] = true;
            mat[alignment_y + 2][alignment_x - 1] = true;
            mat[alignment_y + 2][alignment_x] = true;
            mat[alignment_y + 2][alignment_x + 1] = true;
            mat[alignment_y + 2][alignment_x + 2] = true;
        }
    }
}

/// Returns a version where alignments, timer & all are full blocks/lines
/// instead of square in squares
pub fn non_available_matrix_from_version<const N: usize>(version: Version) -> [[bool; N]; N] {
    let length = N;
    let mut mat = [[false; N]; N];

    let (y, x) = (0, 0);
    for i in 0..=8 {
        for j in 0..=8 {
            mat[j + y][i + x] = true;
        }
    }

    let (y, x) = (length - POSITION_SIZE - 1, 0);
    for i in 0..=7 {
        for j in 0..=8 {
            mat[i + y][j + x] = true;
        }
    }

    let (y, x) = (0, length - POSITION_SIZE - 1);
    for i in 0..=8 {
        for j in 0..=7 {
            mat[i + y][j + x] = true;
        }
    }

    for i in POSITION_SIZE + 1..length - POSITION_SIZE {
        mat[POSITION_SIZE - 1][i] = true;
        mat[i][POSITION_SIZE - 1] = true;
    }

    mat[N - 8][8] = true;

    // Alignments (smaller cubes)
    if let Version::V01 = version {
        return mat;
    }

    let alignment_patterns = version.alignment_patterns_grid();
    let max = alignment_patterns.len() - 1;

    for (i, &alignment_y) in alignment_patterns.iter().enumerate() {
        for (j, &alignment_x) in alignment_patterns.iter().enumerate() {
            if i == 0 && (j == max || j == 0) || (i == max && j == 0) {
                continue;
            }

            mat[alignment_y - 2][alignment_x - 2] = true;
            mat[alignment_y - 2][alignment_x - 1] = true;
            mat[alignment_y - 2][alignment_x] = true;
            mat[alignment_y - 2][alignment_x + 1] = true;
            mat[alignment_y - 2][alignment_x + 2] = true;

            mat[alignment_y - 1][alignment_x - 2] = true;
            mat[alignment_y - 1][alignment_x - 1] = true;
            mat[alignment_y - 1][alignment_x] = true;
            mat[alignment_y - 1][alignment_x + 1] = true;
            mat[alignment_y - 1][alignment_x + 2] = true;

            mat[alignment_y][alignment_x - 2] = true;
            mat[alignment_y][alignment_x - 1] = true;
            mat[alignment_y][alignment_x] = true;
            mat[alignment_y][alignment_x + 1] = true;
            mat[alignment_y][alignment_x + 2] = true;

            mat[alignment_y + 1][alignment_x - 2] = true;
            mat[alignment_y + 1][alignment_x - 1] = true;
            mat[alignment_y + 1][alignment_x] = true;
            mat[alignment_y + 1][alignment_x + 1] = true;
            mat[alignment_y + 1][alignment_x + 2] = true;

            mat[alignment_y + 2][alignment_x - 2] = true;
            mat[alignment_y + 2][alignment_x - 1] = true;
            mat[alignment_y + 2][alignment_x] = true;
            mat[alignment_y + 2][alignment_x + 1] = true;
            mat[alignment_y + 2][alignment_x + 2] = true;
        }
    }

    if (version as usize) < (Version::V07 as usize) {
        return mat;
    }

    for i in 0..=2 {
        for j in 0..=5 {
            mat[j][length - 11 + i] = true;
            mat[length - 11 + i][j] = true;
        }
    }

    mat
}
