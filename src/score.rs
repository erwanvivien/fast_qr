//! QRCodes need a way to define if they are readable, this is the point to
//! this scoring system. The lesser, the better

#![warn(missing_docs)]

#[cfg(test)]
use crate::default::transpose;
use crate::module::{Module, ModuleType};
use crate::QRCode;

use super::hardcode;

#[allow(dead_code)]
#[cfg(test)]
pub fn test_score_line(line: &[Module]) -> u32 {
    score_line(line).1
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_score_pattern(line: &[Module]) -> u32 {
    score_line(line).0
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_line(qr: &QRCode) -> u32 {
    let transpose = transpose(qr);
    let (line_score, _, _) = matrix_pattern_and_line(qr, &transpose);
    line_score
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_pattern(qr: &QRCode) -> u32 {
    let transpose = transpose(qr);
    let (_, _, patt_score) = matrix_pattern_and_line(qr, &transpose);
    patt_score
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_col(qr: &QRCode) -> u32 {
    let transpose = transpose(qr);

    let (_, col_score, _) = matrix_pattern_and_line(qr, &transpose);
    col_score
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_dark_modules(qr: &QRCode) -> u32 {
    dark_module_score(qr)
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_pattern_and_line(qr: &QRCode) -> (u32, u32, u32) {
    let transpose = transpose(qr);
    matrix_pattern_and_line(qr, &transpose)
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_score_squares(qr: &QRCode) -> u32 {
    matrix_score_squares(qr)
}

/// Computes scores for squares, any 2x2 square (black or white)
/// add 3 to the score
///
/// ### Opti:
/// We don't want to access the 4 squares each time, so we score the left most
/// ones and only fetch the next right ones
fn matrix_score_squares(qr: &QRCode) -> u32 {
    let mut square_score = 0;

    for i in 0..qr.size - 1 {
        let mut count_data = 2;

        let line1 = &qr[i];
        let line2 = &qr[i + 1];

        let mut buffer = 0u8;
        buffer |= (line1[0].value() as u8) << 2;
        buffer |= (line2[0].value() as u8) << 3;

        for j in 0..qr.size - 1 {
            buffer >>= 2;
            buffer |= (line1[j + 1].value() as u8) << 2;
            buffer |= (line2[j + 1].value() as u8) << 3;

            if line1[j + 1].module_type() != ModuleType::Data
                || line2[j + 1].module_type() != ModuleType::Data
            {
                count_data = 0;
            }

            if count_data >= 2 && (buffer == 0b1111 || buffer == 0b0000) {
                square_score += 3;
            }

            count_data += 1;
        }
    }

    square_score
}

/// Computes scores for both patterns (`0b10111010000` or `0b00001011101`)`
///
/// ### Opti:
/// We convert the line to a u11 (supposedly) so comparing it to a pattern is
/// a simple comparaison.
pub fn score_line(line: &[Module]) -> (u32, u32) {
    const PATTERN_LEN: usize = 7;

    let mut line_score = 0;
    let mut patt_score = 0;

    let mut count = 1;
    let mut current = !line[0].value();

    let mut buffer = 0;
    let mut count_data = 0;

    for &item in line.iter() {
        buffer = ((buffer << 1) | (item.value() as u16)) & 0b1111111;
        count_data += 1;

        if item.value() != current {
            if count >= 5 {
                line_score += count - 2;
            }
            count = 0;
            current = item.value();
        }

        if item.module_type() != ModuleType::Data {
            if count >= 5 {
                line_score += count - 2;
            }

            count_data = 0;
            count = 0;
            continue;
        }

        if count_data >= PATTERN_LEN && buffer == 0b1011101 {
            patt_score += 40;
        }

        count += 1;
    }

    if count >= 5 {
        line_score += count - 2;
    }

    (patt_score, line_score)
}

/// Converts the matrix to lines & columns and feed it to `score_line`
///
/// ### Opti:
/// While parsing the whole matrix (converting to col) we also count the
/// number of dark_modules.
fn matrix_pattern_and_line(qr: &QRCode, qr_transpose: &QRCode) -> (u32, u32, u32) {
    let mut line_score = 0;
    let mut col_score = 0;
    let mut patt_score = 0;

    let n = qr.size;

    for i in 0..n {
        let l = score_line(&qr[i]);
        line_score += l.1;

        let c = score_line(&qr_transpose[i]);
        col_score += c.1;

        patt_score += l.0 + c.0;
    }

    (line_score, col_score, patt_score)
}

fn dark_module_score(qr: &QRCode) -> u32 {
    let n = qr.size;
    let dark_modules = qr.data[..n * n]
        .iter()
        .filter(|m| m.value() == Module::DARK)
        .count();

    let percent = (dark_modules * 100) / (n * n);

    hardcode::PERCENT_SCORE[percent as usize] as u32
}

/// Adds every score together
pub fn matrix_score(qr: &QRCode, qr_transpose: &QRCode) -> u32 {
    let dark_score = dark_module_score(qr);
    let square_score = matrix_score_squares(qr);
    let (line_score, col_score, patt_score) = matrix_pattern_and_line(qr, qr_transpose);

    line_score + patt_score + col_score + dark_score + square_score
}
