#[cfg(test)]
pub fn matrix_score_rows_test(mat: &Vec<Vec<bool>>) -> u32 {
    return matrix_score_rows(mat);
}

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

fn matrix_score_squares(mat: &Vec<Vec<bool>>) -> u32 {
    let mut score = 0;

    for i in 0..mat.len() - 1 {
        for j in 0..mat[0].len() - 1 {
            const OFFSET: [(usize, usize); 3] = [(0, 1), (1, 0), (1, 1)];
            let current = mat[i][j];

            let mut k = 0;
            while k < OFFSET.len() {
                if current != mat[i + OFFSET[k].0][j + OFFSET[k].1] {
                    break;
                }
                k += 1;
            }

            if k == OFFSET.len() {
                println!("{} {}", i, j);
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

fn matrix_score_pattern(mat: &Vec<Vec<bool>>) -> u32 {
    let mut score = 0;
    const PATTERN: [bool; 11] = [
        true, false, true, true, true, false, true, false, false, false, false,
    ];
    const PATTERN_LEN: usize = PATTERN.len();

    for i in 0..=mat.len() - PATTERN_LEN {
        for j in 0..mat[0].len() {
            for pat in 0..2 {
                let mut k = 0;

                while k < PATTERN_LEN {
                    let tmp_k = if pat == 0 { k } else { PATTERN_LEN - 1 - k };

                    if mat[i + k][j] != PATTERN[tmp_k] {
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
    for i in 0..mat.len() {
        for j in 0..=mat[0].len() - PATTERN_LEN {
            for pat in 0..2 {
                let mut k = 0;

                while k < PATTERN_LEN {
                    let tmp_k = if pat == 0 { k } else { PATTERN_LEN - 1 - k };

                    if mat[i][j + k] != PATTERN[tmp_k] {
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

fn matrix_score_modules(mat: &Vec<Vec<bool>>) -> u32 {
    let mut dark_modules = 0;

    for row in mat {
        for &module in row {
            if module {
                dark_modules += 1;
            }
        }
    }

    println!("{}", dark_modules);

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

pub fn matrix_score(mat: &Vec<Vec<bool>>) -> u32 {
    return matrix_score_rows(mat)
        + matrix_score_lines(mat)
        + matrix_score_squares(mat)
        + matrix_score_pattern(mat)
        + matrix_score_modules(mat);
}
