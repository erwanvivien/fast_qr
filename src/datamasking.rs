//! Contains the HEIGHT functions that can alter `QRCode`
#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::module::ModuleType;
use crate::QRCode;

/// The different mask patterns. The mask pattern should only be applied to
/// the data and error correction portion of the QR code.
#[derive(Debug, Copy, Clone)]
pub enum Mask {
    /// QR code pattern n°0: `(x + y) % 2 == 0`.
    Checkerboard = 0,
    /// QR code pattern n°1: `y % 2 == 0`.
    HorizontalLines = 1,
    /// QR code pattern n°2: `x % 3 == 0`.
    VerticalLines = 2,
    /// QR code pattern n°3: `(x + y) % 3 == 0`.
    DiagonalLines = 3,
    /// QR code pattern n°4: `((x/3) + (y/2)) % 2 == 0`.
    LargeCheckerboard = 4,
    /// QR code pattern n°5: `(x*y)%2 + (x*y)%3 == 0`.
    Fields = 5,
    /// QR code pattern n°6: `((x*y)%2 + (x*y)%3) % 2 == 0`.
    Diamonds = 6,
    /// QR code pattern n°7: `((x+y)%2 + (x*y)%3) % 2 == 0`.
    Meadow = 7,
}

/// Mask function nb°**0**, `Mask::Checkerboard`.
fn mask_checkerboard(qr: &mut QRCode) {
    for row in 0..qr.size {
        for column in (row & 1..qr.size).step_by(2) {
            let module = &mut qr[row][column];
            if module.module_type() == ModuleType::Data {
                module.toggle();
            }
        }
    }
}

/// Mask function nb°**1**, `Mask::HorizontalLines`.
fn mask_horizontal(qr: &mut QRCode) {
    for row in (0..qr.size).step_by(2) {
        for column in 0..qr.size {
            let module = &mut qr[row][column];
            if module.module_type() == ModuleType::Data {
                module.toggle();
            }
        }
    }
}

/// Mask function nb°**2**, `Mask::VerticalLines`.
fn mask_vertical(qr: &mut QRCode) {
    for row in 0..qr.size {
        for column in (0..qr.size).step_by(3) {
            let module = &mut qr[row][column];
            if module.module_type() == ModuleType::Data {
                module.toggle();
            }
        }
    }
}

/// Mask function nb°**3**, `Mask::DiagonalLines`.
fn mask_diagonal(qr: &mut QRCode) {
    for row in 0..qr.size {
        let start = (3 - row % 3) % 3;
        for column in (start..qr.size).step_by(3) {
            let module = &mut qr[row][column];
            if module.module_type() == ModuleType::Data {
                module.toggle();
            }
        }
    }
}

/// Mask function nb°**4**, `Mask::LargeCheckerboard`.
fn mask_large_checkerboard(qr: &mut QRCode) {
    for row in 0..qr.size {
        let start = ((row >> 1) & 1) * 3; // ((row / 2) % 2) * 3;
        for column in (start..qr.size).step_by(6) {
            for i in column..core::cmp::min(qr.size, column + 3) {
                let module = &mut qr[row][i];
                if module.module_type() == ModuleType::Data {
                    module.toggle();
                }
            }
        }
    }
}

fn mask_5_6(qr: &mut QRCode, offset: &[(usize, usize)]) {
    for row in (0..qr.size).step_by(6) {
        for column in 0..qr.size {
            let module = &mut qr[row][column];
            if module.module_type() == ModuleType::Data {
                module.toggle();
            }
            let module = &mut qr[column][row];
            if module.module_type() == ModuleType::Data && (row % 6 != 0 || column % 6 != 0) {
                module.toggle();
            }
        }
    }

    for row in (0..qr.size).step_by(6) {
        for column in (0..qr.size).step_by(6) {
            for (y, x) in offset {
                if row + y >= qr.size || column + x >= qr.size {
                    continue;
                }

                let module = &mut qr[row + y][column + x];
                if module.module_type() == ModuleType::Data {
                    module.toggle();
                }
            }
        }
    }
}

/// Mask function nb°**5**, `Mask::Fields`.
fn mask_field(qr: &mut QRCode) {
    const OFFSETS: [(usize, usize); 4] = [(2, 3), (3, 2), (3, 4), (4, 3)];
    mask_5_6(qr, &OFFSETS);
}

/// Mask function nb°**6**, `Mask::Diamonds`.
fn mask_diamond(qr: &mut QRCode) {
    #[rustfmt::skip]
    const OFFSETS: [(usize, usize); 12] = [
        (1, 1), (1, 2), (2, 1), (2, 3),
        (2, 4), (3, 2), (3, 4), (4, 2),
        (4, 3), (4, 5), (5, 4), (5, 5)
    ];
    mask_5_6(qr, &OFFSETS);
}

/// Mask function nb°**7**, `Mask::Meadow`.
fn mask_meadow(qr: &mut QRCode) {
    for row in 0..qr.size {
        for column in row..qr.size {
            if (((row + column) % 2) + ((row * column) % 3)) % 2 != 0 {
                continue;
            }

            let module = &mut qr[row][column];
            if module.module_type() == ModuleType::Data {
                module.toggle();
            }

            let module = &mut qr[column][row];
            if column != row && module.module_type() == ModuleType::Data {
                module.toggle();
            }
        }
    }
}

/// Applies the function at `mask_nb` on `mat`
pub fn mask(qr: &mut QRCode, mask: Mask) {
    match mask {
        Mask::Checkerboard => mask_checkerboard(qr),
        Mask::HorizontalLines => mask_horizontal(qr),
        Mask::VerticalLines => mask_vertical(qr),
        Mask::DiagonalLines => mask_diagonal(qr),
        Mask::LargeCheckerboard => mask_large_checkerboard(qr),
        Mask::Fields => mask_field(qr),
        Mask::Diamonds => mask_diamond(qr),
        Mask::Meadow => mask_meadow(qr),
    }
}
