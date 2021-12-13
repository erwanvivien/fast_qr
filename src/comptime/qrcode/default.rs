//! Creates the default empty QRCodes (no data)
#![deny(unsafe_code)]
#![warn(missing_docs)]

use super::Version;

/// Size of FIP (Finder Patterns)
const POSITION_SIZE: usize = 7;

/// Adds the 3 needed squares
pub const fn create_matrix_pattern<const N: usize>(mut mat: [[bool; N]; N]) -> [[bool; N]; N] {
    let length = mat.len();
    let offsets = [
        (0, 0),
        (length - POSITION_SIZE, 0),
        (0, length - POSITION_SIZE),
    ];

    // Required pattern (4.1 Positions)
    let mut i = 0;
    while i < offsets.len() {
        let off = offsets[i];
        let (y, x) = off;

        // Border
        let mut j = 0;
        while j <= 6 {
            mat[y][j + x] = true;
            mat[6 + y][j + x] = true;

            mat[j + y][x] = true;
            mat[j + y][6 + x] = true;

            j += 1;
        }

        // Inside
        let mut j = 2;
        while j <= 4 {
            mat[j + y][2 + x] = true;
            mat[j + y][3 + x] = true;
            mat[j + y][4 + x] = true;

            j += 1;
        }

        i += 1;
    }

    mat
}

/// Adds the two lines of Timing patterns
pub const fn create_matrix_timing<const N: usize>(mut mat: [[bool; N]; N]) -> [[bool; N]; N] {
    let length = mat.len();
    // Required pattern (4.3 Timing)
    let mut i = POSITION_SIZE + 1;
    let max = length - POSITION_SIZE;

    while i < max {
        mat[POSITION_SIZE - 1][i] = true;
        mat[i][POSITION_SIZE - 1] = true;

        i += 2;
    }

    mat
}

/// Adds the forever present pixel
pub const fn create_matrix_black_module<const N: usize>(mut mat: [[bool; N]; N]) -> [[bool; N]; N] {
    // https://www.thonky.com/qr-code-tutorial/format-version-information
    // Dark module
    mat[N - 8][8] = true;
    mat
}

/// Adds the smaller squares if needed
pub const fn create_matrix_alignments<const N: usize>(
    mut mat: [[bool; N]; N],
    version: Version,
) -> [[bool; N]; N] {
    if let Version::V01 = version {
        return mat;
    }

    // Alignments (smaller cubes)
    let alignment_patterns = version.alignment_patterns_grid();
    let max = alignment_patterns.len() - 1;

    let mut i = 0;
    while i < alignment_patterns.len() {
        let mut j = 0;
        while j < alignment_patterns.len() {
            if (i == 0 && j == 0) || (i == max && j == 0) || (i == 0 && j == max) {
                j += 1;
                continue;
            }

            let alignment_y = alignment_patterns[i];
            let alignment_x = alignment_patterns[j];

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

            j += 1;
        }
        i += 1;
    }

    mat
}

/// Returns a version where alignments, timer & all are full blocks/lines
/// instead of square in squares
pub const fn non_available_matrix_from_version<const N: usize>(version: Version) -> [[bool; N]; N] {
    let length = N;
    let mut mat = [[false; N]; N];

    let (y, x) = (0, 0);
    let mut i = 0;
    while i <= 8 {
        let mut j = 0;
        while j <= 8 {
            mat[j + y][i + x] = true;
            j += 1;
        }
        i += 1;
    }
    let (y, x) = (length - POSITION_SIZE - 1, 0);
    let mut i = 0;
    while i <= 7 {
        let mut j = 0;
        while j <= 8 {
            mat[i + y][j + x] = true;
            j += 1;
        }
        i += 1;
    }
    let (y, x) = (0, length - POSITION_SIZE - 1);
    let mut i = 0;
    while i <= 8 {
        let mut j = 0;
        while j <= 7 {
            mat[i + y][j + x] = true;
            j += 1;
        }
        i += 1;
    }

    let mut i = POSITION_SIZE + 1;
    let max = length - POSITION_SIZE;

    while i < max {
        mat[POSITION_SIZE - 1][i] = true;
        mat[i][POSITION_SIZE - 1] = true;

        i += 1;
    }

    mat[N - 8][8] = true;

    let alignment_patterns = version.alignment_patterns_grid();
    // Alignments (smaller cubes)
    if let Version::V01 = version {
        return mat;
    }

    let max = alignment_patterns.len() - 1;

    let mut i = 0;
    while i < alignment_patterns.len() {
        let mut j = 0;
        while j < alignment_patterns.len() {
            if (i == 0 && j == 0) || (i == max && j == 0) || (i == 0 && j == max) {
                j += 1;
                continue;
            }

            let alignment_y = alignment_patterns[i];
            let alignment_x = alignment_patterns[j];

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

            j += 1;
        }
        i += 1;
    }

    if (version as usize) < (Version::V07 as usize) {
        return mat;
    }

    let mut i = 0;
    while i <= 2 {
        let mut j = 0;
        while j <= 5 {
            mat[j][length - 11 + i] = true;
            mat[length - 11 + i][j] = true;

            j += 1;
        }
        i += 1;
    }

    mat
}
