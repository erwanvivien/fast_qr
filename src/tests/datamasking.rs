use crate::datamasking::Mask;
use crate::QRCode;

const F: bool = false;
const T: bool = true;

#[test]
fn mask_checkerboard_test() {
    let mut qr = QRCode::default(10);
    crate::datamasking::mask(&mut qr, Mask::Checkerboard);

    #[rustfmt::skip]
        let qr_bool = [
        &qr[0][..10], &qr[1][..10], &qr[2][..10], &qr[3][..10], &qr[4][..10],
        &qr[5][..10], &qr[6][..10], &qr[7][..10], &qr[8][..10], &qr[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        qr_bool,
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
    let mut qr = QRCode::default(10);
    crate::datamasking::mask(&mut qr, Mask::HorizontalLines);

    #[rustfmt::skip]
        let qr_bool = [
        &qr[0][..10], &qr[1][..10], &qr[2][..10], &qr[3][..10], &qr[4][..10],
        &qr[5][..10], &qr[6][..10], &qr[7][..10], &qr[8][..10], &qr[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        qr_bool,
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
    let mut qr = QRCode::default(10);
    crate::datamasking::mask(&mut qr, Mask::VerticalLines);

    #[rustfmt::skip]
        let qr_bool = [
        &qr[0][..10], &qr[1][..10], &qr[2][..10], &qr[3][..10], &qr[4][..10],
        &qr[5][..10], &qr[6][..10], &qr[7][..10], &qr[8][..10], &qr[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        qr_bool,
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
    let mut qr = QRCode::default(10);
    crate::datamasking::mask(&mut qr, Mask::DiagonalLines);

    #[rustfmt::skip]
        let qr_bool = [
        &qr[0][..10], &qr[1][..10], &qr[2][..10], &qr[3][..10], &qr[4][..10],
        &qr[5][..10], &qr[6][..10], &qr[7][..10], &qr[8][..10], &qr[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        qr_bool,
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
    let mut qr = QRCode::default(10);
    crate::datamasking::mask(&mut qr, Mask::LargeCheckerboard);

    #[rustfmt::skip]
        let qr_bool = [
        &qr[0][..10], &qr[1][..10], &qr[2][..10], &qr[3][..10], &qr[4][..10],
        &qr[5][..10], &qr[6][..10], &qr[7][..10], &qr[8][..10], &qr[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        qr_bool,
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
    let mut qr = QRCode::default(10);
    crate::datamasking::mask(&mut qr, Mask::Fields);

    #[rustfmt::skip]
        let qr_bool = [
        &qr[0][..10], &qr[1][..10], &qr[2][..10], &qr[3][..10], &qr[4][..10],
        &qr[5][..10], &qr[6][..10], &qr[7][..10], &qr[8][..10], &qr[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        qr_bool,
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
    let mut qr = QRCode::default(10);
    crate::datamasking::mask(&mut qr, Mask::Diamonds);

    #[rustfmt::skip]
        let qr_bool = [
        &qr[0][..10], &qr[1][..10], &qr[2][..10], &qr[3][..10], &qr[4][..10],
        &qr[5][..10], &qr[6][..10], &qr[7][..10], &qr[8][..10], &qr[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        qr_bool,
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
    let mut qr = QRCode::default(10);

    crate::datamasking::mask(&mut qr, Mask::Meadow);

    #[rustfmt::skip]
        let qr_bool = [
        &qr[0][..10], &qr[1][..10], &qr[2][..10], &qr[3][..10], &qr[4][..10],
        &qr[5][..10], &qr[6][..10], &qr[7][..10], &qr[8][..10], &qr[9][..10],
    ];

    #[rustfmt::skip]
    assert_eq!(
        qr_bool,
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
