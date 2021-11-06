#[rustfmt::skip]
const MAT_VAHAN_DEV: [[bool; 25]; 25] = [
    [true, true, true, true, true, true, true, false, true, false, false, true, false, false,false, false, false, false, true, true, true, true, true, true, true,],
    [true, false, false, false, false, false, true, false, false, false, true, true, true,false, true, false, true, false, true, false, false, false, false, false, true,],
    [true, false, true, true, true, false, true, false, false, true, true, false, false, false,true, true, false, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, true, true, true, false, false, true,true, false, true, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, false, false, true, true, false, true,false, false, false, false, true, false, true, true, true, false, true,],
    [true, false, false, false, false, false, true, false, false, true, false, true, false,true, true, false, true, false, true, false, false, false, false, false, true,],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false,true, false, true, false, true, true, true, true, true, true, true,],
    [false, false, false, false, false, false, false, false, false, true, true, false, false,true, true, true, true, false, false, false, false, false, false, false, false,],
    [true, true, false, true, false, false, true, false, true, false, true, false, true, false,false, true, false, true, false, false, false, true, false, false, true,],
    [false, true, false, true, true, false, false, true, true, false, false, true, false, true,false, true, false, false, false, false, true, false, false, true, true,],
    [true, true, false, true, true, true, true, true, false, false, true, false, true, false,false, false, false, true, true, false, true, true, true, true, true,],
    [true, false, false, true, false, true, false, true, false, false, false, false, false,true, true, true, false, false, false, true, false, false, false, true, true,],
    [false, false, true, true, false, false, true, true, false, true, true, false, false, false,false, false, true, false, true, true, false, false, true, false, false,],
    [false, false, false, false, false, true, false, false, true, true, true, true, true, false,true, true, true, true, true, false, true, false, false, true, true,],
    [true, false, true, true, false, true, true, false, false, false, true, false, true, false,false, true, true, false, false, false, true, false, false, false, false,],
    [false, true, true, true, true, false, false, true, false, true, true, true, false, true,true, false, true, false, true, true, false, true, false, false, false,],
    [true, false, false, true, true, true, true, false, true, true, false, false, true, false,false, true, true, true, true, true, true, true, false, false, true,],
    [false, false, false, false, false, false, false, false, true, false, false, false, false,false, true, true, true, false, false, false, true, false, true, false, true,],
    [true, true, true, true, true, true, true, false, false, false, false, true, false, true,false, false, true, false, true, false, true, true, true, false, true,],
    [true, false, false, false, false, false, true, false, true, false, true, true, true, false,true, true, true, false, false, false, true, false, true, false, false,],
    [true, false, true, true, true, false, true, false, true, false, false, true, false, false,false, false, true, true, true, true, true, true, true, false, false,],
    [true, false, true, true, true, false, true, false, false, true, false, false, true, false,true, true, false, false, true, true, true, false, false, true, false,],
    [true, false, true, true, true, false, true, false, true, false, false, true, false, false,false, false, true, true, true, true, true, true, true, false, true,],
    [true, false, false, false, false, false, true, false, false, true, false, false, true,false, false, true, true, true, true, false, false, false, false, true, true,],
    [true, true, true, true, true, true, true, false, false, false, true, false, false, false,true, true, false, true, true, true, false, true, false, false, true,],
];

#[test]
fn score_total_vahan_dev() {
    assert_eq!(
        crate::comptime::qrcode::score::matrix_score(&MAT_VAHAN_DEV),
        441
    );
}

#[rustfmt::skip]
const MAT_URL_DOESNT_EXIST: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, false, true, false, true, false, false, false, true, true, true, true, false, true, false, true, true, true, true, true, true, true,],
    [true, false, false, false, false, false, true, false, false, true, true, false, false, true, true, true, true, false, true, true, true, false, true, false, false, false, false, false, true,],
    [true, false, true, true, true, false, true, false, false, false, true, false, true, false, true, false, false, false, true, false, true, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, false, false, true, true, true, false, false, false, false, true, false, true, false, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, true, true, true, false, true, true, true, false, false, true, false, true, false, false, true, false, true, true, true, false, true,],
    [true, false, false, false, false, false, true, false, false, false, true, false, false, false, true, true, true, false, false, true, false, false, true, false, false, false, false, false, true,],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true,],
    [false, false, false, false, false, false, false, false, true, true, true, false, false, true, true, false, true, true, true, false, false, false, false, false, false, false, false, false, false,],
    [false, false, true, true, false, false, true, true, true, false, false, false, false, false, false, true, false, false, false, true, true, true, true, false, true, false, false, false, false,],
    [false, false, true, true, true, true, false, false, true, true, true, true, true, false, false, true, true, true, false, false, false, false, true, true, true, true, true, true, true,],
    [true, false, true, true, false, false, true, true, false, false, false, false, true, false, false, true, true, false, false, false, false, true, false, true, false, false, false, true, false,],
    [false, true, false, false, false, true, false, true, true, false, true, true, true, false, true, true, false, true, true, true, false, true, true, false, true, false, false, false, true,],
    [true, true, true, true, false, false, true, false, true, true, true, false, false, false, true, false, false, false, false, false, false, false, false, true, false, true, true, false, false,],
    [true, false, true, false, false, true, false, false, true, true, false, false, false, false, false, true, true, false, false, true, false, true, true, false, false, false, false, false, true,],
    [false, false, false, true, true, false, true, false, false, false, false, false, true, true, true, false, false, false, true, true, true, false, true, true, true, true, false, true, true,],
    [false, true, true, true, false, false, false, true, true, false, true, true, true, true, true, false, true, false, false, true, true, false, true, true, false, true, false, false, true,],
    [false, true, false, false, true, true, true, true, false, false, true, false, false, false, true, false, false, true, true, true, true, true, false, false, true, false, false, false, false,],
    [false, true, false, true, true, true, false, true, false, true, true, false, false, true, true, true, true, true, true, true, true, true, false, true, false, false, false, true, false,],
    [true, false, true, true, true, false, true, true, true, false, true, false, true, true, true, true, false, true, false, true, false, true, false, true, true, false, false, false, false,],
    [false, false, true, true, true, true, false, true, true, true, true, true, true, false, true, true, false, true, true, true, false, false, true, false, false, true, true, false, false,],
    [false, true, true, false, false, false, true, true, true, true, true, true, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, true, true,],
    [false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, true, true, true, true, true, false, false, false, true, false, true, false, true,],
    [true, true, true, true, true, true, true, true, false, false, true, false, true, true, false, false, true, false, true, false, true, false, true, false, true, true, true, true, false,],
    [true, false, false, false, false, false, true, false, false, false, false, true, false, false, true, true, false, true, false, false, true, false, false, false, true, false, false, true, true,],
    [true, false, true, true, true, false, true, false, false, false, false, true, true, false, true, true, false, false, true, false, true, true, true, true, true, true, true, false, false,],
    [true, false, true, true, true, false, true, false, true, false, true, false, false, false, false, true, false, true, true, false, true, true, false, false, false, true, false, true, false,],
    [true, false, true, true, true, false, true, false, true, true, true, false, true, true, false, false, false, true, true, false, true, false, false, true, false, false, true, false, true,],
    [true, false, false, false, false, false, true, false, false, true, false, false, false, true, true, false, false, false, true, false, false, false, false, true, false, true, false, true, false,],
    [true, true, true, true, true, true, true, false, false, true, false, true, true, false, true, false, false, false, true, false, true, false, false, true, false, false, false, true, false,],
];

#[test]
fn score_total_url_doesnt_exist() {
    assert_eq!(
        crate::comptime::qrcode::score::matrix_score(&MAT_URL_DOESNT_EXIST),
        586
    );
}

#[rustfmt::skip]
const MAT_VTF: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, true, false, true, false, true, true, false, false, false, false, true, false, true, false, true, true, true, true, true, true, true,],
    [true, false, false, false, false, false, true, false, true, false, true, true, true, false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, true,],
    [true, false, true, true, true, false, true, false, true, true, true, false, true, false, false, false, false, false, false, true, true, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, false, true, true, true, true, true, false, true, true, true, true, true, false, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, false, true, false, true, true, true, false, false, false, false, true, true, true, false, true, false, true, true, true, false, true,],
    [true, false, false, false, false, false, true, false, true, false, false, true, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, true,],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true,],
    [false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, false,],
    [false, true, true, true, false, false, true, true, true, true, true, false, false, false, false, true, true, false, true, false, true, true, false, true, false, false, true, true, true,],
    [false, true, false, false, true, true, false, false, false, false, true, true, true, true, true, true, false, false, false, false, false, true, false, false, true, true, true, true, true,],
    [true, true, true, false, false, false, true, true, false, true, false, false, true, false, false, false, false, true, false, false, true, true, false, true, true, true, true, false, false,],
    [true, true, false, true, false, true, false, true, false, false, true, true, true, false, true, false, false, false, false, false, false, true, false, true, true, true, false, true, true,],
    [true, false, true, false, true, false, true, true, false, false, true, false, false, true, true, false, false, true, false, false, true, false, false, false, false, true, true, false, false,],
    [false, false, false, true, false, true, false, true, true, false, true, true, false, false, false, false, false, false, false, true, true, true, true, false, true, false, false, true, true,],
    [true, false, true, false, false, true, true, true, false, true, true, false, true, true, false, false, true, false, true, false, true, false, true, true, true, true, false, false, false,],
    [true, false, false, false, true, false, false, false, false, false, true, true, false, false, false, true, false, true, false, false, false, true, true, true, true, false, false, true, true,],
    [false, false, false, false, true, true, true, false, true, false, true, true, false, false, true, true, true, true, true, true, false, false, true, false, false, false, true, false, false,],
    [true, true, false, false, true, false, false, false, true, false, true, true, true, false, false, false, true, false, false, false, true, true, false, true, true, true, false, false, true,],
    [true, false, true, false, true, false, true, false, false, false, true, false, true, true, false, false, false, true, true, true, false, false, false, false, false, true, false, false, false,],
    [true, false, true, true, false, false, false, true, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, false, true, true, false, true, true,],
    [true, false, false, true, false, false, true, true, true, true, true, true, true, false, false, false, true, true, false, true, true, true, true, true, true, true, true, true, false,],
    [false, false, false, false, false, false, false, false, true, true, true, false, false, false, true, true, true, true, false, false, true, false, false, false, true, false, false, true, true,],
    [true, true, true, true, true, true, true, true, false, false, false, true, true, false, true, false, false, true, true, true, true, false, true, false, true, false, false, false, false,],
    [true, false, false, false, false, false, true, false, false, true, true, true, false, false, true, false, false, false, false, true, true, false, false, false, true, true, false, true, false,],
    [true, false, true, true, true, false, true, false, true, false, true, true, true, true, false, false, true, true, false, false, true, true, true, true, true, true, true, true, false,],
    [true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, true, false, false, false,],
    [true, false, true, true, true, false, true, false, true, true, false, true, false, true, false, true, false, true, true, false, false, true, true, true, true, false, false, true, false,],
    [true, false, false, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, true, true, false, true, false, false, true, true, false, true, false,],
    [true, true, true, true, true, true, true, false, false, true, false, true, false, false, false, true, false, true, false, false, false, false, false, true, true, true, true, false, false,],
];

#[test]
fn score_total_vtf() {
    assert_eq!(crate::comptime::qrcode::score::matrix_score(&MAT_VTF), 572);
}

#[rustfmt::skip]
const MAT_VERY_LONG_TEST: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, false, true, false, true, true, false, true, true, true, true, true, true, true, false, true, true, true, true, true, true, true,],
    [true, false, false, false, false, false, true, false, true, true, true, true, false, true, false, true, true, false, true, false, false, false, true, false, false, false, false, false, true,],
    [true, false, true, true, true, false, true, false, true, true, true, true, true, false, true, false, true, false, true, true, true, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, true, false, false, true, false, true, false, false, true, false, true, false, true, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, true, false, false, true, false, true, true, false, false, false, true, false, true, false, true, false, true, true, true, false, true,],
    [true, false, false, false, false, false, true, false, true, true, false, false, false, false, true, false, false, true, true, false, true, false, true, false, false, false, false, false, true,],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true,],
    [false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, true, false, true, true, false, false, false, false, false, false, false, false,],
    [true, false, false, true, false, false, true, false, true, true, true, true, false, false, true, true, true, true, false, true, true, true, true, true, true, true, true, true, false,],
    [true, false, false, true, false, false, false, true, false, false, true, false, true, true, true, true, false, true, false, false, false, true, false, true, false, false, true, false, true,],
    [true, false, false, false, true, false, true, true, false, true, true, true, false, false, true, false, false, true, false, false, true, true, true, false, true, true, true, true, false,],
    [false, true, false, true, true, true, false, false, false, false, false, false, false, false, true, true, true, false, false, false, true, true, false, false, false, false, true, false, true,],
    [false, true, true, true, true, false, true, false, false, true, false, false, false, true, true, true, true, false, false, true, true, false, false, true, true, false, false, true, true,],
    [false, true, false, false, true, true, false, true, false, true, true, false, false, false, true, false, false, true, true, false, false, false, true, false, true, true, false, false, true,],
    [true, false, true, true, false, true, true, false, true, false, true, true, false, true, true, false, false, true, false, false, false, false, false, true, true, true, false, false, false,],
    [false, false, false, false, true, true, false, true, true, true, false, true, true, false, false, true, false, false, true, true, false, false, true, false, true, false, true, true, false,],
    [true, true, true, false, true, true, true, false, true, true, true, true, false, false, true, true, false, true, false, true, true, true, false, true, true, true, true, false, true,],
    [false, false, false, false, true, true, false, true, true, true, false, false, true, false, true, true, false, false, false, false, false, true, false, true, false, false, false, true, false,],
    [true, true, true, true, false, true, true, false, false, true, true, false, true, false, true, false, true, false, false, false, true, false, true, false, false, true, true, false, true,],
    [false, false, false, true, false, true, false, false, true, false, false, false, true, false, true, true, false, false, true, true, true, false, true, false, true, true, false, false, false,],
    [true, true, false, true, true, false, true, false, false, true, false, false, true, false, false, true, false, false, false, false, true, true, true, true, true, true, false, true, true,],
    [false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, true, false, true, true, true, true, false, false, false, true, true, true, true, true,],
    [true, true, true, true, true, true, true, false, false, false, false, false, false, true, true, false, false, true, true, false, true, false, true, false, true, false, true, false, false,],
    [true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, true, false, true, false, false,],
    [true, false, true, true, true, false, true, false, false, true, true, false, true, false, true, false, false, false, false, true, true, true, true, true, true, false, true, false, true,],
    [true, false, true, true, true, false, true, false, false, false, true, true, true, true, true, false, false, true, false, false, false, false, true, false, true, false, false, true, false,],
    [true, false, true, true, true, false, true, false, true, true, false, true, false, true, false, false, false, true, false, true, true, false, false, true, false, true, true, true, true,],
    [true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, true, true, false, true, false, true, false, true, true, false, true,],
    [true, true, true, true, true, true, true, false, false, false, false, false, false, true, true, true, false, true, false, true, false, true, true, true, false, false, false, true, true,],
];

#[test]
fn score_total_very_long_test() {
    assert_eq!(
        crate::comptime::qrcode::score::matrix_score(&MAT_VERY_LONG_TEST),
        567
    );
}

#[rustfmt::skip]
const MAT_LONGER_TEST: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, false, false, false, true, false, true, false, false, false, false, true, true, true, false, true, true, true, true, true, true, true,],
    [true, false, false, false, false, false, true, false, true, true, false, true, true, false, true, false, true, false, false, true, true, false, true, false, false, false, false, false, true,],
    [true, false, true, true, true, false, true, false, false, true, true, false, false, true, false, false, true, false, false, false, false, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, false, true, true, true, false, true, false, true, false, true, true, true, true, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, false, true, true, true, false, false, true, false, false, true, true, false, true, false, true, false, true, true, true, false, true,],
    [true, false, false, false, false, false, true, false, true, true, false, true, false, true, false, false, true, true, false, true, false, false, true, false, false, false, false, false, true,],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true,],
    [false, false, false, false, false, false, false, false, true, false, true, false, true, true, true, true, false, false, false, true, true, false, false, false, false, false, false, false, false,],
    [true, true, false, false, false, false, true, true, false, true, true, false, true, false, true, false, true, false, false, true, true, false, true, true, false, false, false, true, false,],
    [true, false, true, false, false, true, false, false, true, false, false, false, true, true, false, false, false, true, true, true, false, true, false, true, true, true, false, false, true,],
    [false, false, true, false, true, false, true, true, true, true, false, true, true, false, false, false, true, false, false, true, false, true, true, false, false, false, false, true, false,],
    [true, false, true, true, false, true, false, true, false, true, false, true, false, true, false, true, true, false, true, false, true, false, true, true, false, false, true, true, false,],
    [true, false, false, false, false, false, true, true, false, false, true, false, false, false, true, false, false, false, false, true, true, true, true, false, true, false, false, false, false,],
    [false, false, true, false, false, true, false, true, true, false, true, true, false, false, true, true, true, true, false, true, true, false, true, false, false, false, true, false, true,],
    [true, false, false, true, false, false, true, true, true, true, false, true, true, true, true, false, false, true, true, true, true, false, false, true, false, false, true, false, false,],
    [true, false, false, false, false, true, false, true, false, false, false, true, false, false, true, true, true, false, true, true, false, true, false, true, true, false, true, false, true,],
    [true, true, true, false, false, false, true, true, true, false, false, false, false, true, true, true, false, true, false, true, true, false, true, false, true, true, true, true, false,],
    [true, false, false, true, true, true, false, true, false, false, true, true, false, false, true, true, false, false, true, true, true, true, false, true, true, true, true, true, false,],
    [false, false, false, false, false, true, true, false, false, false, true, true, false, true, false, false, true, true, false, true, false, false, true, false, true, false, false, false, false,],
    [false, false, true, true, false, false, false, true, false, true, false, false, true, false, false, true, false, false, false, true, true, true, false, true, true, true, false, true, true,],
    [true, true, true, true, false, true, true, true, true, false, true, false, true, false, false, true, false, true, true, false, true, true, true, true, true, true, false, false, false,],
    [false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, true, false, true, false, false, false, true, false, false, true, true,],
    [true, true, true, true, true, true, true, true, false, false, false, true, false, false, true, true, true, true, false, true, true, false, true, false, true, true, false, false, false,],
    [true, false, false, false, false, false, true, false, true, true, false, true, false, false, true, true, true, false, true, false, true, false, false, false, true, false, true, true, true,],
    [true, false, true, true, true, false, true, false, true, false, true, true, false, true, false, false, true, false, true, true, true, true, true, true, true, false, true, true, false,],
    [true, false, true, true, true, false, true, false, false, true, false, false, true, false, false, true, false, false, true, true, true, false, true, false, false, true, true, true, false,],
    [true, false, true, true, true, false, true, false, false, false, true, false, true, true, true, true, true, true, true, false, false, false, false, true, true, false, false, true, true,],
    [true, false, false, false, false, false, true, false, false, false, true, true, false, false, true, true, true, false, true, true, false, false, true, false, false, true, true, true, false,],
    [true, true, true, true, true, true, true, false, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false,],
];

#[test]
fn score_total_longer_test() {
    assert_eq!(
        crate::comptime::qrcode::score::matrix_score(&MAT_LONGER_TEST),
        617
    );
}

#[rustfmt::skip]
const MAT_A: [[bool; 21]; 21] = [
    [true, true, true, true, true, true, true, false, false, false, true, true, false, false,true, true, true, true, true, true, true,],
    [true, false, false, false, false, false, true, false, true, false, false, false, true,false, true, false, false, false, false, false, true,],
    [true, false, true, true, true, false, true, false, false, false, true, true, true, false,true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, false, false, true, true, false, false,true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, false, true, false, false, true, false,true, false, true, true, true, false, true,],
    [true, false, false, false, false, false, true, false, true, false, false, false, true,false, true, false, false, false, false, false, true,],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false,true, true, true, true, true, true, true,],
    [false, false, false, false, false, false, false, false, true, true, true, true, false,false, false, false, false, false, false, false, false,],
    [true, true, false, false, false, false, true, true, false, false, false, false, false,false, true, true, false, false, false, true, false,],
    [false, true, true, true, false, false, false, true, true, false, false, false, true, false,false, true, false, true, false, true, true,],
    [false, false, true, false, false, true, true, false, false, true, false, true, true, false,true, true, true, true, true, false, false,],
    [true, false, false, true, true, true, false, true, false, true, true, false, false, true,false, false, true, false, true, false, false,],
    [false, true, false, true, false, false, true, true, false, true, true, false, true, false,false, false, false, false, true, false, true,],
    [false, false, false, false, false, false, false, false, true, false, false, true, true,false, false, false, false, false, true, false, true,],
    [true, true, true, true, true, true, true, true, false, true, false, false, true, false,true, false, false, false, false, false, true,],
    [true, false, false, false, false, false, true, false, true, false, false, true, false,false, true, true, false, true, false, false, false,],
    [true, false, true, true, true, false, true, false, true, true, false, false, false, false,false, true, true, true, true, true, true,],
    [true, false, true, true, true, false, true, false, false, false, true, true, true, false,false, true, false, true, false, true, true,],
    [true, false, true, true, true, false, true, false, false, true, true, false, false, false,false, true, true, true, true, false, false,],
    [true, false, false, false, false, false, true, false, false, true, false, true, true,false, false, false, true, false, true, false, true,],
    [true, true, true, true, true, true, true, false, false, true, true, false, true, true,true, false, false, false, true, true, true,],
];

#[test]
fn score_total_a() {
    assert_eq!(crate::comptime::qrcode::score::matrix_score(&MAT_A), 343);
}

#[test]
fn score_line_only_false() {
    let tmp = [false; 60];
    assert_eq!(crate::comptime::qrcode::score::test_score_line(&tmp), 58);
}

#[test]
fn score_line_only_false_one_true() {
    let mut tmp = [false; 60];
    tmp[30] = true;
    assert_eq!(crate::comptime::qrcode::score::test_score_line(&tmp), 55);
}

#[test]
fn score_line_12_first() {
    let mut tmp = [false; 12];
    tmp[0] = true;
    assert_eq!(crate::comptime::qrcode::score::test_score_line(&tmp), 9);
}

#[test]
fn score_line_12_last() {
    let mut tmp = [false; 12];
    tmp[11] = true;
    assert_eq!(crate::comptime::qrcode::score::test_score_line(&tmp), 9);
}

#[test]
fn score_line_12_second() {
    let mut tmp = [false; 12];
    tmp[1] = true;
    assert_eq!(crate::comptime::qrcode::score::test_score_line(&tmp), 8);
}

#[test]
fn score_line_12_third() {
    let mut tmp = [false; 12];
    tmp[2] = true;
    assert_eq!(crate::comptime::qrcode::score::test_score_line(&tmp), 7);
}

#[test]
fn score_square_1() {
    let tmp = [[false; 2]; 2];
    assert_eq!(
        crate::comptime::qrcode::score::test_matrix_score_squares(&tmp),
        3
    );
}

#[test]
fn score_square_4() {
    let tmp = [[false; 3]; 3];
    assert_eq!(
        crate::comptime::qrcode::score::test_matrix_score_squares(&tmp),
        3 * 4
    );
}

#[test]
fn score_square_0() {
    let mut tmp = [[false; 3]; 3];
    tmp[1][1] = true;
    assert_eq!(
        crate::comptime::qrcode::score::test_matrix_score_squares(&tmp),
        0
    );
}

#[test]
fn score_square_2() {
    let mut tmp = [[false; 3]; 3];
    tmp[1][0] = true;
    assert_eq!(
        crate::comptime::qrcode::score::test_matrix_score_squares(&tmp),
        2 * 3
    );
}

// #[rustfmt::skip]
// const VAHAN_DEV: [[bool; 29]; 29] = [
//     [true, true, true, true, true, true, true, false, false, true, false, false, true, false,false, false, false, true, true, true, false, false, true, true, true, true, true, true,true,],
//     [true, false, false, false, false, false, true, false, true, true, true, true, false, true,true, false, true, false, true, false, true, false, true, false, false, false, false,false, true,],
//     [true, false, true, true, true, false, true, false, false, true, false, true, false, false,true, false, false, true, false, true, true, false, true, false, true, true, true, false,true,],
//     [true, false, true, true, true, false, true, false, false, false, true, false, true, false,false, false, false, false, true, true, false, false, true, false, true, true, true, false,true,],
//     [true, false, true, true, true, false, true, false, false, true, false, false, true, false,true, false, true, false, true, false, false, false, true, false, true, true, true, false,true,],
//     [true, false, false, false, false, false, true, false, true, true, true, false, true, true,true, false, true, false, true, true, false, false, true, false, false, false, false,false, true,],
//     [true, true, true, true, true, true, true, false, true, false, true, false, true, false,true, false, true, false, true, false, true, false, true, true, true, true, true, true,true,],
//     [false, false, false, false, false, false, false, false, true, false, false, true, false,true, true, false, false, true, true, false, false, false, false, false, false, false,false, false, false,],
//     [true, true, false, false, false, false, true, true, false, true, false, true, true, false,true, false, false, false, true, true, false, false, true, true, false, false, false, true,false,],
//     [false, false, true, false, true, false, false, true, true, true, false, true, false, false,false, true, true, true, false, true, true, false, false, false, true, false, false, true,true,],
//     [true, true, true, true, false, true, true, false, false, false, true, true, false, true,false, true, false, false, true, false, true, true, false, false, true, false, true, false,true,],
//     [true, false, true, true, true, true, false, true, false, false, false, true, false, false,false, false, false, false, false, false, false, true, false, false, true, true, false,true, true,],
//     [true, true, true, false, true, true, true, false, true, true, false, true, false, true,false, true, true, false, true, true, true, false, false, true, false, true, false, false,true,],
//     [false, false, false, true, true, false, false, false, true, false, true, false, false,false, true, true, false, false, false, false, false, false, true, false, true, false,true, false, true,],
//     [true, false, true, true, false, false, true, false, false, true, true, false, false, true,true, true, false, false, false, false, false, false, false, false, true, false, false,true, true,],
//     [false, false, true, true, false, true, false, false, true, false, false, true, true, false,false, false, true, false, false, true, true, true, false, true, false, true, false, true,false,],
//     [false, true, true, false, false, false, true, true, true, true, true, false, false, false,false, true, false, false, false, true, true, true, true, false, false, false, false, true,true,],
//     [true, true, false, true, false, false, false, true, false, false, true, false, true, false,true, true, false, true, false, true, false, false, false, false, true, false, true, false,true,],
//     [false, false, true, true, true, true, true, true, true, true, false, true, false, false,true, false, false, false, false, true, false, false, false, true, true, false, false,false, true,],
//     [false, false, true, false, false, false, false, true, false, true, false, false, false,false, false, false, false, false, false, true, true, false, false, false, false, true,false, true, false,],
//     [true, true, true, false, false, false, true, false, true, false, false, false, false,false, false, false, true, true, true, false, true, true, true, true, true, true, false,false, false,],
//     [false, false, false, false, false, false, false, false, true, true, false, false, false,false, true, false, true, true, true, false, true, false, false, false, true, false, true,false, true,],
//     [true, true, true, true, true, true, true, true, false, true, false, false, false, false,false, false, false, true, true, true, true, false, true, false, true, false, false, false,true,],
//     [true, false, false, false, false, false, true, false, true, true, false, true, false,false, true, true, true, true, false, true, true, false, false, false, true, true, false,false, true,],
//     [true, false, true, true, true, false, true, false, true, true, true, true, false, true,true, false, true, true, false, true, true, true, true, true, true, false, false, false,false,],
//     [true, false, true, true, true, false, true, false, false, false, true, false, true, false,false, true, false, false, true, false, false, false, false, true, false, false, true,true, false,],
//     [true, false, true, true, true, false, true, false, false, true, true, false, true, true,false, true, false, false, false, true, true, true, false, false, false, true, false, true,true,],
//     [true, false, false, false, false, false, true, false, false, true, false, false, true,false, false, false, false, false, true, true, false, true, true, true, false, true, false,true, true,],
//     [true, true, true, true, true, true, true, false, false, false, true, true, true, false,true, true, false, true, true, true, true, true, false, false, true, false, false, true,false,],
// ];

// #[test]
// fn score_vahan_dev() {
//     assert_eq!(crate::score::matrix_score(&VAHAN_DEV), 608);
// }

#[rustfmt::skip]
const HELLO_WORLD_MASK0: [[bool; 21]; 21] = [
    [true, true, true, true, true, true, true, false, true, true, false, false, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, true, false, false, true, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, false, false, false, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, false, true, false, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, true, true, false, true, false, true, true, false, false, false, false, true, false, true, false, true, true, true, true, true],
    [false, true, false, false, false, false, false, false, true, true, true, true, false, false, false, false, true, false, false, false, true],
    [false, false, true, true, false, true, true, true, false, true, true, false, false, false, true, false, true, true, false, false, false],
    [false, true, true, false, true, true, false, true, false, false, true, true, false, true, false, true, false, true, true, true, false],
    [true, false, false, false, true, false, true, false, true, false, true, true, true, false, true, true, true, false, true, false, true],
    [false, false, false, false, false, false, false, false, true, true, false, true, false, false, true, false, false, false, true, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, false, false, false, true, false, true, true, false, false],
    [true, false, false, false, false, false, true, false, false, true, false, true, true, false, true, true, false, true, false, false, false],
    [true, false, true, true, true, false, true, false, true, false, true, false, false, false, true, true, true, true, true, true, true],
    [true, false, true, true, true, false, true, false, false, true, false, true, false, true, false, true, false, false, false, true, false],
    [true, false, true, true, true, false, true, false, true, false, false, false, true, true, true, true, false, true, false, false, true],
    [true, false, false, false, false, false, true, false, true, false, true, true, false, true, false, false, false, true, false, true, true],
    [true, true, true, true, true, true, true, false, false, false, false, false, true, true, true, true, false, false, false, false, true],
];

#[test]
fn hello_world_mask0() {
    use crate::comptime::qrcode::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK0;

    assert_eq!(test_matrix_line(&mat) + test_matrix_col(&mat), 180);
    assert_eq!(test_matrix_score_squares(&mat), 90);
    assert_eq!(test_matrix_pattern(&mat), 80);
    assert_eq!(test_matrix_dark_modules(&mat), 0);
}

#[rustfmt::skip]
const HELLO_WORLD_MASK1: [[bool; 21];21] = [
    [true, true, true, true, true, true, true, false, false, false, false, true, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, false, true, false, false, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, true, false, true, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, true, true, true, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, true, true, true, true, false, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false],
    [false, true, true, false, false, false, true, false, false, true, false, true, true, false, true, true, false, true, false, false, false],
    [false, false, false, true, false, true, false, true, true, false, true, false, false, true, false, true, true, true, false, true, true],
    [false, true, true, false, false, false, true, false, false, false, true, true, false, true, true, true, true, false, false, true, false],
    [false, false, true, true, true, false, false, false, false, true, true, false, false, false, false, false, false, false, true, false, false],
    [true, true, false, true, true, true, true, true, true, true, true, false, true, true, true, false, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, true, false, true, true, true, true],
    [true, true, true, true, true, true, true, false, false, true, true, true, false, true, false, false, false, false, true, true, false],
    [true, false, false, false, false, false, true, false, false, false, false, false, true, true, true, false, false, false, false, true, false],
    [true, false, true, true, true, false, true, false, false, true, true, true, false, true, true, false, true, false, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false],
    [true, false, true, true, true, false, true, false, true, true, false, true, true, false, true, false, false, false, false, true, true],
    [true, false, false, false, false, false, true, false, true, true, true, false, false, false, false, true, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, false, true, false, true, true, false, true, false, false, true, false, true, true]
];

#[test]
fn hello_world_mask1() {
    use crate::comptime::qrcode::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK1;

    assert_eq!(test_matrix_line(&mat) + test_matrix_col(&mat), 172);
    assert_eq!(test_matrix_score_squares(&mat), 129);
    assert_eq!(test_matrix_pattern(&mat), 120);
    assert_eq!(test_matrix_dark_modules(&mat), 0);
}

#[rustfmt::skip]
const HELLO_WORLD_MASK2: [[bool; 21];21] = [
    [true, true, true, true, true, true, true, false, true, false, true, false, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, false, true, true, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, true, false, true, true, true, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false],
    [false, true, true, true, true, true, true, true, false, true, true, false, true, false, false, true, true, false, false, false, true],
    [true, false, false, false, false, true, false, true, true, true, true, false, true, true, false, false, true, true, true, true, true],
    [false, false, false, false, true, true, true, true, true, false, false, false, false, false, false, true, false, true, false, false, true],
    [true, false, true, false, true, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false],
    [true, false, true, true, false, false, true, false, false, true, false, true, true, false, false, false, false, false, true, false, false],
    [false, false, false, false, false, false, false, false, true, true, false, false, true, true, true, false, false, true, false, true, true],
    [true, true, true, true, true, true, true, false, true, true, false, false, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, true, true, false, false, false, true, true, true, false, false, true, true, false],
    [true, false, true, true, true, false, true, false, true, true, false, false, false, false, false, false, false, true, true, true, false],
    [true, false, true, true, true, false, true, false, true, true, false, false, true, false, false, true, false, true, true, false, false],
    [true, false, true, true, true, false, true, false, true, true, true, false, true, true, false, false, true, true, false, false, false],
    [true, false, false, false, false, false, true, false, true, false, true, false, true, false, false, false, false, false, true, false, true],
    [true, true, true, true, true, true, true, false, false, true, true, false, true, true, false, false, true, false, false, false, false]
];

#[test]
fn hello_world_mask2() {
    use crate::comptime::qrcode::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK2;

    assert_eq!(test_matrix_line(&mat) + test_matrix_col(&mat), 206);
    assert_eq!(test_matrix_score_squares(&mat), 141);
    assert_eq!(test_matrix_pattern(&mat), 160);
    assert_eq!(test_matrix_dark_modules(&mat), 0);
}

#[rustfmt::skip]
const HELLO_WORLD_MASK3: [[bool; 21];21] = [
    [true, true, true, true, true, true, true, false, false, false, true, false, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, true, true, false, true, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, false, false, true, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, false, true, false, true, false, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false],
    [false, true, true, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true, true, false],
    [true, false, false, false, false, true, false, true, true, true, true, false, true, true, false, false, true, true, true, true, true],
    [true, false, true, true, true, false, true, true, false, true, false, true, true, false, true, false, false, false, true, false, false],
    [false, true, true, true, false, false, false, true, false, true, false, false, false, true, false, false, true, false, true, true, false],
    [true, false, true, true, false, false, true, false, false, true, false, true, true, false, false, false, false, false, true, false, false],
    [false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, true, false, false, true, true, false],
    [true, true, true, true, true, true, true, false, false, false, true, false, true, true, true, true, false, true, false, true, true],
    [true, false, false, false, false, false, true, false, true, true, false, false, false, true, true, true, false, false, true, true, false],
    [true, false, true, true, true, false, true, false, false, false, false, true, true, false, true, true, false, false, false, true, true],
    [true, false, true, true, true, false, true, false, true, false, true, false, false, true, false, false, true, true, false, true, false],
    [true, false, true, true, true, false, true, false, true, true, true, false, true, true, false, false, true, true, false, false, false],
    [true, false, false, false, false, false, true, false, true, true, true, true, false, false, true, true, false, true, false, false, false],
    [true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, true, false, false, true, true, false]
];

#[test]
fn hello_world_mask3() {
    use crate::comptime::qrcode::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK3;

    assert_eq!(test_matrix_line(&mat) + test_matrix_col(&mat), 180);
    assert_eq!(test_matrix_score_squares(&mat), 141);
    assert_eq!(test_matrix_pattern(&mat), 120);
    assert_eq!(test_matrix_dark_modules(&mat), 0);
}

#[rustfmt::skip]
const HELLO_WORLD_MASK4: [[bool; 21];21] = [
    [true, true, true, true, true, true, true, false, false, true, true, false, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, false, true, false, false, true, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, true, false, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, false, false, false, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, true, true, true, true, true, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false],
    [false, true, false, false, true, false, true, false, true, false, true, false, true, true, false, true, true, false, true, false, false],
    [true, true, true, true, false, true, false, false, false, false, true, false, true, false, true, true, true, true, true, false, false],
    [true, false, false, false, false, false, true, true, true, false, true, true, true, false, false, true, true, false, true, false, true],
    [false, false, true, false, false, true, false, false, false, false, false, true, false, false, false, true, true, true, true, false, false],
    [true, true, false, false, false, false, true, true, true, false, false, true, true, true, true, true, false, false, true, true, true],
    [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, true, false, false, false],
    [true, true, true, true, true, true, true, false, false, true, true, true, true, false, true, false, false, false, false, false, true],
    [true, false, false, false, false, false, true, false, false, true, true, true, true, true, true, true, true, true, false, true, false],
    [true, false, true, true, true, false, true, false, true, false, false, false, false, true, true, true, false, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, false, false, true, true, true, false, false, true, true, true, true],
    [true, false, true, true, true, false, true, false, false, true, false, true, false, true, false, false, false, false, true, false, false],
    [true, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, true, true, false, false, true],
    [true, true, true, true, true, true, true, false, false, false, true, false, true, false, true, true, true, false, false, true, true]
];

#[test]
fn hello_world_mask4() {
    use crate::comptime::qrcode::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK4;

    assert_eq!(test_matrix_line(&mat) + test_matrix_col(&mat), 195);
    assert_eq!(test_matrix_score_squares(&mat), 138);
    assert_eq!(test_matrix_pattern(&mat), 200);
    assert_eq!(test_matrix_dark_modules(&mat), 0);
}

#[rustfmt::skip]
const HELLO_WORLD_MASK5: [[bool; 21];21] = [
    [true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, true, true, false, false, true, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, false, true, true, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, true, true, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, true, false, false, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, false, true, true, true, true, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false, false, false],
    [false, true, false, false, false, false, true, true, true, true, true, false, true, true, false, false, false, false, false, true, true],
    [true, false, true, true, true, true, false, true, false, false, false, false, true, true, true, true, false, true, true, true, false],
    [false, false, false, false, true, true, true, true, true, false, false, false, false, false, false, true, false, true, false, false, true],
    [true, false, true, true, true, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false],
    [true, true, false, true, true, true, true, true, true, true, true, false, true, true, true, false, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, false, false, false, true, true, true, true, false, true, false, true, true],
    [true, true, true, true, true, true, true, false, true, true, false, false, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, false, false, true, false, false, true, false, false, true, false, true, true, true],
    [true, false, true, true, true, false, true, false, false, true, false, false, false, false, false, false, false, true, true, true, false],
    [true, false, true, true, true, false, true, false, false, false, false, false, true, false, false, false, false, true, true, false, false],
    [true, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false, false, true, true],
    [true, false, false, false, false, false, true, false, true, true, true, false, true, false, false, true, false, false, true, false, true],
    [true, true, true, true, true, true, true, false, false, true, true, false, true, true, false, false, true, false, false, false, false]
];

#[test]
fn hello_world_mask5() {
    use crate::comptime::qrcode::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK5;

    assert_eq!(test_matrix_line(&mat) + test_matrix_col(&mat), 189);
    assert_eq!(test_matrix_score_squares(&mat), 156);
    assert_eq!(test_matrix_pattern(&mat), 200);
    assert_eq!(test_matrix_dark_modules(&mat), 0);
}

#[rustfmt::skip]
const HELLO_WORLD_MASK6: [[bool; 21];21] = [
    [true, true, true, true, true, true, true, false, false, false, false, true, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, true, true, false, false, true, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, true, true, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, true, false, true, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, false, true, false, false, true, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, true, false, true, true, false, false, false, false, false, false, false, false],
    [false, true, false, true, true, true, true, false, true, true, false, false, true, true, true, false, true, true, false, true, false],
    [true, false, true, true, true, true, false, true, false, false, false, false, true, true, true, true, false, true, true, true, false],
    [false, false, true, false, true, false, true, true, false, false, false, true, false, false, true, true, false, false, false, false, false],
    [true, false, true, true, false, true, false, false, false, true, false, true, true, false, false, false, true, true, false, false, false],
    [true, true, false, true, true, true, true, true, true, true, true, false, true, true, true, false, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, true, false, false, false],
    [true, true, true, true, true, true, true, false, false, true, true, false, false, true, true, false, false, true, true, true, true],
    [true, false, false, false, false, false, true, false, true, false, true, false, false, true, false, false, true, false, true, true, true],
    [true, false, true, true, true, false, true, false, true, true, false, true, false, false, true, false, false, false, true, true, true],
    [true, false, true, true, true, false, true, false, true, false, true, true, true, false, false, false, true, false, true, false, false],
    [true, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false, false, true, true],
    [true, false, false, false, false, false, true, false, true, true, true, false, true, true, true, true, false, false, true, true, false],
    [true, true, true, true, true, true, true, false, false, true, false, false, true, false, false, false, false, false, false, true, false]
];

#[test]
fn hello_world_mask6() {
    use crate::comptime::qrcode::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK6;

    assert_eq!(test_matrix_line(&mat) + test_matrix_col(&mat), 171);
    assert_eq!(test_matrix_score_squares(&mat), 102);
    assert_eq!(test_matrix_pattern(&mat), 80);
    assert_eq!(test_matrix_dark_modules(&mat), 0);
}

#[rustfmt::skip]
const HELLO_WORLD_MASK7: [[bool; 21];21] = [
    [true, true, true, true, true, true, true, false, true, true, false, false, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, false, false, true, true, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, false, false, false, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, false, false, false, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, false, false, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, true, false, true, true, false, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false],
    [false, true, false, true, false, true, true, true, true, false, false, true, true, true, true, true, false, true, true, false, true],
    [false, true, false, false, false, false, false, false, true, true, true, true, false, false, false, false, true, false, false, false, true],
    [false, true, true, true, true, true, true, false, false, true, false, false, false, true, true, false, false, true, false, true, false],
    [false, true, false, false, true, false, false, true, true, false, true, false, false, true, true, true, false, false, true, true, true],
    [true, false, false, false, true, false, true, false, true, false, true, true, true, false, true, true, true, false, true, false, true],
    [false, false, false, false, false, false, false, false, true, true, true, true, false, true, true, false, true, false, true, true, true],
    [true, true, true, true, true, true, true, false, true, false, true, true, false, false, true, true, false, false, true, false, true],
    [true, false, false, false, false, false, true, false, true, true, false, true, true, false, true, true, false, true, false, false, false],
    [true, false, true, true, true, false, true, false, false, false, false, false, false, true, true, true, false, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, true, false, false, false, true, true, true, false, true, false, true, true],
    [true, false, true, true, true, false, true, false, false, false, false, false, true, true, true, true, false, true, false, false, true],
    [true, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, true, true, false, false, true],
    [true, true, true, true, true, true, true, false, false, false, false, true, true, true, false, true, false, true, false, false, false]
];

#[test]
fn hello_world_mask7() {
    use crate::comptime::qrcode::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK7;

    assert_eq!(test_matrix_line(&mat) + test_matrix_col(&mat), 197);
    assert_eq!(test_matrix_score_squares(&mat), 123);
    assert_eq!(test_matrix_pattern(&mat), 200);
    assert_eq!(test_matrix_dark_modules(&mat), 0);
}
