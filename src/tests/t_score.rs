const MAT_HELLO_WORLD: [[bool; 21]; 21] = [
    [
        true, true, true, true, true, true, true, false, true, true, false, false, false, false,
        true, true, true, true, true, true, true,
    ],
    [
        true, false, false, false, false, false, true, false, true, false, false, true, false,
        false, true, false, false, false, false, false, true,
    ],
    [
        true, false, true, true, true, false, true, false, true, false, false, true, true, false,
        true, false, true, true, true, false, true,
    ],
    [
        true, false, true, true, true, false, true, false, true, false, false, false, false, false,
        true, false, true, true, true, false, true,
    ],
    [
        true, false, true, true, true, false, true, false, true, false, true, false, false, false,
        true, false, true, true, true, false, true,
    ],
    [
        true, false, false, false, false, false, true, false, false, false, true, false, false,
        false, true, false, false, false, false, false, true,
    ],
    [
        true, true, true, true, true, true, true, false, true, false, true, false, true, false,
        true, true, true, true, true, true, true,
    ],
    [
        false, false, false, false, false, false, false, false, true, false, false, false, false,
        false, false, false, false, false, false, false, false,
    ],
    [
        false, true, false, true, true, false, true, true, false, false, false, false, true, false,
        true, false, true, true, true, true, true,
    ],
    [
        false, true, false, false, false, false, false, false, true, true, true, true, false,
        false, false, false, true, false, false, false, true,
    ],
    [
        false, false, true, true, false, true, true, true, false, true, true, false, false, false,
        true, false, true, true, false, false, false,
    ],
    [
        false, true, true, false, true, true, false, true, false, false, true, true, false, true,
        false, true, false, true, true, true, false,
    ],
    [
        true, false, false, false, true, false, true, false, true, false, true, true, true, false,
        true, true, true, false, true, false, true,
    ],
    [
        false, false, false, false, false, false, false, false, true, true, false, true, false,
        false, true, false, false, false, true, false, true,
    ],
    [
        true, true, true, true, true, true, true, true, false, false, true, false, false, false,
        false, true, false, true, true, false, false,
    ],
    [
        true, false, false, false, false, false, true, false, false, true, false, true, true,
        false, true, true, false, true, false, false, false,
    ],
    [
        true, false, true, true, true, false, true, false, true, false, true, false, false, false,
        true, true, true, true, true, true, true,
    ],
    [
        true, false, true, true, true, false, true, false, false, true, false, true, false, true,
        false, true, false, false, false, true, false,
    ],
    [
        true, false, true, true, true, false, true, false, true, false, false, true, false, true,
        true, true, false, true, false, false, true,
    ],
    [
        true, false, false, false, false, false, true, false, true, false, true, true, true, true,
        false, false, false, true, false, true, true,
    ],
    [
        true, true, true, true, true, true, true, false, false, false, false, true, false, true,
        true, true, false, false, false, false, true,
    ],
];

#[test]
fn score_rows_hello() {
    let mut mat = Vec::new();
    for e in MAT_HELLO_WORLD {
        mat.push(Vec::from(e));
    }

    assert_eq!(
        crate::score::matrix_score_rows_test(&mat),
        10 + 6 + 0 + 3 + 0 + 6 + 10 + 16 + 3 + 4 + 0 + 0 + 0 + 6 + 6 + 3 + 5 + 0 + 0 + 3 + 5
    );
}

#[test]
fn score_lines_hello() {
    let mut mat = Vec::new();
    for e in MAT_HELLO_WORLD {
        mat.push(Vec::from(e));
    }

    assert_eq!(
        crate::score::matrix_score_lines_test(&mat),
        13 + 6 + 0 + 0 + 0 + 6 + 10 + 10 + 3 + 6 + 0 + 4 + 0 + 12 + 5 + 6 + 0 + 0 + 0 + 3 + 5
    );
}

#[test]
fn score_squares_hello() {
    let mut mat = Vec::new();
    for e in MAT_HELLO_WORLD {
        mat.push(Vec::from(e));
    }

    assert_eq!(crate::score::matrix_score_squares_test(&mat), 90);
}

#[test]
fn score_pattern_hello() {
    let mut mat = Vec::new();
    for e in MAT_HELLO_WORLD {
        mat.push(Vec::from(e));
    }

    assert_eq!(crate::score::matrix_score_pattern_test(&mat), 80);
}

#[test]
fn score_modules_hello() {
    let mut mat = Vec::new();
    for e in MAT_HELLO_WORLD {
        mat.push(Vec::from(e));
    }

    assert_eq!(crate::score::matrix_score_modules_test(&mat), 0);
}

#[test]
fn score_total_hello() {
    let mut mat = Vec::new();
    for e in MAT_HELLO_WORLD {
        mat.push(Vec::from(e));
    }

    assert_eq!(
        crate::score::matrix_score(&mat),
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

const MAT_SALUT: [[bool; 21]; 21] = [
    [
        true, true, true, true, true, true, true, false, true, false, true, false, true, false,
        true, true, true, true, true, true, true,
    ],
    [
        true, false, false, false, false, false, true, false, true, true, false, true, false,
        false, true, false, false, false, false, false, true,
    ],
    [
        true, false, true, true, true, false, true, false, false, false, true, false, false, false,
        true, false, true, true, true, false, true,
    ],
    [
        true, false, true, true, true, false, true, false, false, false, true, true, false, false,
        true, false, true, true, true, false, true,
    ],
    [
        true, false, true, true, true, false, true, false, false, true, false, true, true, false,
        true, false, true, true, true, false, true,
    ],
    [
        true, false, false, false, false, false, true, false, false, true, false, false, true,
        false, true, false, false, false, false, false, true,
    ],
    [
        true, true, true, true, true, true, true, false, true, false, true, false, true, false,
        true, true, true, true, true, true, true,
    ],
    [
        false, false, false, false, false, false, false, false, true, false, false, false, false,
        false, false, false, false, false, false, false, false,
    ],
    [
        false, false, false, false, true, false, true, false, true, true, false, true, false, true,
        true, false, false, false, false, true, true,
    ],
    [
        true, true, true, true, false, true, false, true, true, true, false, false, false, true,
        false, true, true, false, true, false, false,
    ],
    [
        false, true, false, true, true, false, true, false, true, true, false, false, true, false,
        true, true, false, true, true, true, false,
    ],
    [
        false, false, true, false, true, true, false, true, false, true, false, false, false, true,
        true, true, false, true, true, true, true,
    ],
    [
        true, true, true, false, false, true, true, true, false, false, false, true, true, true,
        true, true, true, true, false, true, true,
    ],
    [
        false, false, false, false, false, false, false, false, true, false, false, true, false,
        false, false, false, false, true, false, true, true,
    ],
    [
        true, true, true, true, true, true, true, false, false, true, false, true, false, true,
        false, false, true, false, true, true, false,
    ],
    [
        true, false, false, false, false, false, true, false, false, false, false, false, false,
        false, true, false, false, true, true, true, true,
    ],
    [
        true, false, true, true, true, false, true, false, false, true, false, false, false, true,
        false, false, true, true, false, true, false,
    ],
    [
        true, false, true, true, true, false, true, false, false, true, false, true, true, true,
        true, false, false, true, true, false, false,
    ],
    [
        true, false, true, true, true, false, true, false, false, false, false, true, true, true,
        false, true, true, false, true, true, true,
    ],
    [
        true, false, false, false, false, false, true, false, true, false, true, true, true, true,
        true, false, false, true, true, false, false,
    ],
    [
        true, true, true, true, true, true, true, false, false, true, true, false, true, false,
        false, false, false, true, false, true, false,
    ],
];

#[test]
fn score_rows_salut() {
    let mut mat = Vec::new();
    for e in MAT_SALUT {
        mat.push(Vec::from(e));
    }

    assert_eq!(
        crate::score::matrix_score_rows_test(&mat),
        10 + 6 + 0 + 0 + 0 + 6 + 10 + 16 + 0 + 0 + 0 + 0 + 5 + 6 + 3 + 5 + 8 + 0 + 0 + 0 + 6 + 5
    );
}

#[test]
fn score_lines_salut() {
    let mut mat = Vec::new();
    for e in MAT_SALUT {
        mat.push(Vec::from(e));
    }

    assert_eq!(
        crate::score::matrix_score_lines_test(&mat),
        10 + 6 + 0 + 0 + 0 + 6 + 10 + 13 + 6 + 0 + 10 + 0 + 0 + 6 + 5 + 6 + 0 + 0 + 0 + 8 + 5
    );
}

const MAT_PATTERN_14: [[bool; 12]; 11] = [
    [
        true, false, true, true, true, false, true, false, false, false, false, true,
    ],
    [
        false, false, false, false, true, false, true, true, true, false, true, true,
    ],
    [
        true, false, true, true, true, false, true, false, false, false, false, true,
    ],
    [
        true, false, true, true, true, false, true, false, false, false, false, true,
    ],
    [
        true, false, true, true, true, false, true, false, false, false, false, true,
    ],
    [
        false, false, false, false, true, false, true, true, true, false, true, true,
    ],
    [
        true, false, true, true, true, false, true, false, false, false, false, true,
    ],
    [
        false, false, false, false, true, false, true, true, true, false, true, true,
    ],
    [
        false, false, false, false, true, false, true, true, true, false, true, true,
    ],
    [
        false, false, false, false, true, false, true, true, true, false, true, true,
    ],
    [
        false, false, false, false, true, false, true, true, true, false, true, true,
    ],
];

#[test]
fn score_pattern_salut_14() {
    let mut mat = Vec::new();
    for e in MAT_PATTERN_14 {
        mat.push(Vec::from(e));
    }

    assert_eq!(crate::score::matrix_score_pattern_test(&mat), 40 * (11 + 3));
}
