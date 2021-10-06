//! Creates the default empty QRCodes (no data)
#![deny(unsafe_code)]
#![warn(missing_docs)]

/// Size of FIP (Finder Patterns)
const POSITION_SIZE: usize = 7;

/// For each version, it's where the alignments are placed
const ALIGNMENT_PATTERNS_GRID: [&'static [usize]; 41] = [
    &[],
    &[],
    &[6, 18],
    &[6, 22],
    &[6, 26],
    &[6, 30],
    &[6, 34],
    &[6, 22, 38],
    &[6, 24, 42],
    &[6, 26, 46],
    &[6, 28, 50],
    &[6, 30, 54],
    &[6, 32, 58],
    &[6, 34, 62],
    &[6, 26, 46, 66],
    &[6, 26, 48, 70],
    &[6, 26, 50, 74],
    &[6, 30, 54, 78],
    &[6, 30, 56, 82],
    &[6, 30, 58, 86],
    &[6, 34, 62, 90],
    &[6, 28, 50, 72, 94],
    &[6, 26, 50, 74, 98],
    &[6, 30, 54, 78, 102],
    &[6, 28, 54, 80, 106],
    &[6, 32, 58, 84, 110],
    &[6, 30, 58, 86, 114],
    &[6, 34, 62, 90, 118],
    &[6, 26, 50, 74, 98, 122],
    &[6, 30, 54, 78, 102, 126],
    &[6, 26, 52, 78, 104, 130],
    &[6, 30, 56, 82, 108, 134],
    &[6, 34, 60, 86, 112, 138],
    &[6, 30, 58, 86, 114, 142],
    &[6, 34, 62, 90, 118, 146],
    &[6, 30, 54, 78, 102, 126, 150],
    &[6, 24, 50, 76, 102, 128, 154],
    &[6, 28, 54, 80, 106, 132, 158],
    &[6, 32, 58, 84, 110, 136, 162],
    &[6, 26, 54, 82, 110, 138, 166],
    &[6, 30, 58, 86, 114, 142, 170],
];

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
            mat[0 + y][j + x] = true;
            mat[6 + y][j + x] = true;

            mat[j + y][0 + x] = true;
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

    return mat;
}

/// Adds the two lines of Timing patterns
pub fn create_matrix_timing<const N: usize>(mut mat: [[bool; N]; N]) -> [[bool; N]; N] {
    let length = mat.len();
    // Required pattern (4.3 Timing)
    let mut i = POSITION_SIZE + 1;
    let max = length - POSITION_SIZE;

    while i < max {
        mat[POSITION_SIZE - 1][i] = true;
        mat[i][POSITION_SIZE - 1] = true;

        i += 2;
    }

    return mat;
}

/// Adds the forever present pixel
pub const fn create_matrix_black_module<const N: usize>(
    mut mat: [[bool; N]; N],
    version: usize,
) -> [[bool; N]; N] {
    // https://www.thonky.com/qr-code-tutorial/format-version-information
    // Dark module
    mat[4 * version + 9][8] = true;
    return mat;
}

/// Adds the smaller squares if needed
pub const fn create_matrix_alignments<const N: usize>(
    mut mat: [[bool; N]; N],
    version: usize,
) -> [[bool; N]; N] {
    let alignment_patterns = ALIGNMENT_PATTERNS_GRID[version];
    // Alignments (smaller cubes)
    if version == 1 {
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

            mat[alignment_y][alignment_x] = true;

            mat[alignment_y - 2][alignment_x - 2] = true;
            mat[alignment_y - 2][alignment_x - 1] = true;
            mat[alignment_y - 2][alignment_x + 0] = true;
            mat[alignment_y - 2][alignment_x + 1] = true;
            mat[alignment_y - 2][alignment_x + 2] = true;
            mat[alignment_y - 1][alignment_x - 2] = true;
            mat[alignment_y - 1][alignment_x + 2] = true;
            mat[alignment_y + 0][alignment_x - 2] = true;
            mat[alignment_y + 0][alignment_x + 2] = true;
            mat[alignment_y + 1][alignment_x - 2] = true;
            mat[alignment_y + 1][alignment_x + 2] = true;
            mat[alignment_y + 2][alignment_x - 2] = true;
            mat[alignment_y + 2][alignment_x - 1] = true;
            mat[alignment_y + 2][alignment_x + 0] = true;
            mat[alignment_y + 2][alignment_x + 1] = true;
            mat[alignment_y + 2][alignment_x + 2] = true;

            j += 1;
        }
        i += 1;
    }

    return mat;
}

/// Returns a version where alignments, timer & all are full blocks/lines
/// instead of square in squares
pub const fn non_available_matrix_from_version<const N: usize>(version: usize) -> [[bool; N]; N] {
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

    mat[4 * version + 9][8] = true;

    let alignment_patterns = ALIGNMENT_PATTERNS_GRID[version];
    // Alignments (smaller cubes)
    if version == 1 {
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

            mat[alignment_y][alignment_x] = true;

            let mut x: i16 = -2;
            while x <= 2 {
                let mut y: i16 = -2;
                while y <= 2 {
                    mat[(alignment_y as i16 + y) as usize][(alignment_x as i16 + x) as usize] =
                        true;
                    y += 1;
                }
                x += 1;
            }
            mat[alignment_y - 2][alignment_x - 2] = true;

            j += 1;
        }
        i += 1;
    }

    if version < 7 {
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

    return mat;
}

// pub const DEFAULT_MATRIX: [&'static [bool]; 41] = [
//     &[],
//     &default_matrix::<441>(),
//     &default_matrix::<625>(),
//     &default_matrix::<841>(),
//     &default_matrix::<1089>(),
//     &default_matrix::<1369>(),
//     &default_matrix::<1681>(),
//     &default_matrix::<2025>(),
//     &default_matrix::<2401>(),
//     &default_matrix::<2809>(),
//     &default_matrix::<3249>(),
//     &default_matrix::<3721>(),
//     &default_matrix::<4225>(),
//     &default_matrix::<4761>(),
//     &default_matrix::<5329>(),
//     &default_matrix::<5929>(),
//     &default_matrix::<6561>(),
//     &default_matrix::<7225>(),
//     &default_matrix::<7921>(),
//     &default_matrix::<8649>(),
//     &default_matrix::<9409>(),
//     &default_matrix::<10201>(),
//     &default_matrix::<11025>(),
//     &default_matrix::<11881>(),
//     &default_matrix::<12769>(),
//     &default_matrix::<13689>(),
//     &default_matrix::<14641>(),
//     &default_matrix::<15625>(),
//     &default_matrix::<16641>(),
//     &default_matrix::<17689>(),
//     &default_matrix::<18769>(),
//     &default_matrix::<19881>(),
//     &default_matrix::<21025>(),
//     &default_matrix::<22201>(),
//     &default_matrix::<23409>(),
//     &default_matrix::<24649>(),
//     &default_matrix::<25921>(),
//     &default_matrix::<27225>(),
//     &default_matrix::<28561>(),
//     &default_matrix::<29929>(),
//     &default_matrix::<31329>(),
// ];

// pub const DEFAULT_FULL_MATRIX: [&'static [bool]; 41] = [
//     &[],
//     &default_full_matrix::<441>(),
//     &default_full_matrix::<625>(),
//     &default_full_matrix::<841>(),
//     &default_full_matrix::<1089>(),
//     &default_full_matrix::<1369>(),
//     &default_full_matrix::<1681>(),
//     &default_full_matrix::<2025>(),
//     &default_full_matrix::<2401>(),
//     &default_full_matrix::<2809>(),
//     &default_full_matrix::<3249>(),
//     &default_full_matrix::<3721>(),
//     &default_full_matrix::<4225>(),
//     &default_full_matrix::<4761>(),
//     &default_full_matrix::<5329>(),
//     &default_full_matrix::<5929>(),
//     &default_full_matrix::<6561>(),
//     &default_full_matrix::<7225>(),
//     &default_full_matrix::<7921>(),
//     &default_full_matrix::<8649>(),
//     &default_full_matrix::<9409>(),
//     &default_full_matrix::<10201>(),
//     &default_full_matrix::<11025>(),
//     &default_full_matrix::<11881>(),
//     &default_full_matrix::<12769>(),
//     &default_full_matrix::<13689>(),
//     &default_full_matrix::<14641>(),
//     &default_full_matrix::<15625>(),
//     &default_full_matrix::<16641>(),
//     &default_full_matrix::<17689>(),
//     &default_full_matrix::<18769>(),
//     &default_full_matrix::<19881>(),
//     &default_full_matrix::<21025>(),
//     &default_full_matrix::<22201>(),
//     &default_full_matrix::<23409>(),
//     &default_full_matrix::<24649>(),
//     &default_full_matrix::<25921>(),
//     &default_full_matrix::<27225>(),
//     &default_full_matrix::<28561>(),
//     &default_full_matrix::<29929>(),
//     &default_full_matrix::<31329>(),
// ];

// const fn array_len_to_width(array_len: usize) -> usize {
//     match array_len {
//         441 => 21,
//         625 => 25,
//         841 => 29,
//         1089 => 33,
//         1369 => 37,
//         1681 => 41,
//         2025 => 45,
//         2401 => 49,
//         2809 => 53,
//         3249 => 57,
//         3721 => 61,
//         4225 => 65,
//         4761 => 69,
//         5329 => 73,
//         5929 => 77,
//         6561 => 81,
//         7225 => 85,
//         7921 => 89,
//         8649 => 93,
//         9409 => 97,
//         10201 => 101,
//         11025 => 105,
//         11881 => 109,
//         12769 => 113,
//         13689 => 117,
//         14641 => 121,
//         15625 => 125,
//         16641 => 129,
//         17689 => 133,
//         18769 => 137,
//         19881 => 141,
//         21025 => 145,
//         22201 => 149,
//         23409 => 153,
//         24649 => 157,
//         25921 => 161,
//         27225 => 165,
//         28561 => 169,
//         29929 => 173,
//         31329 => 177,
//         _ => 0,
//     }
// }

// const fn default_matrix<const N: usize>() -> [bool; N] {
//     let mut mat = [false; N];
//     let length = array_len_to_width(N);
//     let version = (length - 17) / 4;

//     {
//         let offsets = [
//             (0, 0),
//             (length - POSITION_SIZE, 0),
//             (0, length - POSITION_SIZE),
//         ];

//         // Border
//         let mut i = 0;
//         while i < offsets.len() {
//             let (y, x) = offsets[i];
//             {
//                 let mut j = 0;
//                 while j <= 6 {
//                     mat[(0 + x) * (j + x)] = true;
//                     mat[(6 + x) * (j + x)] = true;
//                     mat[(j + x) * (0 + x)] = true;
//                     mat[(j + x) * (6 + x)] = true;

//                     j += 1;
//                 }
//             }
//             // Inside
//             {
//                 let mut j = 2;

//                 while j <= 5 {
//                     mat[(j + y) * (2 + x)] = true;
//                     mat[(j + y) * (3 + x)] = true;
//                     mat[(j + y) * (4 + x)] = true;

//                     j += 1;
//                 }
//             }
//             i += 1;
//         }
//     }

//     {
//         let mut i = POSITION_SIZE + 1;
//         while i < length - POSITION_SIZE {
//             mat[(POSITION_SIZE - 1) * i] = true;
//             mat[i * (POSITION_SIZE - 1)] = true;

//             i += 2;
//         }
//     }

//     {
//         mat[(4 * version + 9) * 8] = true;
//     }

//     if version != 1 {
//         let alignment_patterns = ALIGNMENT_PATTERNS_GRID[version];
//         // Alignments (smaller cubes)

//         let max = alignment_patterns.len() - 1;

//         let mut i = 0;
//         while i < alignment_patterns.len() {
//             let mut j = 0;
//             while j < alignment_patterns.len() {
//                 if (i == 0 && j == 0) || (i == max && j == 0) || (i == 0 && j == max) {
//                     j += 1;
//                     continue;
//                 }

//                 let alignment_y = alignment_patterns[i];
//                 let alignment_x = alignment_patterns[j];

//                 mat[alignment_y * alignment_x] = true;

//                 let mut x = -2;
//                 while x <= 2 {
//                     let mut y = -2;
//                     while y <= 2 {
//                         if x != -2 && x != 2 && y != -2 && y != 2 {
//                             y += 1;
//                             continue;
//                         }

//                         mat[((alignment_y as i16 + y) * (alignment_x as i16 + x)) as usize] = true;
//                         y += 1;
//                     }
//                     x += 1;
//                 }

//                 j += 1;
//             }
//             i += 1;
//         }
//     }

//     return mat;
// }

// const fn default_full_matrix<const N: usize>() -> [bool; N] {
//     let mut mat = [false; N];
//     let length = array_len_to_width(N);
//     let version = (length - 17) / 4;

//     {
//         let (y, x) = (0, 0);
//         let mut i = 0;
//         while i <= 8 {
//             let mut j = 0;
//             while j <= 8 {
//                 mat[(j + y) * (i + x)] = true;
//                 j += 1;
//             }
//             i += 1;
//         }
//     }
//     {
//         let (y, x) = (length - POSITION_SIZE - 1, 0);

//         let mut i = 0;
//         while i <= 7 {
//             let mut j = 0;
//             while j <= 8 {
//                 mat[(i + y) * (j + x)] = true;
//                 j += 1;
//             }
//             i += 1;
//         }
//     }

//     {
//         let (y, x) = (0, length - POSITION_SIZE - 1);
//         let mut i = 0;
//         while i <= 8 {
//             let mut j = 0;
//             while j <= 7 {
//                 mat[(i + y) * (j + x)] = true;
//                 j += 1;
//             }
//             i += 1;
//         }
//     }

//     {
//         let mut i = POSITION_SIZE + 1;
//         while i < length - POSITION_SIZE {
//             mat[(POSITION_SIZE - 1) * i] = true;
//             mat[i * (POSITION_SIZE - 1)] = true;
//             i += 1;
//         }
//     }
//     {
//         mat[(4 * version + 9) * 8] = true;
//     }

//     {
//         // Alignments (smaller cubes)
//         if version == 1 {
//             return mat;
//         }

//         let alignment_patterns = ALIGNMENT_PATTERNS_GRID[version];
//         // Alignments (smaller cubes)

//         let max = alignment_patterns.len() - 1;

//         let mut i = 0;
//         while i < alignment_patterns.len() {
//             let mut j = 0;
//             while j < alignment_patterns.len() {
//                 if (i == 0 && j == 0) || (i == max && j == 0) || (i == 0 && j == max) {
//                     j += 1;
//                     continue;
//                 }

//                 let alignment_y = alignment_patterns[i];
//                 let alignment_x = alignment_patterns[j];

//                 mat[alignment_y * alignment_x] = true;

//                 let mut x = -2;
//                 while x <= 2 {
//                     let mut y = -2;
//                     while y <= 2 {
//                         mat[((alignment_y as i16 + y) * (alignment_x as i16 + x)) as usize] = true;
//                         y += 1;
//                     }
//                     x += 1;
//                 }

//                 j += 1;
//             }
//             i += 1;
//         }
//     }

//     {
//         if version < 7 {
//             return mat;
//         }

//         let mut i = 0;
//         while i <= 2 {
//             let mut j = 0;
//             while j <= 5 {
//                 mat[j * (length - 11 + i)] = true;
//                 mat[(length - 11 + i) * j] = true;
//                 j += 1;
//             }
//             i += 1;
//         }
//     }

//     return mat;
// }
