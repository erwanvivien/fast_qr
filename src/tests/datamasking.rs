use crate::datamasking::Mask;
use crate::module::Module;
use crate::QRCode;

const F: bool = false;
const T: bool = true;

#[test]
fn mask_checkerboard_test() {
    let mat = QRCode::DEFAULT;
    let mut mat = QRCode {
        size: 10,
        data: mat,
        mode: None,
        mask: None,
        ecl: None,
        version: None,
    };
    crate::datamasking::mask(&mut mat, Mask::Checkerboard);

    #[rustfmt::skip]
    let mat_bool = [
        &mat[0][..10], &mat[1][..10], &mat[2][..10], &mat[3][..10], &mat[4][..10],
        &mat[5][..10], &mat[6][..10], &mat[7][..10], &mat[8][..10], &mat[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        mat_bool,
        [
            [T, F, T, F, T, F, T, F, T, F],
            [F, T, F, T, F, T, F, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [F, T, F, T, F, T, F, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [F, T, F, T, F, T, F, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [F, T, F, T, F, T, F, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [F, T, F, T, F, T, F, T, F, T],
        ]
    );
}
#[test]
fn mask_horizontal_test() {
    let mat = QRCode::DEFAULT;
    let mut mat = QRCode {
        size: 10,
        data: mat,
        mode: None,
        mask: None,
        ecl: None,
        version: None,
    };
    crate::datamasking::mask(&mut mat, Mask::HorizontalLines);

    #[rustfmt::skip]
    let mat_bool = [
        &mat[0][..10], &mat[1][..10], &mat[2][..10], &mat[3][..10], &mat[4][..10],
        &mat[5][..10], &mat[6][..10], &mat[7][..10], &mat[8][..10], &mat[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        mat_bool,
        [
            [T, T, T, T, T, T, T, T, T, T],
            [F, F, F, F, F, F, F, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [F, F, F, F, F, F, F, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [F, F, F, F, F, F, F, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [F, F, F, F, F, F, F, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [F, F, F, F, F, F, F, F, F, F],
        ]
    );
}
#[test]
fn mask_vertical_test() {
    let mat = QRCode::DEFAULT;
    let mut mat = QRCode {
        size: 10,
        data: mat,
        mode: None,
        mask: None,
        ecl: None,
        version: None,
    };
    crate::datamasking::mask(&mut mat, Mask::VerticalLines);

    #[rustfmt::skip]
    let mat_bool = [
        &mat[0][..10], &mat[1][..10], &mat[2][..10], &mat[3][..10], &mat[4][..10],
        &mat[5][..10], &mat[6][..10], &mat[7][..10], &mat[8][..10], &mat[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        mat_bool,
        [
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, T, F, F, T, F, F, T],
        ]
    );
}
#[test]
fn mask_diagonal_test() {
    let mat = QRCode::DEFAULT;
    let mut mat = QRCode {
        size: 10,
        data: mat,
        mode: None,
        mask: None,
        ecl: None,
        version: None,
    };
    crate::datamasking::mask(&mut mat, Mask::DiagonalLines);

    #[rustfmt::skip]
    let mat_bool = [
        &mat[0][..10], &mat[1][..10], &mat[2][..10], &mat[3][..10], &mat[4][..10],
        &mat[5][..10], &mat[6][..10], &mat[7][..10], &mat[8][..10], &mat[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        mat_bool,
        [
            [T, F, F, T, F, F, T, F, F, T],
            [F, F, T, F, F, T, F, F, T, F],
            [F, T, F, F, T, F, F, T, F, F],
            [T, F, F, T, F, F, T, F, F, T],
            [F, F, T, F, F, T, F, F, T, F],
            [F, T, F, F, T, F, F, T, F, F],
            [T, F, F, T, F, F, T, F, F, T],
            [F, F, T, F, F, T, F, F, T, F],
            [F, T, F, F, T, F, F, T, F, F],
            [T, F, F, T, F, F, T, F, F, T],
        ]
    );
}
#[test]
fn mask_large_checkerboard_test() {
    let mat = QRCode::DEFAULT;
    let mut mat = QRCode {
        size: 10,
        data: mat,
        mode: None,
        mask: None,
        ecl: None,
        version: None,
    };
    crate::datamasking::mask(&mut mat, Mask::LargeCheckerboard);

    #[rustfmt::skip]
    let mat_bool = [
        &mat[0][..10], &mat[1][..10], &mat[2][..10], &mat[3][..10], &mat[4][..10],
        &mat[5][..10], &mat[6][..10], &mat[7][..10], &mat[8][..10], &mat[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        mat_bool,
        [
            [T, T, T, F, F, F, T, T, T, F],
            [T, T, T, F, F, F, T, T, T, F],
            [F, F, F, T, T, T, F, F, F, T],
            [F, F, F, T, T, T, F, F, F, T],
            [T, T, T, F, F, F, T, T, T, F],
            [T, T, T, F, F, F, T, T, T, F],
            [F, F, F, T, T, T, F, F, F, T],
            [F, F, F, T, T, T, F, F, F, T],
            [T, T, T, F, F, F, T, T, T, F],
            [T, T, T, F, F, F, T, T, T, F],
        ]
    );
}
#[test]
fn mask_field_test() {
    let mat = QRCode::DEFAULT;
    let mut mat = QRCode {
        size: 10,
        data: mat,
        mode: None,
        mask: None,
        ecl: None,
        version: None,
    };
    crate::datamasking::mask(&mut mat, Mask::Fields);

    #[rustfmt::skip]
    let mat_bool = [
        &mat[0][..10], &mat[1][..10], &mat[2][..10], &mat[3][..10], &mat[4][..10],
        &mat[5][..10], &mat[6][..10], &mat[7][..10], &mat[8][..10], &mat[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        mat_bool,
        [
            [T, T, T, T, T, T, T, T, T, T],
            [T, F, F, F, F, F, T, F, F, F],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, F, F, F, F, T, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [T, F, F, F, F, F, T, F, F, F],
            [T, F, F, T, F, F, T, F, F, T],
            [T, F, T, F, T, F, T, F, T, F],
        ]
    );
}
#[test]
fn mask_diamond_test() {
    let mat = QRCode::DEFAULT;
    let mut mat = QRCode {
        size: 10,
        data: mat,
        mode: None,
        mask: None,
        ecl: None,
        version: None,
    };
    crate::datamasking::mask(&mut mat, Mask::Diamonds);

    #[rustfmt::skip]
    let mat_bool = [
        &mat[0][..10], &mat[1][..10], &mat[2][..10], &mat[3][..10], &mat[4][..10],
        &mat[5][..10], &mat[6][..10], &mat[7][..10], &mat[8][..10], &mat[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        mat_bool,
        [
            [T, T, T, T, T, T, T, T, T, T],
            [T, T, T, F, F, F, T, T, T, F],
            [T, T, F, T, T, F, T, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
            [T, F, T, T, F, T, T, F, T, T],
            [T, F, F, F, T, T, T, F, F, F],
            [T, T, T, T, T, T, T, T, T, T],
            [T, T, T, F, F, F, T, T, T, F],
            [T, T, F, T, T, F, T, T, F, T],
            [T, F, T, F, T, F, T, F, T, F],
        ]
    );
}

#[test]
fn mask_meadow_test() {
    let mat = QRCode::DEFAULT;
    let mut mat = QRCode {
        size: 10,
        data: mat,
        mode: None,
        mask: None,
        ecl: None,
        version: None,
    };

    crate::datamasking::mask(&mut mat, Mask::Meadow);

    #[rustfmt::skip]
    let mat_bool = [
        &mat[0][..10], &mat[1][..10], &mat[2][..10], &mat[3][..10], &mat[4][..10],
        &mat[5][..10], &mat[6][..10], &mat[7][..10], &mat[8][..10], &mat[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        mat_bool,
        [
            [T, F, T, F, T, F, T, F, T, F],
            [F, F, F, T, T, T, F, F, F, T],
            [T, F, F, F, T, T, T, F, F, F],
            [F, T, F, T, F, T, F, T, F, T],
            [T, T, T, F, F, F, T, T, T, F],
            [F, T, T, T, F, F, F, T, T, T],
            [T, F, T, F, T, F, T, F, T, F],
            [F, F, F, T, T, T, F, F, F, T],
            [T, F, F, F, T, T, T, F, F, F],
            [F, T, F, T, F, T, F, T, F, T],
        ]
    );
}
