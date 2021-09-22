//! QRCodes need a way to define if they are readable, this is the point to
//! this scoring system. The lesser, the better

#[cfg(test)]
pub fn matrix_score_rows_test(mat: &Vec<Vec<bool>>) -> u32 {
    return matrix_score_rows(mat);
}

/**
 * Takes a matrix and return the score for the rows
 * If 5 or more elements are lined up, the score goes
 * up
 */
fn matrix_score_rows(mat: &Vec<Vec<bool>>) -> u32 {
    let height = mat.len();
    let width = mat[0].len();

    let mut score = 0u32;

    for i in 0..height {
        let mut j = 0;

        while j < width {
            let save = j;
            let value = mat[i][j];
            j += 1;
            while j < width && mat[i][j] == value {
                j += 1;
            }

            if j - save >= 5 {
                score += (j - save - 2) as u32;
            }
        }
    }

    return score;
}

#[cfg(test)]
pub fn matrix_score_lines_test(mat: &Vec<Vec<bool>>) -> u32 {
    return matrix_score_lines(mat);
}

/**
 * Takes a matrix and return the score for the lines
 * If 5 or more elements are lined up, the score goes
 * up
 */
fn matrix_score_lines(mat: &Vec<Vec<bool>>) -> u32 {
    let height = mat.len();
    let width = mat[0].len();

    let mut score = 0u32;

    for i in 0..width {
        let mut x = 0;

        while x < height {
            let save = x;
            let value = mat[x][i];
            x += 1;
            while x < width && mat[x][i] == value {
                x += 1;
            }

            if x - save >= 5 {
                score += (x - save - 2) as u32;
            }
        }
    }

    return score;
}

#[cfg(test)]
pub fn matrix_score_squares_test(mat: &Vec<Vec<bool>>) -> u32 {
    return matrix_score_squares(mat);
}

/**
 * Takes a matrix and return the score formed by square (2x2)
 * If a square appears (black or white), score goes up
 */
fn matrix_score_squares(mat: &Vec<Vec<bool>>) -> u32 {
    let mut score = 0;

    for i in 0..mat.len() - 1 {
        for j in 0..mat[0].len() - 1 {
            let current = mat[i][j];

            if current == mat[i + 1][j + 0]
                && current == mat[i + 1][j + 1]
                && current == mat[i + 0][j + 1]
            {
                score += 3;
            }
        }
    }

    return score;
}

#[cfg(test)]
pub fn matrix_score_pattern_test(mat: &Vec<Vec<bool>>) -> u32 {
    return matrix_score_pattern(mat);
}

#[inline]
fn pattern_col(mat: &Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
    if mat[i + 2][j]
        && mat[i + 3][j]
        && mat[i + 4][j]
        && mat[i + 0][j]
        && !mat[i + 7][j]
        && !mat[i + 8][j]
        && !mat[i + 9][j]
        && !mat[i + 10][j]
        && !mat[i + 1][j]
        && !mat[i + 5][j]
        && mat[i + 6][j]
    {
        return 40;
    }

    if !mat[i + 2][j]
        && !mat[i + 0][j]
        && !mat[i + 1][j]
        && !mat[i + 3][j]
        && mat[i + 6][j]
        && mat[i + 7][j]
        && mat[i + 8][j]
        && mat[i + 4][j]
        && !mat[i + 5][j]
        && !mat[i + 9][j]
        && mat[i + 10][j]
    {
        return 40;
    }

    return 0;
}

#[inline]
fn pattern_line(mat: &Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
    if mat[i][j + 2]
        && mat[i][j + 3]
        && mat[i][j + 4]
        && !mat[i][j + 7]
        && !mat[i][j + 8]
        && !mat[i][j + 9]
        && !mat[i][j + 10]
        && mat[i][j + 0]
        && !mat[i][j + 1]
        && !mat[i][j + 5]
        && mat[i][j + 6]
    {
        return 40;
    }

    if !mat[i][j + 2]
        && !mat[i][j + 0]
        && !mat[i][j + 1]
        && !mat[i][j + 3]
        && mat[i][j + 6]
        && mat[i][j + 7]
        && mat[i][j + 8]
        && mat[i][j + 4]
        && !mat[i][j + 5]
        && !mat[i][j + 9]
        && mat[i][j + 10]
    {
        return 40;
    }

    return 0;
}

/**
 * Takes a matrix and return the score formed by finder pattern patterns
 * If a pattern appears, score goes up: `█_███_█____` like so
 */
fn matrix_score_pattern(mat: &Vec<Vec<bool>>) -> u32 {
    let mut score = 0;

    const PATTERN_LEN: usize = 11;
    let height = mat.len();
    let width = mat[0].len();

    let height_limit = height - PATTERN_LEN;
    let width_limit = width - PATTERN_LEN;

    for i in 0..=height_limit {
        for j in 0..=width_limit {
            score += pattern_col(mat, i, j);
            score += pattern_line(mat, i, j);
        }
    }
    for i in height_limit + 1..height {
        for j in 0..=width_limit {
            score += pattern_line(mat, i, j);
        }
    }
    for i in 0..=height_limit {
        for j in width_limit + 1..width {
            score += pattern_col(mat, i, j);
        }
    }

    return score;
}

#[cfg(test)]
pub fn matrix_score_modules_test(mat: &Vec<Vec<bool>>) -> u32 {
    return matrix_score_modules(mat);
}

/**
 * Takes a matrix and return the score of the overall qrcode
 * Takes the nb of 'set' pixel and the total number
 * Find if it's close to 50% or not
 */
fn matrix_score_modules(mat: &Vec<Vec<bool>>) -> u32 {
    let mut dark_modules = 0;

    for row in mat {
        for &module in row {
            if module {
                dark_modules += 1;
            }
        }
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

/// Adds every score together
pub fn matrix_score(mat: &Vec<Vec<bool>>) -> u32 {
    return matrix_score_rows(mat)
        + matrix_score_lines(mat)
        + matrix_score_squares(mat)
        + matrix_score_pattern(mat)
        + matrix_score_modules(mat);
}
