//! QRCodes need a way to define if they are readable, this is the point to
//! this scoring system. The lesser, the better

#![warn(missing_docs)]
use crate::hardcode;

#[allow(dead_code)]
#[cfg(test)]
pub fn test_score_line<const N: usize>(mat: &[bool; N]) -> u32 {
    return score_line(mat).1;
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_line<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let (line_score, _, _, _) = matrix_pattern_and_line(mat);
    return line_score;
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_pattern<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let (_, patt_score, _, _) = matrix_pattern_and_line(mat);
    return patt_score;
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_col<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let (_, _, col_score, _) = matrix_pattern_and_line(mat);
    return col_score;
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_dark_modules<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let (_, _, _, dark_score) = matrix_pattern_and_line(mat);
    return dark_score;
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_score_squares<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    return matrix_score_squares(mat);
}

/// Computes scores for squares, any 2x2 square (black or white)
/// add 3 to the score
///
/// ### Opti:
/// We don't want to access the 4 squares each time, so we score the left most
/// ones and only fetch the next right ones
const fn matrix_score_squares<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let mut square_score = 0;

    let mut i = 0;
    while i < N - 1 {
        let mut j = 0;

        let mut buffer = 0u8;
        buffer |= (mat[i + 0][j + 0] as u8) << 2;
        buffer |= (mat[i + 1][j + 0] as u8) << 3;

        while j < N - 1 {
            buffer >>= 2;

            buffer |= (mat[i + 0][j + 1] as u8) << 2;
            buffer |= (mat[i + 1][j + 1] as u8) << 3;

            if buffer == 0b1111 || buffer == 0b0000 {
                square_score += 3;
            }

            j += 1;
        }
        i += 1;
    }

    return square_score;
}

const PATTERN_LEN: u32 = 11;

/// Computes scores for consecutive modules of the same colors
/// if we have 5 or more modules, the score is `x - 2`
/// (x being the number of modules)
///
/// ### Opti:
/// We are using `u16::trailing_zeros` and `u16::trailing_ones` to
/// compute the consecutive squares
const fn score_trailing(buffer: u16, buffer_size: u32) -> u32 {
    let buffer = buffer & 0b11111111111;
    let mut trailing = hardcode::TRAILLING[buffer as usize];

    if trailing >= buffer_size && buffer_size == PATTERN_LEN {
        return 1;
    }

    if trailing >= buffer_size {
        trailing = buffer_size;
    }

    if trailing >= 5 {
        return trailing - 2;
    }

    return 0;
}

const fn score_trailing_11(buffer: u16) -> u32 {
    let buffer = buffer & 0b11111111111;
    let trailing = hardcode::TRAILLING[buffer as usize];

    if trailing >= PATTERN_LEN {
        return 1;
    }

    if trailing >= 5 {
        return trailing - 2;
    }

    return 0;
}

/// Computes scores for both patterns (`0b10111010000` or `0b00001011101`)`
///
/// ### Opti:
/// We convert the line to a u11 (supposedly) so comparing it to a pattern is
/// a simple comparaison.
pub const fn score_line<const N: usize>(line: &[bool; N]) -> (u32, u32) {
    let mut line_score = 0;
    let mut patt_score = 0;
    let mut buffer = 0u16;

    let mut i = 0usize;
    while i < PATTERN_LEN as usize {
        if line[i] {
            buffer |= 1 << i;
        }
        i += 1;
    }

    let mut current_color = ((buffer & 1) + 1) & 1;
    while i < N {
        if buffer == 0b10111010000 || buffer == 0b00001011101 {
            patt_score += 40;
        }
        if buffer & 1 != current_color {
            let tmp = score_trailing_11(buffer);
            line_score += tmp;
            if tmp != 1 {
                current_color = buffer & 1;
            }
        }

        buffer >>= 1;
        if line[i] {
            buffer |= 1 << (PATTERN_LEN - 1);
        }

        i += 1;
    }

    if buffer == 0b10111010000 || buffer == 0b00001011101 {
        patt_score += 40;
    }

    let mut i = 0;
    if buffer == 0b11111111111 || buffer == 0b00000000000 {
        current_color = ((buffer & 1) + 1) & 1;
        line_score += 1;
        i = 1;
        buffer >>= 1;
    }

    while i <= PATTERN_LEN - 5 {
        if buffer & 1 != current_color {
            line_score += score_trailing(buffer, PATTERN_LEN - i);
            current_color = buffer & 1;
        }
        buffer >>= 1;
        i += 1;
    }

    return (patt_score, line_score);
}

/// Converts the matrix to lines & columns and feed it to `score_line`
///
/// ### Opti:
/// While parsing the whole matrix (converting to col) we also count the
/// number of dark_modules.
const fn matrix_pattern_and_line<const N: usize>(mat: &[[bool; N]; N]) -> (u32, u32, u32, u32) {
    let mut buffer_col = [false; N];
    let mut line_score = 0;
    let mut col_score = 0;
    let mut patt_score = 0;

    let mut dark_modules = 0usize;

    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            let tmp = mat[j][i];
            buffer_col[j] = tmp;
            if tmp {
                dark_modules += 1;
            }

            j += 1;
        }

        let l = score_line(&mat[i]);
        line_score += l.1;

        let c = score_line(&buffer_col);
        col_score += c.1;

        patt_score += l.0 + c.0;

        i += 1;
    }

    let percent = (dark_modules * 100) / (N * N);
    let dark_score = hardcode::PERCENT_SCORE[percent as usize];

    return (line_score, patt_score, col_score, dark_score);
}

/// Adds every score together
pub const fn matrix_score<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let square_score = matrix_score_squares(mat);
    let (line_score, patt_score, col_score, dark_score) = matrix_pattern_and_line(mat);

    return line_score + patt_score + col_score + dark_score + square_score;
}
