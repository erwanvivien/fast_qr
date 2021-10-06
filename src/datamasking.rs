//! Contains the HEIGHT functions that can alter QRCode
#![deny(unsafe_code)]
#![warn(missing_docs)]

/// Mask function nb°0
const fn mask_0<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    let mut row = 0;
    while row < N {
        let mut column = row & 1;
        while column < N {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }

            column += 2;
        }

        row += 1;
    }

    return mat;
}

/// Mask function nb°1
const fn mask_1<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    let mut row = 0;
    while row < N {
        let mut column = 0;
        while column < N {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }

            column += 1;
        }

        row += 2;
    }

    return mat;
}

/// Mask function nb°2
const fn mask_2<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    let mut row = 0;
    while row < N {
        let mut column = 0;
        while column < N {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }

            column += 3;
        }

        row += 1;
    }

    return mat;
}

/// Mask function nb°3
const fn mask_3<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    let mut row = 0;
    while row < N {
        let mut column = (3 - row % 3) % 3;
        while column < N {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }

            column += 3;
        }

        row += 1;
    }

    return mat;
}

/// Mask function nb°4
const fn mask_4<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    let mut row = 0;
    while row < N {
        let mut column = ((row / 2) % 2) * 3;
        while column < N {
            let mut i = column;
            let max = if N > column + 3 { column + 3 } else { N };
            while i < max {
                if !mat_full[row][i] {
                    mat[row][i] = !mat[row][i];
                }

                i += 1;
            }

            column += 6;
        }

        row += 1;
    }

    return mat;
}

/// Mask function nb°5
const fn mask_5<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    let mut row = 0;
    while row < N {
        let mut column = 0;
        while column < N {
            let row_col = row * column;
            let ok: bool = (row_col % 2) + (row_col % 3) == 0;

            if !mat_full[row][column] && ok {
                mat[row][column] = !mat[row][column];
            }

            if !mat_full[column][row] && ok && column != row {
                mat[column][row] = !mat[column][row];
            }

            column += 1;
        }

        row += 1;
    }

    return mat;
}

/// Mask function nb°6
const fn mask_6<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    let mut row = 0;
    while row < N {
        let mut column = 0;
        while column < N {
            let row_col = row * column;
            let ok: bool = ((row_col % 2) + (row_col % 3)) % 2 == 0;

            if !mat_full[row][column] && ok {
                mat[row][column] = !mat[row][column];
            }

            if !mat_full[column][row] && ok && column != row {
                mat[column][row] = !mat[column][row];
            }

            column += 1;
        }

        row += 1;
    }

    return mat;
}

/// Mask function nb°7
const fn mask_7<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    let mut row = 0;
    while row < N {
        let mut column = 0;
        while column < N {
            if !mat_full[row][column] && (((row + column) % 2) + ((row * column) % 3)) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }

            column += 1;
        }

        row += 1;
    }

    return mat;
}

/// Applies the function at `mask_nb` on `mat`
pub const fn mask<const N: usize>(
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
        _ => mat,
    }
}
