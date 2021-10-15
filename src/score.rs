//! QRCodes need a way to define if they are readable, this is the point to
//! this scoring system. The lesser, the better

#![warn(missing_docs)]

#[allow(dead_code)]
#[cfg(test)]
pub const fn test_score_line<const N: usize>(line: &[bool; N]) -> u32 {
    return score_line(line);
}

#[allow(dead_code)]
#[cfg(test)]
pub fn test_matrix_pattern_and_line<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    return matrix_pattern_and_line(mat);
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

/// Computes scores for consecutive modules of the same colors
/// if we have 5 or more modules, the score is `x - 2`
/// (x being the number of modules)
///
/// ### Opti:
/// We are using `u16::trailing_zeros` and `u16::trailing_ones` to
/// compute the consecutive squares
const fn score_trailing(buffer: u16, buffer_size: u32) -> u32 {
    let mut trailing = if buffer & 1 == 1 {
        buffer.trailing_ones()
    } else {
        buffer.trailing_zeros()
    };

    if trailing >= buffer_size {
        trailing = buffer_size;
        if buffer_size == 11 {
            return 1;
        }
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
pub const fn score_line<const N: usize>(line: &[bool; N]) -> u32 {
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
            let tmp = score_trailing(buffer, 11);
            score += tmp;
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
        score += 40;
    }

    let mut i = 0;
    if buffer == 0b11111111111 || buffer == 0b00000000000 {
        current_color = ((buffer & 1) + 1) & 1;
        score += 1;
        i = 1;
        buffer >>= 1;
    }

    while i <= 11 - 5 {
        if buffer & 1 != current_color {
            score += score_trailing(buffer, 11 - i);
            current_color = buffer & 1;
        }
        buffer >>= 1;
        i += 1;
    }

    return score;
}

/// Converts the matrix to lines & columns and feed it to `score_line`
///
/// ### Opti:
/// While parsing the whole matrix (converting to col) we also count the
/// number of dark_modules.
const fn matrix_pattern_and_line<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    let mut buffer_col = [false; N];
    let mut score = 0;

    let mut dark_modules = 0;

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

        score += score_line(&mat[i]);
        score += score_line(&buffer_col);

        i += 1;
    }

    let percent = (dark_modules as usize * 100) / (N * N);
    let mut lower_bound = (percent - (percent % 5)) as i8;
    let mut higher_bound = (percent + (5 - percent % 5)) as i8;

    lower_bound = (lower_bound - 50).abs();
    higher_bound = (higher_bound - 50).abs();

    let dark_score = if lower_bound < higher_bound {
        lower_bound * 2
    } else {
        higher_bound * 2
    } as u32;

    return score + dark_score;
}

/// Adds every score together
pub const fn matrix_score<const N: usize>(mat: &[[bool; N]; N]) -> u32 {
    return matrix_score_squares(mat) + matrix_pattern_and_line(mat);
}
