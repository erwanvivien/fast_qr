//! QRCodes need a way to define if they are readable, this is the point to
//! this scoring system. The lesser, the better

#![warn(missing_docs)]

/**
 * Takes a matrix and return the score formed by square (2x2)
 * If a square appears (black or white), score goes up
 */
const fn matrix_score_squares<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let mut score = 0;

    let mut i = 0;
    while i < N - 1 {
        let mut j = 0;
        while j < N - 1 {
            let current = mat[i][j];

            if current == mat[i + 1][j + 0]
                && current == mat[i + 1][j + 1]
                && current == mat[i + 0][j + 1]
            {
                score += 3;
            }

            j += 1;
        }
        i += 1;
    }

    return score;
}

/**
 * Takes a matrix and return the score of the overall qrcode
 * Takes the nb of 'set' pixel and the total number
 * Find if it's close to 50% or not
 */
const fn matrix_score_modules<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let mut dark_modules = 0;

    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            if mat[i][j] {
                dark_modules += 1;
            }

            j += 1;
        }
        i += 1;
    }

    let total_modules = mat.len() * mat[0].len();

    let percent = (dark_modules * 100) / total_modules;
    let mut lower_bound = (percent - (percent % 5)) as i8;
    let mut higher_bound = (percent + (5 - percent % 5)) as i8;

    lower_bound = (lower_bound - 50).abs();
    higher_bound = (higher_bound - 50).abs();

    return if lower_bound < higher_bound {
        lower_bound * 2
    } else {
        higher_bound * 2
    } as u32;
}

const fn score_trailing<const N: usize>(
    buffer: u16,
    buffer_size: u32,
    line: &[bool; N],
    index: usize,
) -> u32 {
    let mut trailing = if buffer & 1 == 1 {
        buffer.trailing_ones()
    } else {
        buffer.trailing_zeros()
    };

    if trailing > buffer_size {
        trailing = buffer_size;
    }

    if trailing >= 11 {
        let mut idx = index;
        while idx < N && line[idx] == line[idx - 1] {
            trailing += 1;
            idx += 1;
        }
    }

    if trailing >= 5 {
        return trailing - 2;
    }

    return 0;
}

const fn score_line<const N: usize>(line: &[bool; N]) -> u32 {
    const PATTERN_LEN: usize = 11;

    let mut score = 0;
    let mut buffer = 0u16;

    let mut i = 0;
    while i < PATTERN_LEN {
        if line[i] {
            buffer |= 1 << i;
        }
        i += 1;
    }

    let mut current_color = ((buffer & 1) + 1) & 1;
    while i < N {
        if buffer == 0b10111010000 || buffer == 0b00001011101 {
            score += 40;
        }
        if buffer & 1 != current_color {
            score += score_trailing(buffer, 11, line, i);
            current_color = buffer & 1;
        }

        buffer >>= 1;
        if line[i] {
            buffer |= 1 << (PATTERN_LEN - 1);
        }

        i += 1;
    }

    if buffer == 0b10111010000 || buffer == 0b00001011101 {
        score += 40;
    }

    let mut i = 0;
    while i <= 11 - 5 {
        if buffer & 1 != current_color {
            score += score_trailing(buffer, 11 - i, line, usize::MAX / 2);
            current_color = buffer & 1;
        }

        buffer >>= 1;
        i += 1;
    }

    return score;
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_pattern_and_line<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    return matrix_pattern_and_line(mat);
}

const fn matrix_pattern_and_line<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let mut buffer_col = [false; N];
    let mut score = 0;

    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            buffer_col[j] = mat[j][i];
            j += 1;
        }

        score += score_line(&mat[i]);
        score += score_line(&buffer_col);

        i += 1;
    }

    return score;
}

/// Adds every score together
pub const fn matrix_score<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    return matrix_score_squares(mat) + matrix_pattern_and_line(mat) + matrix_score_modules(mat);
}
