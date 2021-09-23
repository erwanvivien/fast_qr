//! QRCodes need a way to define if they are readable, this is the point to
//! this scoring system. The lesser, the better

#![warn(missing_docs)]

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
            unsafe {
                let value = mat.get_unchecked(i).get_unchecked(j);
                j += 1;
                while j < width && mat.get_unchecked(i).get_unchecked(j) == value {
                    j += 1;
                }
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
            unsafe {
                let value = mat.get_unchecked(x).get_unchecked(i);
                x += 1;
                while x < width && mat.get_unchecked(x).get_unchecked(i) == value {
                    x += 1;
                }
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
            unsafe {
                let current = mat.get_unchecked(i).get_unchecked(j);

                if current == mat.get_unchecked(i + 1).get_unchecked(j + 0)
                    && current == mat.get_unchecked(i + 1).get_unchecked(j + 1)
                    && current == mat.get_unchecked(i + 0).get_unchecked(j + 1)
                {
                    score += 3;
                }
            }
        }
    }

    return score;
}

#[cfg(test)]
pub fn matrix_score_pattern_test(mat: &Vec<Vec<bool>>) -> u32 {
    return matrix_score_pattern(mat);
}

/// Ref to false
const FALSE_REF: &bool = &false;
/// Ref to true
const TRUE_REF: &bool = &true;

#[inline(always)]
fn pattern_col_12(mat: &Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
    unsafe {
        if mat.get_unchecked(i + 0).get_unchecked(j) == TRUE_REF
            && mat.get_unchecked(i + 1).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 2).get_unchecked(j) == TRUE_REF
            && mat.get_unchecked(i + 3).get_unchecked(j) == TRUE_REF
            && mat.get_unchecked(i + 4).get_unchecked(j) == TRUE_REF
            && mat.get_unchecked(i + 5).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 6).get_unchecked(j) == TRUE_REF
            && mat.get_unchecked(i + 7).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 8).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 9).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 10).get_unchecked(j) == FALSE_REF
        {
            return 40;
        }
    }
    unsafe {
        if mat.get_unchecked(i + 0).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 1).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 2).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 3).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 4).get_unchecked(j) == TRUE_REF
            && mat.get_unchecked(i + 5).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 6).get_unchecked(j) == TRUE_REF
            && mat.get_unchecked(i + 7).get_unchecked(j) == TRUE_REF
            && mat.get_unchecked(i + 8).get_unchecked(j) == TRUE_REF
            && mat.get_unchecked(i + 9).get_unchecked(j) == FALSE_REF
            && mat.get_unchecked(i + 10).get_unchecked(j) == TRUE_REF
        {
            return 40;
        }
    }

    return 0;
}

#[inline(always)]
fn pattern_line_12(mat: &Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
    unsafe {
        if mat.get_unchecked(i).get_unchecked(j + 0) == TRUE_REF
            && mat.get_unchecked(i).get_unchecked(j + 1) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 2) == TRUE_REF
            && mat.get_unchecked(i).get_unchecked(j + 3) == TRUE_REF
            && mat.get_unchecked(i).get_unchecked(j + 4) == TRUE_REF
            && mat.get_unchecked(i).get_unchecked(j + 5) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 6) == TRUE_REF
            && mat.get_unchecked(i).get_unchecked(j + 7) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 8) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 9) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 10) == FALSE_REF
        {
            return 40;
        }
    }

    unsafe {
        if mat.get_unchecked(i).get_unchecked(j + 0) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 1) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 2) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 3) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 4) == TRUE_REF
            && mat.get_unchecked(i).get_unchecked(j + 5) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 6) == TRUE_REF
            && mat.get_unchecked(i).get_unchecked(j + 7) == TRUE_REF
            && mat.get_unchecked(i).get_unchecked(j + 8) == TRUE_REF
            && mat.get_unchecked(i).get_unchecked(j + 9) == FALSE_REF
            && mat.get_unchecked(i).get_unchecked(j + 10) == TRUE_REF
        {
            return 40;
        }
    }

    return 0;
}

fn matrix_score_pattern(mat: &Vec<Vec<bool>>) -> u32 {
    let mut score = 0;

    const PATTERN_LEN: usize = 11;
    let height = mat.len();
    let width = mat[0].len();

    let height_limit = height - PATTERN_LEN;
    let width_limit = width - PATTERN_LEN;

    for i in 0..=height_limit {
        for j in 0..=width_limit {
            score += pattern_col_12(mat, i, j);
            score += pattern_line_12(mat, i, j);
        }
    }
    for i in height_limit + 1..height {
        for j in 0..=width_limit {
            score += pattern_line_12(mat, i, j);
        }
    }
    for i in 0..=height_limit {
        for j in width_limit + 1..width {
            score += pattern_col_12(mat, i, j);
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
