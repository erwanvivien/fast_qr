//! Contains the HEIGHT functions that can alter QRCode
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::module::{Matrix, Module, ModuleType};

#[cfg(test)]
mod test;

/// Mask function nb°**0**
fn mask_0<const N: usize>(mat: &mut Matrix<N>) {
    for row in 0..N {
        for column in (row & 1..N).step_by(2) {
            if mat[row][column].module_type() == ModuleType::Data {
                mat[row][column].toggle();
            }
        }
    }
}

/// Mask function nb°**1**
fn mask_1<const N: usize>(mat: &mut Matrix<N>) {
    for row in (0..N).step_by(2) {
        for column in 0..N {
            if mat[row][column].module_type() == ModuleType::Data {
                mat[row][column].toggle();
            }
        }
    }
}

/// Mask function nb°**2**
fn mask_2<const N: usize>(mat: &mut Matrix<N>) {
    for row in 0..N {
        for column in (0..N).step_by(3) {
            if mat[row][column].module_type() == ModuleType::Data {
                mat[row][column].toggle();
            }
        }
    }
}

/// Mask function nb°**3**
fn mask_3<const N: usize>(mat: &mut Matrix<N>) {
    for row in 0..N {
        let start = (3 - row % 3) % 3;
        for column in (start..N).step_by(3) {
            if mat[row][column].module_type() == ModuleType::Data {
                mat[row][column].toggle();
            }
        }
    }
}

/// Mask function nb°**4**
fn mask_4<const N: usize>(mat: &mut Matrix<N>) {
    for row in 0..N {
        let start = ((row >> 1) & 1) * 3; // ((row / 2) % 2) * 3;
        for column in (start..N).step_by(6) {
            for i in column..std::cmp::min(N, column + 3) {
                if mat[row][i].module_type() == ModuleType::Data {
                    mat[row][i].toggle();
                }
            }
        }
    }
}

fn mask_5_6<const N: usize>(mat: &mut Matrix<N>, offset: &[(usize, usize)]) {
    for row in (0..N).step_by(6) {
        for column in 0..N {
            if mat[row][column].module_type() == ModuleType::Data {
                mat[row][column].toggle();
            }
            if mat[column][row].module_type() == ModuleType::Data
                && (row % 6 != 0 || column % 6 != 0)
            {
                mat[column][row].toggle();
            }
        }
    }

    for row in (0..N).step_by(6) {
        for column in (0..N).step_by(6) {
            for (y, x) in offset {
                if row + y < N
                    && column + x < N
                    && mat[row + y][column + x].module_type() == ModuleType::Data
                {
                    mat[row + y][column + x].toggle();
                }
            }
        }
    }
}

/// Mask function nb°**5**
fn mask_5<const N: usize>(mat: &mut Matrix<N>) {
    const OFFSETS: [(usize, usize); 4] = [(2, 3), (3, 2), (3, 4), (4, 3)];
    mask_5_6(mat, &OFFSETS);
}

/// Mask function nb°**6**
fn mask_6<const N: usize>(mat: &mut Matrix<N>) {
    #[rustfmt::skip]
    const OFFSETS: [(usize, usize); 12] = [
        (1, 1), (1, 2), (2, 1), (2, 3),
        (2, 4), (3, 2), (3, 4), (4, 2),
        (4, 3), (4, 5), (5, 4), (5, 5)
    ];
    mask_5_6(mat, &OFFSETS);
}

/// Mask function nb°**7**
fn mask_7<const N: usize>(mat: &mut Matrix<N>) {
    for row in 0..N {
        for column in row..N {
            if mat[row][column].module_type() == ModuleType::Data
                && (((row + column) % 2) + ((row * column) % 3)) % 2 == 0
            {
                mat[row][column].toggle();
            }
            if column != row
                && mat[column][row].module_type() == ModuleType::Data
                && (((row + column) % 2) + ((row * column) % 3)) % 2 == 0
            {
                mat[column][row].toggle();
            }
        }
    }
}

/// Applies the function at `mask_nb` on `mat`
pub fn mask<const N: usize>(mat: &mut Matrix<N>, mask_nb: usize) {
    match mask_nb {
        0 => mask_0(mat),
        1 => mask_1(mat),
        2 => mask_2(mat),
        3 => mask_3(mat),
        4 => mask_4(mat),
        5 => mask_5(mat),
        6 => mask_6(mat),
        7 => mask_7(mat),
        _ => panic!("Mask nb should never be {}", mask_nb),
    }
}
