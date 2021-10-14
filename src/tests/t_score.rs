#[rustfmt::skip]
const MAT_HELLO_WORLD: [[bool; 21]; 21] = [
    [true, true, true, true, true, true, true, false, true, true, false, false, false, false, true, true, true, true, true, true, true,],
    [true, false, false, false, false, false, true, false, true, false, false, true, false, false, true, false, false, false, false, false, true,],
    [true, false, true, true, true, false, true, false, true, false, false, true, true, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, true, false, false, false, false, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, true, false, true, false, false, false, true, false, true, true, true, false, true,],
    [true, false, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, true,],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true,],
    [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false,],
    [false, true, false, true, true, false, true, true, false, false, false, false, true, false, true, false, true, true, true, true, true,],
    [false, true, false, false, false, false, false, false, true, true, true, true, false, false, false, false, true, false, false, false, true,],
    [false, false, true, true, false, true, true, true, false, true, true, false, false, false, true, false, true, true, false, false, false,],
    [false, true, true, false, true, true, false, true, false, false, true, true, false, true, false, true, false, true, true, true, false,],
    [true, false, false, false, true, false, true, false, true, false, true, true, true, false, true, true, true, false, true, false, true,],
    [false, false, false, false, false, false, false, false, true, true, false, true, false, false, true, false, false, false, true, false, true,],
    [true, true, true, true, true, true, true, true, false, false, true, false, false, false, false, true, false, true, true, false, false,],
    [true, false, false, false, false, false, true, false, false, true, false, true, true, false, true, true, false, true, false, false, false,],
    [true, false, true, true, true, false, true, false, true, false, true, false, false, false, true, true, true, true, true, true, true,],
    [true, false, true, true, true, false, true, false, false, true, false, true, false, true, false, true, false, false, false, true, false,],
    [true, false, true, true, true, false, true, false, true, false, false, true, false, true, true, true, false, true, false, false, true,],
    [true, false, false, false, false, false, true, false, true, false, true, true, true, true, false, false, false, true, false, true, true,],
    [true, true, true, true, true, true, true, false, false, false, false, true, false, true, true, true, false, false, false, false, true,],
];

#[rustfmt::skip]
#[test]
fn score_total_hello() {
    // matrix_score_rows_test
    // matrix_score_lines_test
    // matrix_score_pattern_test

    assert_eq!(crate::score::test_matrix_pattern_and_line(&MAT_HELLO_WORLD),
        255
    );

    assert_eq!(
        crate::score::matrix_score(&MAT_HELLO_WORLD),
        0 + 80 + 90 + 13 + 6 + 0 + 0 + 0 + 6 + 10 + 10 + 3 + 6 + 0 + 4 + 0 + 12 + 5 + 6 + 0 + 0 + 0 + 3 + 5 + 10 + 6 + 0 + 3 + 0 + 6 + 10 + 16 + 3 + 4 + 0 + 0 + 0 + 6 + 6 + 3 + 5 + 0 + 0 + 3 + 5
    );
}

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
    assert_eq!(crate::score::matrix_score(&MAT_VAHAN_DEV), 441);
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
    assert_eq!(crate::score::matrix_score(&MAT_URL_DOESNT_EXIST), 586);
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
    assert_eq!(crate::score::matrix_score(&MAT_VTF), 572);
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
    assert_eq!(crate::score::matrix_score(&MAT_VERY_LONG_TEST), 567);
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
    assert_eq!(crate::score::matrix_score(&MAT_LONGER_TEST), 617);
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
    assert_eq!(crate::score::matrix_score(&MAT_A), 343);
}

#[test]
fn score_line_only_false() {
    let tmp = [false; 60];
    assert_eq!(crate::score::test_score_line(&tmp), 58);
}

#[test]
fn score_line_only_false_one_true() {
    let mut tmp = [false; 60];
    tmp[30] = true;
    assert_eq!(crate::score::test_score_line(&tmp), 55);
}

#[test]
fn score_line_12_first() {
    let mut tmp = [false; 12];
    tmp[0] = true;
    assert_eq!(crate::score::test_score_line(&tmp), 9);
}

#[test]
fn score_line_12_last() {
    let mut tmp = [false; 12];
    tmp[11] = true;
    assert_eq!(crate::score::test_score_line(&tmp), 9);
}

#[test]
fn score_line_12_second() {
    let mut tmp = [false; 12];
    tmp[1] = true;
    assert_eq!(crate::score::test_score_line(&tmp), 8);
}

#[test]
fn score_line_12_third() {
    let mut tmp = [false; 12];
    tmp[2] = true;
    assert_eq!(crate::score::test_score_line(&tmp), 7);
}
