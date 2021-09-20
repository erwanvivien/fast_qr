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

    let mut i = 0;
    while i < height {
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

        i += 1;
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

    let mut i = 0;
    while i < width {
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

        i += 1;
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
            const OFFSET: [(usize, usize); 3] = [(0, 1), (1, 0), (1, 1)];
            let current = mat[i][j];

            let mut k = 0;
            for off in &OFFSET {
                if current != mat[i + off.0][j + off.1] {
                    break;
                }
                k += 1;
            }

            if k == OFFSET.len() {
                score += 1;
            }
        }
    }

    return 3 * score;
}

#[cfg(test)]
pub fn matrix_score_pattern_test(mat: &Vec<Vec<bool>>) -> u32 {
    return matrix_score_pattern(mat);
}

/**
 * Takes a matrix and return the score formed by finder pattern patterns
 * If a pattern appears, score goes up: `█_███_█____` like so
 */
fn matrix_score_pattern(mat: &Vec<Vec<bool>>) -> u32 {
    let mut score = 0;
    const PATTERN: [[bool; 11]; 2] = [
        [
            true, false, true, true, true, false, true, false, false, false, false,
        ],
        [
            false, false, false, false, true, false, true, true, true, false, true,
        ],
    ];
    const PATTERN_LEN: usize = PATTERN[0].len();

    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            for pat in &PATTERN {
                let mut k: usize = 0;
                // Cols
                while k < PATTERN_LEN {
                    if i + k >= mat.len() {
                        break;
                    }

                    if mat[i + k][j] != pat[k] {
                        break;
                    }

                    k += 1;
                }
                if k == PATTERN_LEN {
                    score += 1;
                }

                // Lines
                k = 0;
                while k < PATTERN_LEN {
                    if j + k >= mat[0].len() {
                        break;
                    }

                    if mat[i][j + k] != pat[k] {
                        break;
                    }

                    k += 1;
                }
                if k == PATTERN_LEN {
                    score += 1;
                }
            }
        }
    }

    return score * 40;
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
