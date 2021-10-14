//! QRCodes need a way to define if they are readable, this is the point to
//! this scoring system. The lesser, the better

#![warn(missing_docs)]

#[cfg(test)]
pub fn matrix_score_rows_test<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    return matrix_score_rows(mat);
}

/**
 * Takes a matrix and return the score for the rows
 * If 5 or more elements are lined up, the score goes
 * up
 */
const fn matrix_score_rows<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let mut score = 0u32;

    let mut i = 0;
    while i < N {
        let mut j = 0;

        while j < N {
            let save = j;
            let value = mat[i][j];
            j += 1;
            while j < N && mat[i][j] == value {
                j += 1;
            }

            if j - save >= 5 {
                score += (j - save - 2) as u32;
            }
        }

        i += 1;
    }

    return score;
}

#[cfg(test)]
pub fn matrix_score_lines_test<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    return matrix_score_lines(mat);
}

/**
 * Takes a matrix and return the score for the lines
 * If 5 or more elements are lined up, the score goes
 * up
 */
const fn matrix_score_lines<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let mut score = 0u32;

    let mut i = 0;
    while i < N {
        let mut x = 0;

        while x < N {
            let save = x;
            let value = mat[x][i];
            x += 1;
            while x < N && mat[x][i] == value {
                x += 1;
            }

            if x - save >= 5 {
                score += (x - save - 2) as u32;
            }
        }

        i += 1;
    }

    return score;
}

#[cfg(test)]
pub fn matrix_score_squares_test<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    return matrix_score_squares(mat);
}

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

#[cfg(test)]
pub fn matrix_score_pattern_test<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    return matrix_score_pattern(mat);
}

#[inline(always)]
/// Helper function to check column
const fn pattern_col_12<const N: usize>(mat: &[[bool; N]; N], i: usize, j: usize) -> u32 {
    if mat[i + 0][j] == true
        && mat[i + 1][j] == false
        && mat[i + 2][j] == true
        && mat[i + 3][j] == true
        && mat[i + 4][j] == true
        && mat[i + 5][j] == false
        && mat[i + 6][j] == true
        && mat[i + 7][j] == false
        && mat[i + 8][j] == false
        && mat[i + 9][j] == false
        && mat[i + 10][j] == false
    {
        return 40;
    }
    if mat[i + 0][j] == false
        && mat[i + 1][j] == false
        && mat[i + 2][j] == false
        && mat[i + 3][j] == false
        && mat[i + 4][j] == true
        && mat[i + 5][j] == false
        && mat[i + 6][j] == true
        && mat[i + 7][j] == true
        && mat[i + 8][j] == true
        && mat[i + 9][j] == false
        && mat[i + 10][j] == true
    {
        return 40;
    }

    return 0;
}

#[inline(always)]
/// Helper function to check row
const fn pattern_line_12<const N: usize>(mat: &[[bool; N]; N], i: usize, j: usize) -> u32 {
    if mat[i][j + 0] == true
        && mat[i][j + 1] == false
        && mat[i][j + 2] == true
        && mat[i][j + 3] == true
        && mat[i][j + 4] == true
        && mat[i][j + 5] == false
        && mat[i][j + 6] == true
        && mat[i][j + 7] == false
        && mat[i][j + 8] == false
        && mat[i][j + 9] == false
        && mat[i][j + 10] == false
    {
        return 40;
    }

    if mat[i][j + 0] == false
        && mat[i][j + 1] == false
        && mat[i][j + 2] == false
        && mat[i][j + 3] == false
        && mat[i][j + 4] == true
        && mat[i][j + 5] == false
        && mat[i][j + 6] == true
        && mat[i][j + 7] == true
        && mat[i][j + 8] == true
        && mat[i][j + 9] == false
        && mat[i][j + 10] == true
    {
        return 40;
    }

    return 0;
}

/// Checks a finder pattern for every row / column, asif pattern is
/// [[true, false, true, true, true, false, true, false, false, false, false]](https://www.thonky.com/qr-code-tutorial/data-masking#evaluation-condition-3)
const fn matrix_score_pattern<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let mut score = 0;

    const PATTERN_LEN: usize = 11;
    let height_limit = N - PATTERN_LEN;
    let width_limit = N - PATTERN_LEN;

    let mut i = 0;
    while i <= height_limit {
        let mut j = 0;
        while j <= width_limit {
            score += pattern_col_12(mat, i, j);
            score += pattern_line_12(mat, i, j);

            j += 1;
        }
        i += 1;
    }

    let mut i = height_limit + 1;
    while i < N {
        let mut j = 0;
        while j <= width_limit {
            score += pattern_line_12(mat, i, j);
            j += 1;
        }
        i += 1;
    }

    let mut i = 0;
    while i <= height_limit {
        let mut j = width_limit + 1;
        while j < N {
            score += pattern_col_12(mat, i, j);
            j += 1;
        }
        i += 1;
    }

    return score;
}

#[cfg(test)]
pub fn matrix_score_modules_test<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    return matrix_score_modules(mat);
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
