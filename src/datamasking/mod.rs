//! Contains the HEIGHT functions that can alter QRCode
#![deny(unsafe_code)]
#![warn(missing_docs)]

#[cfg(test)]
mod test;

/// Mask function nb°**0**
fn mask_0<const N: usize>(mat: &mut [[bool; N]; N], mat_full: &[[bool; N]; N]) {
    for row in 0..N {
        for column in (row & 1..N).step_by(2) {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°**1**
fn mask_1<const N: usize>(mat: &mut [[bool; N]; N], mat_full: &[[bool; N]; N]) {
    for row in (0..N).step_by(2) {
        for column in 0..N {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°**2**
fn mask_2<const N: usize>(mat: &mut [[bool; N]; N], mat_full: &[[bool; N]; N]) {
    for row in 0..N {
        for column in (0..N).step_by(3) {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°**3**
fn mask_3<const N: usize>(mat: &mut [[bool; N]; N], mat_full: &[[bool; N]; N]) {
    for row in 0..N {
        let start = (3 - row % 3) % 3;
        for column in (start..N).step_by(3) {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°**4**
fn mask_4<const N: usize>(mat: &mut [[bool; N]; N], mat_full: &[[bool; N]; N]) {
    for row in 0..N {
        let start = ((row >> 1) & 1) * 3; // ((row / 2) % 2) * 3;
        for column in (start..N).step_by(6) {
            for i in column..std::cmp::min(N, column + 3) {
                if !mat_full[row][i] {
                    mat[row][i] = !mat[row][i];
                }
            }
        }
    }
}

fn mask_5_6<const N: usize>(
    mat: &mut [[bool; N]; N],
    mat_full: &[[bool; N]; N],
    offset: &[(usize, usize)],
) {
    for row in (0..N).step_by(6) {
        for column in 0..N {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
            if !mat_full[column][row] && (row % 6 != 0 || column % 6 != 0) {
                mat[column][row] = !mat[column][row];
            }
        }
    }

    for row in (0..N).step_by(6) {
        for column in (0..N).step_by(6) {
            for (y, x) in offset {
                if row + y < N && column + x < N && !mat_full[row + y][column + x] {
                    mat[row + y][column + x] = !mat[row + y][column + x];
                }
            }
        }
    }
}

/// Mask function nb°**5**
fn mask_5<const N: usize>(mat: &mut [[bool; N]; N], mat_full: &[[bool; N]; N]) {
    const OFFSETS: [(usize, usize); 4] = [(2, 3), (3, 2), (3, 4), (4, 3)];
    mask_5_6(mat, mat_full, &OFFSETS);
}

/// Mask function nb°**6**
fn mask_6<const N: usize>(mat: &mut [[bool; N]; N], mat_full: &[[bool; N]; N]) {
    #[rustfmt::skip]
    const OFFSETS: [(usize, usize); 12] = [
        (1, 1), (1, 2), (2, 1), (2, 3),
        (2, 4), (3, 2), (3, 4), (4, 2),
        (4, 3), (4, 5), (5, 4), (5, 5)
    ];
    mask_5_6(mat, mat_full, &OFFSETS);
}

/// Mask function nb°**7**
fn mask_7<const N: usize>(mat: &mut [[bool; N]; N], mat_full: &[[bool; N]; N]) {
    for row in 0..N {
        for column in row..N {
            if !mat_full[row][column] && (((row + column) % 2) + ((row * column) % 3)) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
            if column != row
                && !mat_full[column][row]
                && (((row + column) % 2) + ((row * column) % 3)) % 2 == 0
            {
                mat[column][row] = !mat[column][row];
            }
        }
    }
}

/// Applies the function at `mask_nb` on `mat`
pub fn mask<const N: usize>(mat: &mut [[bool; N]; N], mask_nb: usize, mat_full: &[[bool; N]; N]) {
    match mask_nb {
        0 => mask_0(mat, mat_full),
        1 => mask_1(mat, mat_full),
        2 => mask_2(mat, mat_full),
        3 => mask_3(mat, mat_full),
        4 => mask_4(mat, mat_full),
        5 => mask_5(mat, mat_full),
        6 => mask_6(mat, mat_full),
        7 => mask_7(mat, mat_full),
        _ => panic!("Mask nb should never be {}", mask_nb),
    }
}
