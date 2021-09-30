//! Contains the HEIGHT functions that can alter QRCode
#![deny(unsafe_code)]
#![warn(missing_docs)]

/// Mask function nb°0
fn mask_0<const N: usize>(mut mat: [[bool; N]; N], mat_full: &[[bool; N]; N]) -> [[bool; N]; N] {
    for row in 0..mat.len() {
        for column in ((row & 1)..mat[0].len()).step_by(2) {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }

    mat
}

/// Mask function nb°1
fn mask_1<const N: usize>(mut mat: [[bool; N]; N], mat_full: &[[bool; N]; N]) -> [[bool; N]; N] {
    for row in (0..mat.len()).step_by(2) {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }

    mat
}

/// Mask function nb°2
fn mask_2<const N: usize>(mut mat: [[bool; N]; N], mat_full: &[[bool; N]; N]) -> [[bool; N]; N] {
    for row in 0..mat.len() {
        for column in (0..mat[0].len()).step_by(3) {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }

    mat
}

/// Mask function nb°3
fn mask_3<const N: usize>(mut mat: [[bool; N]; N], mat_full: &[[bool; N]; N]) -> [[bool; N]; N] {
    for row in 0..mat.len() {
        for column in ((3 - row % 3) % 3..mat[0].len()).step_by(3) {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }

    mat
}

/// Mask function nb°4
fn mask_4<const N: usize>(mut mat: [[bool; N]; N], mat_full: &[[bool; N]; N]) -> [[bool; N]; N] {
    let width = mat[0].len();
    for row in 0..mat.len() {
        for column in (((row / 2) % 2) * 3..width).step_by(6) {
            for i in column..std::cmp::min(width, column + 3) {
                if !mat_full[row][i] {
                    mat[row][i] = !mat[row][i];
                }
            }
        }
    }

    mat
}

/// Mask function nb°5
fn mask_5<const N: usize>(mut mat: [[bool; N]; N], mat_full: &[[bool; N]; N]) -> [[bool; N]; N] {
    for row in 0..mat.len() {
        for column in row..mat[0].len() {
            let row_col = row * column;
            let ok: bool = (row_col % 2) + (row_col % 3) == 0;

            if !mat_full[row][column] && ok {
                mat[row][column] = !mat[row][column];
            }

            if !mat_full[column][row] && ok && column != row {
                mat[column][row] = !mat[column][row];
            }
        }
    }

    mat
}

/// Mask function nb°6
fn mask_6<const N: usize>(mut mat: [[bool; N]; N], mat_full: &[[bool; N]; N]) -> [[bool; N]; N] {
    for row in 0..mat.len() {
        for column in row..mat[0].len() {
            let row_col = row * column;
            let ok: bool = ((row_col % 2) + (row_col % 3)) % 2 == 0;

            if !mat_full[row][column] && ok {
                mat[row][column] = !mat[row][column];
            }

            if !mat_full[column][row] && ok && column != row {
                mat[column][row] = !mat[column][row];
            }
        }
    }

    mat
}

/// Mask function nb°7
fn mask_7<const N: usize>(mut mat: [[bool; N]; N], mat_full: &[[bool; N]; N]) -> [[bool; N]; N] {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (((row + column) % 2) + ((row * column) % 3)) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }

    mat
}

/// Applies the function at `mask_nb` on `mat`
pub fn mask<const N: usize>(
    mat: [[bool; N]; N],
    mask_nb: u8,
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    match mask_nb {
        0 => mask_0(mat, &mat_full),
        1 => mask_1(mat, &mat_full),
        2 => mask_2(mat, &mat_full),
        3 => mask_3(mat, &mat_full),
        4 => mask_4(mat, &mat_full),
        5 => mask_5(mat, &mat_full),
        6 => mask_6(mat, &mat_full),
        7 => mask_7(mat, &mat_full),
        _ => unreachable!(),
    }
}
