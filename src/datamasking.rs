//! Contains the HEIGHT functions that can alter QRCode
#![deny(unsafe_code)]
#![warn(missing_docs)]

/// Mask function nb°0
fn mask_0(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in ((row & 1)..mat[0].len()).step_by(2) {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°1
fn mask_1(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in (0..mat.len()).step_by(2) {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°2
fn mask_2(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in (0..mat[0].len()).step_by(3) {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°3
fn mask_3(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in ((3 - row % 3) % 3..mat[0].len()).step_by(3) {
            if !mat_full[row][column] {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°4
fn mask_4(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
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
}

/// Mask function nb°5
fn mask_5(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
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
}

/// Mask function nb°6
fn mask_6(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
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
}

/// Mask function nb°7
fn mask_7(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (((row + column) % 2) + ((row * column) % 3)) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Contains the HEIGHT functions for easy indexing
const MASKS: [for<'r, 's> fn(&'r mut Vec<Vec<bool>>, &'s Vec<Vec<bool>>); 8] = [
    mask_0, mask_1, mask_2, mask_3, mask_4, mask_5, mask_6, mask_7,
];

/// Applies the function at `mask_nb` on `mat`
pub fn mask(mat: &mut Vec<Vec<bool>>, mask_nb: u8, mat_full: &Vec<Vec<bool>>) {
    MASKS[mask_nb as usize](mat, &mat_full);
}
