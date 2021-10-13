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

#[test]
fn score_rows_hello() {
    assert_eq!(
        crate::score::matrix_score_rows_test(&MAT_HELLO_WORLD),
        10 + 6 + 0 + 3 + 0 + 6 + 10 + 16 + 3 + 4 + 0 + 0 + 0 + 6 + 6 + 3 + 5 + 0 + 0 + 3 + 5
    );
}

#[test]
fn score_lines_hello() {
    assert_eq!(
        crate::score::matrix_score_lines_test(&MAT_HELLO_WORLD),
        13 + 6 + 0 + 0 + 0 + 6 + 10 + 10 + 3 + 6 + 0 + 4 + 0 + 12 + 5 + 6 + 0 + 0 + 0 + 3 + 5
    );
}

#[test]
fn score_squares_hello() {
    assert_eq!(
        crate::score::matrix_score_squares_test(&MAT_HELLO_WORLD),
        90
    );
}

#[test]
fn score_pattern_hello() {
    assert_eq!(
        crate::score::matrix_score_pattern_test(&MAT_HELLO_WORLD),
        80
    );
}

#[test]
fn score_modules_hello() {
    assert_eq!(crate::score::matrix_score_modules_test(&MAT_HELLO_WORLD), 0);
}

#[test]
fn score_total_hello() {
    assert_eq!(
        crate::score::matrix_score(&MAT_HELLO_WORLD),
        0 + 80
            + 90
            + 13
            + 6
            + 0
            + 0
            + 0
            + 6
            + 10
            + 10
            + 3
            + 6
            + 0
            + 4
            + 0
            + 12
            + 5
            + 6
            + 0
            + 0
            + 0
            + 3
            + 5
            + 10
            + 6
            + 0
            + 3
            + 0
            + 6
            + 10
            + 16
            + 3
            + 4
            + 0
            + 0
            + 0
            + 6
            + 6
            + 3
            + 5
            + 0
            + 0
            + 3
            + 5
    );
}

#[rustfmt::skip]
const MAT_SALUT: [[bool; 21]; 21] = [[true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true,],
    [true, false, false, false, false, false, true, false, true, true, false, true, false, false, true, false, false, false, false, false, true,],
    [true, false, true, true, true, false, true, false, false, false, true, false, false, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, false, false, true, true, false, false, true, false, true, true, true, false, true,],
    [true, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, true, true, true, false, true,],
    [true, false, false, false, false, false, true, false, false, true, false, false, true, false, true, false, false, false, false, false, true,],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true,],
    [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false,],
    [false, false, false, false, true, false, true, false, true, true, false, true, false, true, true, false, false, false, false, true, true,],
    [true, true, true, true, false, true, false, true, true, true, false, false, false, true, false, true, true, false, true, false, false,],
    [false, true, false, true, true, false, true, false, true, true, false, false, true, false, true, true, false, true, true, true, false,],
    [false, false, true, false, true, true, false, true, false, true, false, false, false, true, true, true, false, true, true, true, true,],
    [true, true, true, false, false, true, true, true, false, false, false, true, true, true, true, true, true, true, false, true, true,],
    [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, true, true,],
    [true, true, true, true, true, true, true, false, false, true, false, true, false, true, false, false, true, false, true, true, false,],
    [true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, true, true, true,],
    [true, false, true, true, true, false, true, false, false, true, false, false, false, true, false, false, true, true, false, true, false,],
    [true, false, true, true, true, false, true, false, false, true, false, true, true, true, true, false, false, true, true, false, false,],
    [true, false, true, true, true, false, true, false, false, false, false, true, true, true, false, true, true, false, true, true, true,],
    [true, false, false, false, false, false, true, false, true, false, true, true, true, true, true, false, false, true, true, false, false,],
    [true, true, true, true, true, true, true, false, false, true, true, false, true, false, false, false, false, true, false, true, false,],
];

#[test]
fn score_rows_salut() {
    assert_eq!(
        crate::score::matrix_score_rows_test(&MAT_SALUT),
        10 + 6 + 0 + 0 + 0 + 6 + 10 + 16 + 0 + 0 + 0 + 0 + 5 + 6 + 3 + 5 + 8 + 0 + 0 + 0 + 6 + 5
    );
}

#[test]
fn score_lines_salut() {
    assert_eq!(
        crate::score::matrix_score_lines_test(&MAT_SALUT),
        10 + 6 + 0 + 0 + 0 + 6 + 10 + 13 + 6 + 0 + 10 + 0 + 0 + 6 + 5 + 6 + 0 + 0 + 0 + 8 + 5
    );
}

#[rustfmt::skip]
const MAT_PATTERN_14: [[bool; 11]; 11] = [
    [true, false, true, true, true, false, true, false, false, false, false],
    [false, false, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, false],
    [true, false, true, true, true, false, true, false, false, false, false],
    [true, false, true, true, true, false, true, false, false, false, false],
    [false, false, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, false],
    [false, false, false, false, true, false, true, true, true, false, true],
    [false, false, false, false, true, false, true, true, true, false, true],
    [false, false, false, false, true, false, true, true, true, false, true],
    [false, false, false, false, true, false, true, true, true, false, true],
];

#[test]
fn score_pattern_salut_14() {
    assert_eq!(
        crate::score::matrix_score_pattern_test(&MAT_PATTERN_14),
        40 * (11 + 3)
    );
}
