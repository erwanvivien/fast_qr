//! QRCodes need a way to define if they are readable, this is the point to
//! this scoring system. The lesser, the better

#![warn(missing_docs)]
use super::hardcode;

#[cfg(test)]
mod test;

const PATTERN_LEN: u32 = 11;
const PATTERN_LEN_USIZE: usize = 11;

#[cfg(test)]
pub fn test_score_line<const N: usize>(mat: &[bool; N]) -> u32 {
    score_line(mat).1
}

#[cfg(test)]
pub fn test_matrix_line<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let (line_score, _, _, _) = matrix_pattern_and_line(mat);
    line_score
}

#[cfg(test)]
pub fn test_matrix_pattern<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let (_, patt_score, _, _) = matrix_pattern_and_line(mat);
    patt_score
}

#[cfg(test)]
pub fn test_matrix_col<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let (_, _, col_score, _) = matrix_pattern_and_line(mat);
    col_score
}

#[cfg(test)]
pub fn test_matrix_dark_modules<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let (_, _, _, dark_score) = matrix_pattern_and_line(mat);
    dark_score
}

#[cfg(test)]
pub fn test_matrix_score_squares<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    matrix_score_squares(mat)
}

/// Computes scores for squares, any 2x2 square (black or white)
/// add 3 to the score
///
/// ### Opti:
/// We don't want to access the 4 squares each time, so we score the left most
/// ones and only fetch the next right ones
fn matrix_score_squares<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let mut square_score = 0;

    for i in 0..N - 1 {
        let mut buffer = 0u8;
        buffer |= (mat[i][0] as u8) << 2;
        buffer |= (mat[i + 1][0] as u8) << 3;
        for j in 0..N - 1 {
            buffer |= (mat[i][j + 1] as u8) << 2;
            buffer |= (mat[i + 1][j + 1] as u8) << 3;

            if buffer == 0b1111 || buffer == 0b0000 {
                square_score += 3;
            }
        }
    }

    square_score
}

/// Computes scores for both patterns (`0b10111010000` or `0b00001011101`)`
///
/// ### Opti:
/// We convert the line to a u11 (supposedly) so comparing it to a pattern is
/// a simple comparaison.
pub fn score_line<const N: usize>(line: &[bool; N]) -> (u32, u32) {
    let mut line_score = 0;
    let mut patt_score = 0;
    let mut buffer = 0u16;

    for (i, &item) in line.iter().enumerate().take(PATTERN_LEN_USIZE) {
        buffer |= (item as u16) << i;
    }

    let mut current_color = (buffer & 1) ^ 1;
    for &item in line.iter().take(N).skip(PATTERN_LEN_USIZE) {
        if buffer == 0b10111010000 || buffer == 0b00001011101 {
            patt_score += 40;
        }
        if buffer & 1 != current_color {
            let tmp = hardcode::trailing(buffer & 0b11111111111, PATTERN_LEN);
            line_score += tmp;
            if tmp != 1 {
                current_color = buffer & 1;
            }
        }

        buffer >>= 1;
        buffer |= (item as u16) << (PATTERN_LEN - 1);
    }

    if buffer == 0b10111010000 || buffer == 0b00001011101 {
        patt_score += 40;
    }

    let mut i = 0;
    if buffer == 0b11111111111 || buffer == 0b00000000000 {
        current_color = (buffer & 1) ^ 1;
        line_score += 1;
        i = 1;
        buffer >>= 1;
    }

    for i in i..PATTERN_LEN - 5 {
        if buffer & 1 != current_color {
            line_score += hardcode::trailing(buffer & 0b11111111111, PATTERN_LEN - i);
            current_color = buffer & 1;
        }
        buffer >>= 1;
    }

    (patt_score, line_score)
}

/// Converts the matrix to lines & columns and feed it to `score_line`
///
/// ### Opti:
/// While parsing the whole matrix (converting to col) we also count the
/// number of dark_modules.
fn matrix_pattern_and_line<const N: usize>(mat: &[[bool; N]; N]) -> (u32, u32, u32, u32) {
    let mut line_score = 0;
    let mut col_score = 0;
    let mut patt_score = 0;

    let mut dark_modules = 0usize;

    let mut mat_col = [[false; N]; N];

    for (i, item) in mat.iter().enumerate() {
        for (j, &item) in item.iter().enumerate() {
            mat_col[j][i] = item;
            dark_modules += item as usize;
        }
    }

    for i in 0..N {
        let l = score_line(&mat[i]);
        line_score += l.1;

        let c = score_line(&mat_col[i]);
        col_score += c.1;

        patt_score += l.0 + c.0;
    }

    let percent = (dark_modules * 100) / (N * N);
    let dark_score = hardcode::PERCENT_SCORE[percent as usize];

    (line_score, patt_score, col_score, dark_score)
}

/// Adds every score together
pub fn matrix_score<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let square_score = matrix_score_squares(mat);
    let (line_score, patt_score, col_score, dark_score) = matrix_pattern_and_line(mat);

    line_score + patt_score + col_score + dark_score + square_score
}
