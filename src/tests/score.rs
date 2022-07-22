use crate::default::create_mat_from_bool;
use crate::module::Module;
use crate::score::{
    test_matrix_pattern, test_matrix_pattern_and_line, test_matrix_score_squares, test_score_line,
    test_score_pattern,
};

#[rustfmt::skip]
const MAT_EXAMPLE_COM: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, true, true, false, false, false, true, false, true, false, false, true, false, true, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, true, true, true, false, false, true, false, true, false, false, true, true, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, false, true, false, true, false, true, true, false, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, true, false, true, false, false, true, false, false, true, false, true, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, false, false, true, true, true, true, true, false, true, true, true, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, true, true, true, true, true, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, true, false, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, true, true, true, false, true, false, true, false, false, false, false, false, false, false, true, false, true, true, true, true, true, true, false, false, true, true, true],
    [false, false, false, false, false, true, false, true, true, true, true, true, true, false, true, false, true, false, false, false, false, true, true, true, true, true, true, false, true],
    [true, false, true, true, false, true, true, false, true, true, true, true, true, true, true, true, true, false, true, false, true, true, true, true, false, false, false, false, false],
    [true, false, true, false, true, true, false, true, true, true, false, true, true, false, true, false, true, true, true, false, false, true, true, true, false, true, false, true, false],
    [false, true, false, true, false, true, true, true, true, false, true, false, false, true, false, false, false, false, false, false, true, true, false, true, false, false, true, true, true],
    [false, true, true, false, true, true, false, false, true, false, false, false, true, false, true, false, true, false, false, true, true, true, true, false, true, true, false, true, true],
    [true, true, false, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, true, false, true, false, true, true, false, false, false, false, false],
    [true, false, true, true, true, false, false, false, true, true, false, false, true, false, false, false, false, true, true, false, true, false, false, false, true, true, false, true, false],
    [false, true, true, false, true, true, true, false, false, true, true, false, true, false, false, true, true, false, false, false, false, true, false, false, false, true, true, true, false],
    [true, false, true, true, true, false, false, false, true, false, true, false, true, true, false, true, true, false, false, false, true, true, true, true, true, true, false, true, true],
    [true, false, false, true, true, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, false, false, true, true, false, true, false, false, false],
    [true, false, true, false, false, true, false, true, false, true, true, true, true, false, false, true, true, true, false, true, true, false, true, false, true, false, false, true, false],
    [true, false, true, true, false, true, true, false, true, false, true, true, true, true, true, true, true, false, true, true, true, true, true, true, true, false, true, false, false],
    [false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, true, true, true, false, true, false, false, false, true, true, false, false, true],
    [true, true, true, true, true, true, true, false, false, false, true, false, true, true, true, true, false, true, true, true, true, false, true, false, true, false, false, false, false],
    [true, false, false, false, false, false, true, false, false, true, true, true, true, true, false, true, true, false, true, true, true, false, false, false, true, true, false, false, false],
    [true, false, true, true, true, false, true, false, true, true, false, false, true, false, false, false, true, true, true, false, true, true, true, true, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, true, false, true, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, false],
    [true, false, true, true, true, false, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, true, true, false, true, true, false],
    [true, false, false, false, false, false, true, false, false, true, false, false, false, false, true, true, true, true, false, false, false, false, true, false, true, true, false, true, false],
    [true, true, true, true, true, true, true, false, false, false, true, true, true, true, true, true, false, false, true, false, false, false, true, false, true, false, true, false, false]
];

#[test]
fn example_com() {
    let mat = create_mat_from_bool(&MAT_EXAMPLE_COM);
    let (line_score, col_score, pattern_score, dark_score) = test_matrix_pattern_and_line(&mat);
    let square_score = test_matrix_score_squares(&mat);

    // {"patternScore":160,"lineColScore":106,"squareScore":135,"darkScore":0}
    // {"patternScore":240,"lineColScore":117,"squareScore":138,"darkScore":0}
    assert_eq!(dark_score, 0, "dark score, expected 0");
    assert_eq!(square_score, 138, "square score, expected 138");
    assert_eq!(line_score + col_score, 117, "line col score, expected 117");
    assert_eq!(pattern_score, 240, "pattern score, expected 240");
}

#[rustfmt::skip]
const MAT_FAST_QR_COM: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, true, true, true, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, false, true, false, true, true, false, false, false, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, true, true, true, true, false, true, false, false, true, false, true, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, false, false, true, false, true, false, false, true, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, false, false, true, true, false, true, true, true, true, false, true, false, true, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, true, true, false, false, false, true, true, true, false, true, false, false, false, true, false, true, false, true, false, true, false, true, false, true],
    [true, true, false, true, true, true, false, true, true, true, true, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, true, true, false],
    [false, true, false, false, true, false, true, false, true, false, true, false, true, true, false, false, false, true, true, false, true, true, true, true, false, true, false, false, false],
    [false, false, false, true, true, false, false, false, false, false, true, true, true, false, false, true, true, true, false, false, false, false, true, true, false, true, false, false, false],
    [true, true, false, true, true, true, true, false, true, false, true, true, false, false, false, true, true, false, true, true, true, true, true, false, false, false, false, true, false],
    [true, false, true, true, false, false, false, false, false, true, false, false, false, true, false, false, true, true, true, true, true, false, true, false, true, true, false, true, true],
    [true, true, false, false, false, false, true, true, true, false, false, false, false, false, false, true, false, true, false, false, false, false, true, true, false, false, false, false, false],
    [false, false, true, false, true, false, false, false, false, false, true, true, false, false, false, true, true, false, true, true, false, true, true, false, true, true, true, true, true],
    [false, false, true, false, true, false, true, true, false, false, true, false, true, false, true, true, false, false, true, false, false, true, false, false, false, true, true, false, true],
    [true, true, true, true, false, false, false, true, false, false, true, false, false, false, true, false, true, true, false, false, true, false, true, true, true, false, false, false, true],
    [true, true, true, true, true, true, true, true, true, false, false, false, false, true, true, true, false, false, true, false, true, false, false, false, false, true, false, false, true],
    [true, false, false, false, false, true, false, false, true, false, true, true, false, true, true, false, false, false, true, true, true, true, true, false, true, false, false, false, false],
    [true, false, false, true, false, false, true, false, false, false, true, true, false, true, false, false, true, true, false, true, true, true, true, true, true, false, true, false, false],
    [false, false, false, false, false, false, false, false, true, false, true, false, false, true, true, false, true, true, true, false, true, false, false, false, true, true, true, false, false],
    [true, true, true, true, true, true, true, false, false, false, false, true, true, false, false, false, true, true, true, true, true, false, true, false, true, false, false, true, false],
    [true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, false, true, true, true, false, false, false, true, true, false, false, false],
    [true, false, true, true, true, false, true, false, false, true, true, false, true, false, false, true, true, false, true, true, true, true, true, true, true, false, false, false, true],
    [true, false, true, true, true, false, true, false, false, true, true, false, false, false, true, false, true, false, true, true, true, false, false, true, false, true, true, true, false],
    [true, false, true, true, true, false, true, false, false, true, true, false, false, false, false, true, true, false, true, true, true, true, true, true, true, false, true, true, false],
    [true, false, false, false, false, false, true, false, false, true, false, true, true, true, false, true, true, true, false, false, true, true, false, false, true, true, true, false, true],
    [true, true, true, true, true, true, true, false, false, false, true, true, false, false, true, true, true, true, true, false, false, false, true, false, true, false, true, false, false]
];

#[test]
fn fast_qr_com() {
    let mat = create_mat_from_bool(&MAT_FAST_QR_COM);
    let (line_score, col_score, pattern_score, dark_score) = test_matrix_pattern_and_line(&mat);
    let square_score = test_matrix_score_squares(&mat);

    // {"patternScore":80,"lineColScore":108,"squareScore":174,"darkScore":0}
    assert_eq!(dark_score, 0, "dark score, expected 0");
    assert_eq!(square_score, 174, "square score, expected 174");
    assert_eq!(line_score + col_score, 108, "line col score, expected 108");
    assert_eq!(pattern_score, 80, "pattern score, expected 80");
}

#[rustfmt::skip]
const MAT_XIAOJIBA_DEV: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, false, true, false, true, false, true, true, true, true, true, false, true, true, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, false, true, true, false, true, true, true, false, true, false, true, true, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, true, true, false, true, false, true, true, true, false, false, true, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, true, true, true, false, true, false, false, false, true, false, false, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, true, false, true, true, true, false, false, true, false, true, true, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, false, true, false, false, false, false, true, true, false, false, false, false, true, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true, true, true, false, false, false, false, false, false, false, false],
    [false, false, false, true, true, false, true, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false],
    [true, true, false, true, true, false, false, true, true, false, true, true, false, true, true, true, false, false, true, false, false, false, false, true, true, false, false, true, false],
    [true, true, true, false, false, true, true, true, true, false, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true, true, false, false],
    [true, true, true, false, false, true, false, true, false, true, false, false, true, true, false, false, false, true, false, true, true, false, false, false, true, true, false, false, true],
    [false, true, true, false, false, true, true, true, true, true, true, true, true, true, false, true, true, true, true, false, true, true, true, false, false, true, false, true, false],
    [true, false, false, false, false, true, false, true, false, false, false, true, false, true, true, false, false, true, true, false, false, false, true, false, true, false, true, false, true],
    [false, true, false, false, true, true, true, true, true, true, false, true, true, false, true, true, false, true, false, true, false, false, true, true, false, true, false, false, true],
    [true, true, false, true, false, true, false, false, true, true, true, false, false, false, false, false, false, true, true, false, true, true, true, false, true, true, true, false, false],
    [false, true, false, false, false, true, true, true, false, true, true, true, true, true, true, true, true, false, true, true, true, false, false, true, false, true, false, true, true],
    [true, true, true, false, true, false, false, false, true, true, false, false, true, true, false, true, true, false, false, false, false, true, false, false, true, false, false, false, false],
    [true, true, true, false, true, true, true, false, true, false, false, true, false, true, false, false, true, true, true, true, false, false, false, true, false, true, false, false, true],
    [true, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, true, true, true, false, false, true, true, false],
    [true, true, false, true, true, false, true, true, true, false, false, false, false, true, false, true, false, true, false, false, true, true, true, true, true, true, true, false, true],
    [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, true, true, false, false, false, true, true, true, false, false],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, true, false, false, false, false, false, true, true, false, true, false, true, false, true, false, false],
    [true, false, false, false, false, false, true, false, false, true, true, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false],
    [true, false, true, true, true, false, true, false, true, true, true, true, false, true, false, false, false, true, false, true, true, true, true, true, true, false, false, false, false],
    [true, false, true, true, true, false, true, false, true, false, true, false, true, true, false, true, true, true, true, false, false, false, false, true, false, true, true, true, false],
    [true, false, true, true, true, false, true, false, false, true, true, true, false, true, false, true, true, true, false, false, true, true, false, true, true, false, false, true, true],
    [true, false, false, false, false, false, true, false, false, true, false, true, true, false, false, true, true, true, true, false, false, true, true, false, true, false, true, false, true],
    [true, true, true, true, true, true, true, false, false, false, true, false, true, false, true, true, false, true, true, false, true, true, true, true, true, false, false, false, false]
];

#[test]
fn xiaojiba_dev() {
    let mat = create_mat_from_bool(&MAT_XIAOJIBA_DEV);
    let (line_score, col_score, pattern_score, dark_score) = test_matrix_pattern_and_line(&mat);
    let square_score = test_matrix_score_squares(&mat);

    // {"patternScore":160,"lineColScore":95,"squareScore":150,"darkScore":0}
    assert_eq!(dark_score, 0, "dark score, expected 0");
    assert_eq!(square_score, 150, "square score, expected 150");
    assert_eq!(line_score + col_score, 95, "line col score, expected 95");
    assert_eq!(pattern_score, 160, "pattern score, expected 160");
}

#[test]
fn adjacent_line() {
    // Data module true, false, true, true, true, false, true
    let line = [
        Module::data(true),
        Module::data(false),
        Module::data(true),
        Module::data(true),
        Module::data(true),
        Module::data(false),
        Module::data(true),
    ];

    // Data module true true true true true true true
    let line_2 = [
        Module::data(true),
        Module::data(true),
        Module::data(true),
        Module::data(true),
        Module::data(true),
        Module::data(true),
        Module::data(true),
    ];

    // Data module true true true true true false true true true true true
    let line_3_not_data = [
        Module::empty(true),
        Module::data(true),
        Module::data(true),
        Module::data(true),
        Module::data(true),
        Module::empty(false),
        Module::data(true),
        Module::data(true),
        Module::data(true),
        Module::data(true),
        Module::data(true),
    ];

    assert_eq!(test_score_line(&line), 0, "line score, expected 0");
    assert_eq!(test_score_line(&line_2), 5, "line score, expected 5");
    assert_eq!(
        test_score_line(&line_3_not_data),
        3,
        "line score, expected 3"
    );
}

#[test]
fn line_by_line_xiaojiba() {
    #[rustfmt::skip]
    const XIAOJIBA_MOD: [[Module; 29]; 29] = [
        [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(8), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(14), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3)],
        [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(14), Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3)],
        [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(9), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(14), Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3)],
        [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(9), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(1), Module(14), Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3)],
        [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(8), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(14), Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3)],
        [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(14), Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3)],
        [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(14), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3)],
        [Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(8), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14)],
        [Module(8), Module(8), Module(8), Module(9), Module(9), Module(8), Module(7), Module(9), Module(8), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0), Module(8), Module(8), Module(8), Module(8), Module(9), Module(9), Module(8), Module(8)],
        [Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(6), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0)],
        [Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(7), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0)],
        [Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(6), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1)],
        [Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(7), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0)],
        [Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(6), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1)],
        [Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(7), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1)],
        [Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(6), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0)],
        [Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(7), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1)],
        [Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(6), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0)],
        [Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(7), Module(0), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1)],
        [Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(6), Module(0), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0)],
        [Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(7), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(5), Module(5), Module(5), Module(5), Module(5), Module(1), Module(1), Module(0), Module(1)],
        [Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(13), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(1), Module(5), Module(4), Module(4), Module(4), Module(5), Module(1), Module(1), Module(0), Module(0)],
        [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(9), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(5), Module(4), Module(5), Module(4), Module(5), Module(0), Module(1), Module(0), Module(0)],
        [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(5), Module(4), Module(4), Module(4), Module(5), Module(1), Module(0), Module(1), Module(0)],
        [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(9), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(5), Module(5), Module(5), Module(5), Module(5), Module(0), Module(0), Module(0), Module(0)],
        [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(9), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0)],
        [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(8), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1)],
        [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1)],
        [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(8), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0)]
    ];

    let scores = [
        3,
        0,
        0,
        0,
        0,
        0,
        0,
        6,
        0,
        0,
        6,
        0,
        5,
        0,
        0,
        4,
        6,
        0,
        0,
        3 + 6,
        0,
        0,
        3, // 22
        0,
        0,
        0,
        0,
        0,
        3,
    ];

    for (i, line) in XIAOJIBA_MOD.iter().enumerate() {
        let score = scores[i];
        assert_eq!(test_score_line(line), score, "line {i}, expected {score}",);
    }
}
