use crate::module::{Matrix, Module};

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
    let mat = MAT_VAHAN_DEV.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::matrix_score(&mat), 441);
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
    let mat = MAT_URL_DOESNT_EXIST.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::matrix_score(&mat), 586);
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
    let mat = MAT_VTF.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::matrix_score(&mat), 572);
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
    let mat_very_long = MAT_VERY_LONG_TEST.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::matrix_score(&mat_very_long), 567);
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
    let mat_longer = MAT_LONGER_TEST.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::matrix_score(&mat_longer), 617);
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
    let mat_a = MAT_A.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::matrix_score(&mat_a), 343);
}

#[test]
fn score_line_only_false() {
    let tmp = [false; 60];
    assert_eq!(crate::score::test_score_line(&tmp.map(|x| x.into())), 58);
}

#[test]
fn score_line_only_false_one_true() {
    let mut tmp = [false; 60];
    tmp[30] = true;
    assert_eq!(crate::score::test_score_line(&tmp.map(|x| x.into())), 55);
}

#[test]
fn score_line_12_first() {
    let mut tmp = [false; 12];
    tmp[0] = true;
    assert_eq!(crate::score::test_score_line(&tmp.map(|x| x.into())), 9);
}

#[test]
fn score_line_20_full_sep() {
    let mut tmp = [false; 20];
    for i in (0..20).step_by(2) {
        tmp[i] = true;
    }
    assert_eq!(crate::score::test_score_line(&tmp.map(|x| x.into())), 0);
}

#[test]
fn score_line_20_sep() {
    let mut tmp = [false; 20];
    tmp[15] = true;
    // tmp = [
    //     false, false, false, false, false, false, false, false, false, false, false, false, false,
    //     false, false, true, false, false, false, false,
    // ];
    assert_eq!(crate::score::test_score_line(&tmp.map(|x| x.into())), 13);
}

#[test]
fn score_line_12_last() {
    let mut tmp = [false; 12];
    tmp[11] = true;
    assert_eq!(crate::score::test_score_line(&tmp.map(|x| x.into())), 9);
}

#[test]
fn score_line_12_second() {
    let mut tmp = [false; 12];
    tmp[1] = true;
    assert_eq!(crate::score::test_score_line(&tmp.map(|x| x.into())), 8);
}

#[test]
fn score_line_12_third() {
    let mut tmp = [false; 12];
    tmp[2] = true;
    assert_eq!(crate::score::test_score_line(&tmp.map(|x| x.into())), 7);
}

#[test]
fn pattern_test_first() {
    let mat = [
        true, false, true, true, true, false, true, false, false, false, false, false, false, false,
    ];
    assert_eq!(
        crate::score::test_score_pattern(&mat.map(|x| x.into())),
        1 * 40
    );
}

#[test]
fn pattern_test_end() {
    let mat = [
        false, false, false, true, false, true, true, true, false, true, false, false, false, false,
    ];
    assert_eq!(
        crate::score::test_score_pattern(&mat.map(|x| x.into())),
        1 * 40
    );
}

#[test]
fn pattern_test_double() {
    let mat = [
        true, false, true, true, true, false, true, false, false, false, false, false, false,
        false, false, true, false, true, true, true, false, true,
    ];
    assert_eq!(
        crate::score::test_score_pattern(&mat.map(|x| x.into())),
        2 * 40
    );
}

#[test]
fn pattern_test_4() {
    let mat = [
        false, true, true, false, true, true, false, true, false, false, true, true, true, true,
        true, false, false, false, false, false, true, true, false, true, true, true, false, true,
        false, false, true, true, true, true, false, false, false, true, true, false, false, false,
        false, true, true, true, true, false, false, false, true, true, true, false, true, false,
        false, false, true, false, false, true, true, true, true, false, false, false, true, true,
        true, true, false, true, false, true, false, false, true, false, true, true, true, true,
        false, true, false, false, false, false, false, true, false, false, false, false, false,
        false, true, true, true, false, true, false, true, true, false, true, true, true, false,
        true, false, false, false, false, false, false, false, false,
    ];
    assert_eq!(
        crate::score::test_score_pattern(&mat.map(|x| x.into())),
        1 * 40
    );
}

#[test]
fn pattern_test_35() {
    let mat = [
        false, true, false, true, true, true, false, true, true, true, false, true, false, true,
        true, false, true, false, true, true, true, true, true, false, false, false, false, false,
        false, false, true, false, false, true, true, true, false, true, true, false, true, true,
        false, true, false, true, true, true, true, false, false, true, true, false, true, false,
        false, false, false, true, true, false, true, false, true, false, true, false, false, true,
        true, true, true, true, true, true, true, true, false, false, false, false, true, false,
        false, false, false, false, false, false, true, true, false, true, true, true, false,
        false, false, false, false, true, false, true, true, true, false, false, false, false,
        true, true, true, false, true, true, true, true, true, false,
    ];
    assert_eq!(
        crate::score::test_score_pattern(&mat.map(|x| x.into())),
        0 * 40
    );
}

#[test]
fn score_square_1() {
    let mat = [[false; 2]; 2];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 3);
}

#[test]
fn score_square_4() {
    let mat = [[false; 3]; 3];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 3 * 4);
}

#[test]
fn score_square_0() {
    let mut mat = [[false; 3]; 3];
    mat[1][1] = true;
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 0);
}

#[test]
fn score_square_2() {
    let mut mat = [[false; 3]; 3];
    mat[1][0] = true;
    let mat = mat.map(|x| x.map(|x| x.into()));
    // mat = [
    //         false,
    //         false,
    //         false,
    //     ],
    //     [
    //         true,
    //         false,
    //         false,
    //     ],
    //     [
    //         false,
    //         false,
    //         false,
    //     ],
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 2 * 3);
}

#[test]
fn square_test_0() {
    let mat = [
        [
            true, true, false, false, false, true, false, true, false, true,
        ],
        [
            true, false, false, false, false, true, false, false, false, false,
        ],
        [
            false, true, true, true, false, false, false, false, true, true,
        ],
        [
            true, true, true, true, true, false, true, false, false, true,
        ],
        [
            false, true, false, false, false, true, true, true, true, true,
        ],
        [
            true, true, true, true, true, true, false, false, false, true,
        ],
        [
            false, false, false, false, false, false, false, false, false, true,
        ],
        [
            true, false, false, false, true, false, false, true, true, true,
        ],
        [true, true, false, true, true, true, true, true, true, true],
        [
            false, false, false, false, false, true, true, false, true, true,
        ],
    ];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 14 * 3);
}

#[test]
fn square_test_null() {
    let mat = [[true, true], [true, true]];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 1 * 3);
}

#[test]
fn square_test_1() {
    let mat = [
        [
            true, true, false, true, false, true, false, false, false, true,
        ],
        [
            false, true, false, false, true, true, true, true, false, false,
        ],
        [
            false, false, false, false, true, false, true, false, false, false,
        ],
        [false, true, true, true, true, true, true, false, true, true],
        [
            true, true, false, true, false, true, false, true, true, true,
        ],
        [
            true, true, true, false, false, true, false, false, true, false,
        ],
        [
            true, true, true, false, false, true, false, true, false, true,
        ],
        [
            true, true, true, true, true, false, false, false, false, true,
        ],
        [
            false, false, false, true, false, false, false, true, false, true,
        ],
        [
            true, true, true, true, false, false, true, false, false, false,
        ],
    ];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 11 * 3);
}

#[test]
fn square_test_2() {
    let mat = [
        [
            false, true, false, false, false, true, true, false, false, true,
        ],
        [
            false, true, false, false, false, false, true, true, false, true,
        ],
        [
            false, false, true, true, false, false, true, true, false, true,
        ],
        [
            true, true, false, true, true, true, true, false, false, true,
        ],
        [
            true, true, false, false, true, false, true, false, false, true,
        ],
        [
            true, false, false, true, true, false, true, true, false, false,
        ],
        [
            false, true, true, false, false, true, true, false, false, true,
        ],
        [
            false, false, true, false, true, false, false, false, false, true,
        ],
        [
            true, false, true, false, false, true, true, true, true, false,
        ],
        [
            true, true, false, true, false, true, false, true, false, false,
        ],
    ];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 7 * 3);
}

#[test]
fn square_test_3() {
    let mat = [
        [
            true, false, false, true, false, false, true, true, true, true,
        ],
        [
            true, false, false, false, true, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, true, true, true, true, false,
        ],
        [
            false, true, true, false, false, true, false, false, true, false,
        ],
        [
            true, true, false, false, true, true, false, false, true, false,
        ],
        [
            true, false, true, false, false, true, false, false, false, false,
        ],
        [
            false, true, true, true, false, true, false, true, false, true,
        ],
        [true, true, true, false, true, true, false, true, true, true],
        [
            true, false, true, true, true, true, true, true, false, false,
        ],
        [
            true, false, false, true, false, true, false, false, false, true,
        ],
    ];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 8 * 3);
}

#[test]
fn square_test_4() {
    let mat = [
        [
            false, false, true, false, true, false, false, true, false, true,
        ],
        [
            false, false, true, true, true, false, false, false, true, true,
        ],
        [
            true, false, false, false, true, false, false, true, false, false,
        ],
        [
            true, true, true, true, true, true, false, true, false, false,
        ],
        [
            true, false, true, false, true, true, false, false, true, true,
        ],
        [
            true, false, false, true, true, false, true, true, false, true,
        ],
        [
            false, false, true, true, true, true, true, false, true, false,
        ],
        [
            false, true, true, false, false, false, false, true, true, true,
        ],
        [
            false, false, true, true, false, false, true, false, true, true,
        ],
        [
            true, false, true, false, false, true, true, true, false, false,
        ],
    ];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 8 * 3);
}

#[test]
fn square_test_5() {
    let mat = [
        [
            true, true, true, true, true, false, false, false, false, false,
        ],
        [
            true, false, false, true, false, false, false, true, false, true,
        ],
        [
            false, true, true, true, false, false, false, true, false, true,
        ],
        [
            true, false, false, false, false, true, false, false, true, true,
        ],
        [
            true, false, false, true, true, false, false, false, true, true,
        ],
        [
            false, false, true, false, false, false, true, true, false, false,
        ],
        [
            false, false, true, true, false, true, true, false, true, false,
        ],
        [
            true, true, false, false, false, false, true, true, true, true,
        ],
        [
            true, false, false, true, true, true, true, true, false, true,
        ],
        [
            false, false, false, true, true, true, true, true, true, true,
        ],
    ];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 13 * 3);
}

#[test]
fn square_test_6() {
    let mat = [
        [
            false, true, true, true, true, true, true, false, false, false,
        ],
        [
            true, true, false, true, true, true, true, false, false, false,
        ],
        [
            false, true, false, true, true, true, false, true, false, false,
        ],
        [
            true, false, true, true, false, true, false, true, false, true,
        ],
        [
            false, false, false, true, true, true, false, false, true, true,
        ],
        [
            true, true, true, false, false, false, false, false, true, true,
        ],
        [
            false, false, true, false, false, true, true, false, true, true,
        ],
        [
            true, true, true, false, false, false, false, false, true, true,
        ],
        [
            true, false, true, true, false, false, true, false, true, false,
        ],
        [
            false, false, false, true, false, true, false, true, false, false,
        ],
    ];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 15 * 3);
}

#[test]
fn square_test_7() {
    let mat = [
        [
            true, true, false, true, true, false, true, false, true, false,
        ],
        [false, true, true, true, true, true, true, true, true, true],
        [
            false, true, false, false, true, true, true, true, false, true,
        ],
        [
            true, false, true, true, false, true, false, true, true, true,
        ],
        [
            false, true, true, true, true, false, true, true, false, false,
        ],
        [
            false, true, false, true, false, false, true, true, false, true,
        ],
        [
            false, true, true, false, true, true, true, false, false, true,
        ],
        [
            false, true, false, true, true, true, false, true, false, false,
        ],
        [
            true, false, false, false, true, false, false, false, true, false,
        ],
        [
            true, true, false, false, false, false, true, true, false, true,
        ],
    ];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 8 * 3);
}

#[test]
fn square_test_8() {
    let mat = [
        [
            false, true, false, false, false, false, false, true, false, true,
        ],
        [
            false, true, false, false, true, true, false, true, false, true,
        ],
        [
            false, true, true, true, false, true, true, false, true, true,
        ],
        [
            true, true, false, false, false, false, false, true, false, true,
        ],
        [true, true, false, true, true, true, true, true, true, true],
        [
            false, true, true, false, false, true, true, true, true, false,
        ],
        [
            true, false, false, false, true, false, false, false, false, true,
        ],
        [
            false, true, false, true, false, true, true, true, false, true,
        ],
        [
            true, true, true, false, true, false, true, false, false, false,
        ],
        [
            false, true, true, true, false, false, false, false, true, true,
        ],
    ];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 6 * 3);
}

#[test]
fn square_test_9() {
    let mat = [
        [
            true, false, false, true, true, true, true, false, true, true,
        ],
        [
            false, false, false, false, false, false, true, true, false, false,
        ],
        [
            false, false, false, true, false, true, true, true, false, true,
        ],
        [
            true, false, false, false, true, false, false, false, true, true,
        ],
        [
            false, true, true, true, false, true, true, true, false, false,
        ],
        [
            true, false, false, false, false, true, true, false, false, true,
        ],
        [
            false, true, false, false, false, true, true, false, false, false,
        ],
        [
            false, false, false, true, true, true, true, false, true, false,
        ],
        [
            true, false, false, true, false, false, true, true, false, false,
        ],
        [true, true, false, true, true, true, true, true, true, false],
    ];
    let mat = mat.map(|x| x.map(|x| x.into()));
    assert_eq!(crate::score::test_matrix_score_squares(&mat), 13 * 3);
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

// #[tests]
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
    use crate::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK0;
    let mat = mat.map(|x| x.map(|x| x.into()));

    crate::helpers::print_matrix_with_margin(&mat);

    assert_eq!(test_matrix_pattern(&mat), 80);
    assert_eq!(test_matrix_line(&mat) + test_matrix_col(&mat), 180);
    assert_eq!(test_matrix_score_squares(&mat), 90);
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
    use crate::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK1;
    let mat = mat.map(|x| x.map(|x| x.into()));

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
    use crate::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK2;
    let mat = mat.map(|x| x.map(|x| x.into()));

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
    use crate::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK3;
    let mat = mat.map(|x| x.map(|x| x.into()));

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
    use crate::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK4;
    let mat = mat.map(|x| x.map(|x| x.into()));

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
    use crate::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK5;
    let mat = mat.map(|x| x.map(|x| x.into()));

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
    use crate::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK6;
    let mat = mat.map(|x| x.map(|x| x.into()));

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
    use crate::score::{
        test_matrix_col, test_matrix_dark_modules, test_matrix_line, test_matrix_pattern,
        test_matrix_score_squares,
    };

    let mat = HELLO_WORLD_MASK7;
    let mat = mat.map(|x| x.map(|x| x.into()));

    assert_eq!(test_matrix_line(&mat) + test_matrix_col(&mat), 197);
    assert_eq!(test_matrix_score_squares(&mat), 123);
    assert_eq!(test_matrix_pattern(&mat), 200);
    assert_eq!(test_matrix_dark_modules(&mat), 0);
}
