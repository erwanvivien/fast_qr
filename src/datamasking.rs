//! Contains the HEIGHT functions that can alter QRCode

use crate::default;

/// Mask function nb°0
fn mask_0(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (row + column) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°1
fn mask_1(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (row) % 2 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°2
fn mask_2(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (column) % 3 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°3
fn mask_3(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (row + column) % 3 == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°4
fn mask_4(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column]
                && ((row as f64 / 2.).floor() + (column as f64 / 3.).floor()) as u32 % 2 == 0
            {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°5
fn mask_5(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && ((row * column) % 2) + ((row * column) % 3) == 0 {
                mat[row][column] = !mat[row][column];
            }
        }
    }
}

/// Mask function nb°6
fn mask_6(mat: &mut Vec<Vec<bool>>, mat_full: &Vec<Vec<bool>>) {
    for row in 0..mat.len() {
        for column in 0..mat[0].len() {
            if !mat_full[row][column] && (((row * column) % 2) + ((row * column) % 3)) % 2 == 0 {
                mat[row][column] = !mat[row][column];
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
pub fn mask(mat: &mut Vec<Vec<bool>>, mask_nb: u8) {
    let version = (mat.len() - 17) / 4;
    let mat_full = default::non_available_matrix_from_version(version);
    MASKS[mask_nb as usize](mat, &mat_full);
}
