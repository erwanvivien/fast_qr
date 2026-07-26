//! `QRCode` need a way to define if they are readable, using a
//! scoring system. The lesser, the better.

#![warn(missing_docs)]

use crate::module::{Module, ModuleType};
use crate::QRCode;

use super::hardcode;

#[cfg(test)]
pub fn test_score_line(l: &[Module]) -> u32 {
    line(l).1
}

#[cfg(test)]
pub fn test_score_pattern(l: &[Module]) -> u32 {
    line(l).0
}

#[cfg(test)]
pub fn test_matrix_dark_modules(qr: &QRCode) -> u32 {
    dark_module_score(qr)
}

#[cfg(test)]
pub fn test_matrix_pattern_and_line(qr: &QRCode) -> (u32, u32, u32) {
    matrix_pattern_and_line(qr)
}

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
        buffer |= u8::from(line1[0].value()) << 2;
        buffer |= u8::from(line2[0].value()) << 3;

        for j in 0..qr.size - 1 {
            buffer >>= 2;
            buffer |= u8::from(line1[j + 1].value()) << 2;
            buffer |= u8::from(line2[j + 1].value()) << 3;

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

/// Computes scores for both patterns (`0b10111010000` or `0b00001011101`)
///
/// ### Opti:
/// We convert the line to a u11 (supposedly) so comparing it to a pattern is
/// a simple comparison.
fn line(line: &[Module]) -> (u32, u32) {
    // ISO/IEC 18004 rule N3: a 1011101 finder lookalike only scores when it is
    // preceded or followed by four light modules. The rolling window is 11 bits
    // wide and scans the whole line by value: patterns spanning data and
    // function modules change with the mask and must be counted. Modules
    // beyond the symbol edge count as light (the quiet zone), matching the
    // reference implementations; a lookalike that is light on both sides
    // matches both windows and scores twice.
    const PATTERN_LIGHT_BEFORE: u16 = 0b00001011101;
    const PATTERN_LIGHT_AFTER: u16 = 0b10111010000;
    const PATTERN_WINDOW: u16 = 0b11111111111;

    let mut line_score = 0;
    let mut patt_score = 0;

    let mut count = 1;
    let mut current = !line[0].value();

    // Starts at zero: the leading bits stand in for the light quiet zone.
    let mut buffer: u16 = 0;

    for &item in line {
        buffer = ((buffer << 1) | u16::from(item.value())) & PATTERN_WINDOW;
        if buffer == PATTERN_LIGHT_BEFORE || buffer == PATTERN_LIGHT_AFTER {
            patt_score += 40;
        }

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

            count = 0;
            continue;
        }

        count += 1;
    }

    if count >= 5 {
        line_score += count - 2;
    }

    // Shift the trailing quiet zone in; only the lights-after pattern can
    // still complete here.
    for _ in 0..4 {
        buffer = (buffer << 1) & PATTERN_WINDOW;
        if buffer == PATTERN_LIGHT_AFTER {
            patt_score += 40;
        }
    }

    (patt_score, line_score)
}

/// Converts the matrix to lines & columns and feed it to `score_line`.
///
/// Columns are gathered into a small stack buffer instead of materializing a
/// transposed copy of the matrix: scoring runs once per mask probe, and eight
/// full transposes would dominate the encoding cost.
fn matrix_pattern_and_line(qr: &QRCode) -> (u32, u32, u32) {
    let mut line_score = 0;
    let mut col_score = 0;
    let mut patt_score = 0;

    let n = qr.size;
    let mut column = [Module::data(Module::LIGHT); crate::qr::QR_MAX_WIDTH];

    for i in 0..n {
        let l = line(&qr[i]);
        line_score += l.1;

        for (module, &value) in column[..n]
            .iter_mut()
            .zip(qr.data[i..n * n].iter().step_by(n))
        {
            *module = value;
        }
        let c = line(&column[..n]);
        col_score += c.1;

        patt_score += l.0 + c.0;
    }

    (line_score, col_score, patt_score)
}

/// Computes the number of `ModuleType::Dark` modules
fn dark_module_score(qr: &QRCode) -> u32 {
    let n = qr.size;
    let dark_modules = qr.data[..n * n]
        .iter()
        .filter(|m| m.value() == Module::DARK)
        .count();

    let percent = (dark_modules * 100) / (n * n);
    u32::from(hardcode::PERCENT_SCORE[percent])
}

/// Computes the score for the matrix
/// - `matrix_pattern_and_line`:
///   - 40 points for each [TFTTTFT] pattern (T: true / F: false)
///   - N - 2 points for each line with N consecutive modules of the same color (N >= 5)
/// - `matrix_score_squares`: 3 points for each 2x2 square (black or white)
/// - `dark_module_score`: 10 points for each 5% of dark modules away from 50%
pub fn score(qr: &QRCode) -> u32 {
    let dark_score = dark_module_score(qr);
    let square_score = matrix_score_squares(qr);
    let (line_score, col_score, patt_score) = matrix_pattern_and_line(qr);

    line_score + patt_score + col_score + dark_score + square_score
}
