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
fn score_rows() {
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
fn score_lines() {
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
fn score_squares() {
    let mut mat = Vec::new();
    for e in MAT_HELLO_WORLD {
        mat.push(Vec::from(e));
    }

    assert_eq!(crate::score::matrix_score_squares_test(&mat), 90);
}

#[test]
fn score_pattern() {
    let mut mat = Vec::new();
    for e in MAT_HELLO_WORLD {
        mat.push(Vec::from(e));
    }

    assert_eq!(crate::score::matrix_score_pattern_test(&mat), 80);
}

#[test]
fn score_modules() {
    let mut mat = Vec::new();
    for e in MAT_HELLO_WORLD {
        mat.push(Vec::from(e));
    }

    assert_eq!(crate::score::matrix_score_modules_test(&mat), 0);
}

#[test]
fn score_total() {
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
