//! Contains the HEIGHT functions that can alter QRCode
#![deny(unsafe_code)]
#![warn(missing_docs)]

#[cfg(test)]
mod test;

/// Mask function nb°**0**
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

    mat
}

/// Mask function nb°**1**
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

    mat
}

/// Mask function nb°**2**
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

    mat
}

/// Mask function nb°**3**
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

    mat
}

/// Mask function nb°**4**
const fn mask_4<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    let mut row = 0;
    while row < N {
        let mut column = ((row >> 1) & 1) * 3; // ((row / 2) % 2) * 3;
        while column < N {
            let mut i = column;
            while i < column + 3 && i < N {
                if !mat_full[row][i] {
                    mat[row][i] = !mat[row][i];
                }

                i += 1;
            }

            column += 6;
        }

        row += 1;
    }

    mat
}

/// Mask function nb°**5**
const fn mask_5<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    const OFFSETS: [(usize, usize); 4] = [(2, 3), (3, 2), (3, 4), (4, 3)];

    let mut row = 0;
    while row < N {
        let mut column = 0;
        while column < N {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
            if !mat_full[column][row] && (row % 6 != 0 || column % 6 != 0) {
                mat[column][row] = !mat[column][row];
            }
            column += 1;
        }

        row += 6;
    }

    let mut row = 0;
    while row < N {
        let mut column = 0;
        while column < N {
            let mut idx = 0;
            while idx < OFFSETS.len() {
                let (y, x) = OFFSETS[idx];
                if row + y < N && column + x < N && !mat_full[row + y][column + x] {
                    mat[row + y][column + x] = !mat[row + y][column + x];
                }
                idx += 1;
            }

            column += 6;
        }
        row += 6;
    }

    mat
}

/// Mask function nb°**6**
const fn mask_6<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    #[rustfmt::skip]
    const OFFSETS: [(usize, usize); 12] = [
        (1, 1), (1, 2), (2, 1), (2, 3),
        (2, 4), (3, 2), (3, 4), (4, 2),
        (4, 3), (4, 5), (5, 4), (5, 5)
    ];

    let mut row = 0;
    while row < N {
        let mut column = 0;
        while column < N {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
            if !mat_full[column][row] && (row % 6 != 0 || column % 6 != 0) {
                mat[column][row] = !mat[column][row];
            }
            column += 1;
        }

        row += 6;
    }

    let mut row = 0;
    while row < N {
        let mut column = 0;
        while column < N {
            let mut idx = 0;
            while idx < OFFSETS.len() {
                let (y, x) = OFFSETS[idx];
                if row + y < N && column + x < N && !mat_full[row + y][column + x] {
                    mat[row + y][column + x] = !mat[row + y][column + x];
                }
                idx += 1;
            }

            column += 6;
        }
        row += 6;
    }

    mat
}

/// Mask function nb°**7**
const fn mask_7<const N: usize>(
    mut mat: [[bool; N]; N],
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    let mut row = 0;
    while row < N {
        let mut column = row;
        while column < N {
            if !mat_full[row][column] && (((row + column) % 2) + ((row * column) % 3)) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
            if column != row
                && !mat_full[column][row]
                && (((row + column) % 2) + ((row * column) % 3)) % 2 == 0
            {
                mat[column][row] = !mat[column][row];
            }

            column += 1;
        }

        row += 1;
    }

    mat
}

/// Applies the function at `mask_nb` on `mat`
pub const fn mask<const N: usize>(
    mat: [[bool; N]; N],
    mask_nb: usize,
    mat_full: &[[bool; N]; N],
) -> [[bool; N]; N] {
    match mask_nb {
        0 => mask_0(mat, mat_full),
        1 => mask_1(mat, mat_full),
        2 => mask_2(mat, mat_full),
        3 => mask_3(mat, mat_full),
        4 => mask_4(mat, mat_full),
        5 => mask_5(mat, mat_full),
        6 => mask_6(mat, mat_full),
        7 => mask_7(mat, mat_full),
        _ => mat,
    }
}
